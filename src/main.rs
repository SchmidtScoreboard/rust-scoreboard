#![feature(proc_macro_hygiene, decl_macro, array_map)]

mod animation;
mod aws_screen;
mod baseball;
mod button;
mod clock;
mod common;
mod game;
mod hockey;
mod matrix;
mod message;
mod scoreboard_settings;
mod setup_screen;
mod shell_executor;
mod webserver;
#[macro_use]
extern crate rust_embed;

#[macro_use]
extern crate log;

use animation::AnimationTestScreen;
use aws_screen::AWSScreen;
use baseball::BaseballGame;
use clap;
use common::ScreenId;
use hockey::HockeyGame;
use matrix::{Matrix, ScreenProvider};
use rpi_led_matrix;
use self_update;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("Schmidt Scoreboard")
        .version(self_update::cargo_crate_version!())
        .author("Mark Schmidt <mark.schmidt@hey.com>")
        .about("Runs a Scoreboard to display hockey and baseball scores")
        .arg(clap::Arg::with_name("root_path")
            .short("d")
            .long("root_path")
            .value_name("root_path")
            .help("Specify a directory with a scoreboard_settings.json file and secrets.txt file with a valid API key")
            .takes_value(true))
        .arg(clap::Arg::with_name("skip_update")
            .short("u")
            .long("skip_update")
            .value_name("skip_update")
            .help("Specify this flag to skip the update process")
            .takes_value(false))
        .get_matches();

    let root_path = PathBuf::from(
        matches
            .value_of("root_path")
            .unwrap_or("/var/lib/scoreboard/"),
    );
    let log_dir = root_path.join("logs");
    let _create_dir_result = fs::create_dir(&log_dir);

    flexi_logger::Logger::with_env()
        .log_to_file()
        .directory(log_dir)
        .duplicate_to_stdout(flexi_logger::Duplicate::All)
        .format_for_stdout(flexi_logger::opt_format)
        .format_for_files(flexi_logger::detailed_format)
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(3),
        )
        .start()
        .unwrap();

    let skip_update = matches.is_present("skip_update");
    if !skip_update {
        match common::update() {
            Ok(_) => {}
            Err(e) => error!("Error while updating: {:?}", e), // Ignore the failure
        }
    } else {
        info!("Skipping update");
    }

    let secrets_path = root_path.join("secrets.txt");
    let settings_path = root_path.join("scoreboard_settings.json");
    info!("Loading secrets from {:?}", secrets_path);
    info!("Loading settings from {:?}", settings_path);

    let api_key = fs::read_to_string(&secrets_path).expect(&format!(
        "Could not read from secrets.txt at path {:?}",
        &secrets_path
    ));

    let settings_data: common::ScoreboardSettingsData =
        serde_json::from_str(&fs::read_to_string(&settings_path).expect(&format!(
            "Could not read scoreboard settings at path {:?}",
            &settings_path
        )))
        .expect("Could not parse scoreboard settings from json");

    let (tx, rx) = mpsc::channel();
    let (web_response_sender, web_response_receiver) = mpsc::channel();
    let (shell_sender, shell_receiver) = mpsc::channel();

    let mut settings = scoreboard_settings::ScoreboardSettings::new(settings_data, settings_path);

    if settings.get_settings().setup_state == common::SetupState::Factory {
        settings.set_setup_state(&common::SetupState::Hotspot);
    }

    let shell = shell_executor::CommandExecutor::new(
        web_response_sender.clone(),
        tx.clone(),
        shell_receiver,
    );
    std::thread::spawn(move || {
        shell.run();
    });

    let enable_hotspot = match settings.get_settings().setup_state {
        common::SetupState::Hotspot | common::SetupState::WifiConnect => true,
        _ => false,
    };
    if enable_hotspot || !common::is_connected() {
        info!("Could not reach internet or in setup state, showing hotspot screen and enabling the hotspot");
        settings.set_setup_state(&common::SetupState::Hotspot);
        settings.set_active_screen(&common::ScreenId::Setup);
        shell_sender
            .send(common::ShellCommand::Reset {
                from_matrix: false,
                from_webserver: None,
            })
            .unwrap();
    } else {
        shell_sender
            .send(common::ShellCommand::SetHotspot(false))
            .unwrap();
    }

    let mut button_handler = button::ButtonHandler::new(tx.clone());

    // Setup ScreenProvider map
    let mut map: HashMap<ScreenId, Box<dyn ScreenProvider>> = HashMap::new();

    // Hockey
    let hockey: AWSScreen<HockeyGame> = AWSScreen::new(
        tx.clone(),
        api_key.clone(),
        settings.get_settings().timezone.clone(),
        matrix::FontBook::new(&root_path),
        matrix::PixelBook::new(&root_path),
    );
    map.insert(ScreenId::Hockey, Box::new(hockey));

    // Baseball
    let baseball: AWSScreen<BaseballGame> = AWSScreen::new(
        tx.clone(),
        api_key.clone(),
        settings.get_settings().timezone.clone(),
        matrix::FontBook::new(&root_path),
        matrix::PixelBook::new(&root_path),
    );
    map.insert(ScreenId::Baseball, Box::new(baseball));

    // Clock
    let clock = clock::Clock::new(
        tx.clone(),
        settings.get_settings().timezone.clone(),
        matrix::FontBook::new(&root_path),
    );
    map.insert(ScreenId::Clock, Box::new(clock));

    // Setup Screen
    let setup_screen = setup_screen::SetupScreen::new(
        tx.clone(),
        settings.get_settings().setup_state,
        matrix::FontBook::new(&root_path),
        matrix::PixelBook::new(&root_path),
    );
    map.insert(ScreenId::Setup, Box::new(setup_screen));

    // Animation Test
    let animation = AnimationTestScreen::new(tx.clone());
    map.insert(ScreenId::Animation, Box::new(animation));

    // Message Screen
    let message_screen = message::MessageScreen::new(tx.clone(), matrix::FontBook::new(&root_path));
    // Setup the actual matrix and run it
    // Setup matrix options
    let mut options = rpi_led_matrix::LedMatrixOptions::new();
    let rt_options = rpi_led_matrix::LedRuntimeOptions::new();
    options.set_rows(32);
    options.set_cols(64);
    options.set_hardware_mapping("adafruit-hat-pwm");
    options.set_pwm_lsb_nanoseconds(50);
    options.set_refresh_rate(false);
    let led_matrix: rpi_led_matrix::LedMatrix =
        rpi_led_matrix::LedMatrix::new(Some(options), Some(rt_options))
            .expect("Could not setup matrix");

    let mut matrix = Matrix::new(
        led_matrix,
        message_screen,
        rx,
        map,
        settings,
        web_response_sender,
        shell_sender.clone(),
    );
    let webserver_sender = tx.clone();
    std::thread::spawn(move || {
        webserver::run_webserver(webserver_sender, web_response_receiver, root_path);
    });
    std::thread::spawn(move || {
        button_handler.run();
    });
    info!("Starting matrix runner");
    matrix.run();
    Ok(())
}
