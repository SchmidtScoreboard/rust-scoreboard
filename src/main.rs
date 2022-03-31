#![feature(proc_macro_hygiene, decl_macro)]

mod animation;
mod aws_screen;
mod baseball;
mod basketball;
mod button;
mod custom_message;
mod clock;
mod common;
mod flappy;
mod football;
mod game;
mod golf;
mod hockey;
mod matrix;
mod message;
mod patch_notes;
mod scheduler;
mod scoreboard_settings;
mod setup_screen;
mod shell_executor;
mod sport;
mod updater;
mod webserver;
#[macro_use]
extern crate rust_embed;

#[macro_use]
extern crate log;

use animation::AnimationTestScreen;
use common::ScreenId;
use matrix::{Matrix, ScreenProvider};
use sport::AWSScreen;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::{mpsc, Arc};
use std::thread::sleep;
use std::time::Duration;
use updater::Updater;
const V2_URL: &str = "https://uhoijpn7d1.execute-api.us-east-2.amazonaws.com/Prod/";

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
        .arg(clap::Arg::with_name("wait")
            .short("w")
            .long("wait")
            .value_name("wait")
            .help("Wait for 90 seconds before doing anything important, useful to give DHCP time to unfuck itself")
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
        .duplicate_to_stdout(flexi_logger::Duplicate::Info)
        .format_for_stdout(flexi_logger::opt_format)
        .format_for_files(flexi_logger::detailed_format)
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(3),
        )
        .start()
        .unwrap();

    if matches.is_present("wait") {
        info!("Waiting 90 seconds");
        sleep(Duration::from_secs(90));
    } else {
        info!("Starting up now");
    }

    let skip_update = matches.is_present("skip_update");
    patch_notes::log_patch_notes();

    let mut updater: Updater;
    if !skip_update {
        // Start the update service
        updater = Updater::new();
        std::thread::spawn(move || {
            updater.run();
        });
    } else {
        info!("Skipping updater service");
    }

    let secrets_path = root_path.join("secrets.txt");
    let settings_path = root_path.join("scoreboard_settings.json");
    info!("Loading secrets from {:?}", secrets_path);
    info!("Loading settings from {:?}", settings_path);

    let api_key = fs::read_to_string(&secrets_path).unwrap_or_else(|_| panic!(
        "Could not read from secrets.txt at path {:?}",
        &secrets_path
    ));
    let settings_string = fs::read_to_string(&settings_path).unwrap_or_else(|_| panic!(
            "Could not read scoreboard settings at path {:?}",
            &settings_path
    ));
    let settings_data: common::ScoreboardSettingsData =
        serde_json::from_str(&settings_string).expect("Could not parse scoreboard settings from json");
    let settings_data = Arc::from(settings_data);

    let (matrix_sender, matrix_receiver) = mpsc::channel();
    let (scheduler_sender, scheduler_receiver) = mpsc::channel();
    let (web_response_sender, web_response_receiver) = mpsc::channel();
    let (shell_sender, shell_receiver) = mpsc::channel();

    let mut settings = scoreboard_settings::ScoreboardSettings::new(settings_data, settings_path);
    settings.set_version(7);

    if settings.get_settings().setup_state == common::SetupState::Factory {
        settings.set_setup_state(&common::SetupState::Hotspot);
    }

    let shell = shell_executor::CommandExecutor::new(
        web_response_sender.clone(),
        matrix_sender.clone(),
        shell_receiver,
    );
    std::thread::spawn(move || {
        shell.run();
    });
    let mut button_handler = button::ButtonHandler::new(matrix_sender.clone());
    std::thread::spawn(move || {
        button_handler.run();
    });

    let enable_hotspot = matches!(settings.get_settings().setup_state,
        common::SetupState::Hotspot | common::SetupState::WifiConnect);
    if enable_hotspot {
        info!("In setup state, showing hotspot screen and enabling the hotspot");
        settings.set_setup_state(&common::SetupState::Hotspot);
        settings.set_active_screen(&common::ScreenId::Setup);
        shell_sender
            .send(common::ShellCommand::Reset {
                from_matrix: false,
                from_webserver: None,
            })
            .unwrap();
    } else {
        if settings.get_rotation_time() == Duration::from_secs(0) {
            settings.set_rotation_time(Duration::from_secs(10));
        }
        shell_sender
            .send(common::ShellCommand::SetHotspot(false))
            .unwrap();
    }

    let v2_url = env::var("V2_URL").unwrap_or_else(|_| V2_URL.to_string()); // TODO use the actual new AWS URL

    // Setup ScreenProvider map
    let mut map: HashMap<ScreenId, Box<dyn ScreenProvider>> = HashMap::new();

    let sports: AWSScreen = AWSScreen::new(
        scheduler_sender.clone(),
        v2_url,
        api_key,
        settings.get_settings(),
        matrix::FontBook::new(&root_path),
        matrix::PixelBook::new(&root_path),
    );
    map.insert(ScreenId::Smart, Box::new(sports));

    // Clock
    let clock = clock::Clock::new(
        scheduler_sender.clone(),
        settings.get_settings(),
        matrix::FontBook::new(&root_path),
    );
    map.insert(ScreenId::Clock, Box::new(clock));

    // Setup Screen
    let setup_screen = setup_screen::SetupScreen::new(
        scheduler_sender.clone(),
        settings.get_settings().setup_state,
        matrix::FontBook::new(&root_path),
        matrix::PixelBook::new(&root_path),
    );
    map.insert(ScreenId::Setup, Box::new(setup_screen));

    // Flappy Bird Game Screen
    let flappy = flappy::Flappy::new(
        scheduler_sender.clone(),
        settings.get_settings(),
        matrix::FontBook::new(&root_path),
        matrix::PixelBook::new(&root_path),
    );
    map.insert(ScreenId::Flappy, Box::new(flappy));

    // Custom Message Screen
    let custom_message= custom_message::CustomMessageScreen::new(
        common::read_custom_message(&root_path),
        scheduler_sender.clone(),
        matrix::FontBook::new(&root_path),
    );
    map.insert(ScreenId::CustomMessage, Box::new(custom_message));

    // Animation Test
    let animation = AnimationTestScreen::new(scheduler_sender.clone());
    map.insert(ScreenId::Animation, Box::new(animation));

    let default = 3;
    let slowdown: u32 = env::var("SCOREBOARD_SLOWDOWN")
        .ok()
        .map(|s| s.parse().unwrap_or(default))
        .unwrap_or(default);

    // Message Screen
    let message_screen =
        message::MessageScreen::new(scheduler_sender.clone(), matrix::FontBook::new(&root_path));
    // Setup the actual matrix and run it
    // Setup matrix options
    let mut options = rpi_led_matrix::LedMatrixOptions::new();
    let mut rt_options = rpi_led_matrix::LedRuntimeOptions::new();
    options.set_rows(32);
    options.set_cols(64);
    options.set_hardware_mapping("adafruit-hat-pwm");
    options.set_pwm_lsb_nanoseconds(50);
    options.set_refresh_rate(false);
    let _ignored_brightness_result = options.set_brightness(settings.get_brightness());
    info!("setting drop privileges to false");
    rt_options.set_drop_privileges(false);
    rt_options.set_gpio_slowdown(slowdown);
    let led_matrix: rpi_led_matrix::LedMatrix =
        rpi_led_matrix::LedMatrix::new(Some(options), Some(rt_options))
            .expect("Could not setup matrix");

    let mut scheduler = scheduler::Scheduler::new(scheduler_receiver, matrix_sender.clone());
    std::thread::spawn(move || {
        scheduler.run();
    });

    let daily_reboot: bool = env::var("DAILY_REBOOT")
        .map(|reboot_value| reboot_value.parse().unwrap_or(true))
        .unwrap_or(true);
    let reboot_time: u8 = std::cmp::min(
        env::var("REBOOT_TIME")
            .map(|reboot_time| reboot_time.parse::<u8>().unwrap_or(3))
            .unwrap_or(3),
        23,
    );
    let daily_reboot = match daily_reboot {
        true => Some(reboot_time),
        false => None,
    };

    let matrix_senders = matrix::Senders {
        webserver_responder: web_response_sender,
        shell_sender,
        scheduler_sender
    };

    let mut matrix = Matrix::new(
        led_matrix,
        message_screen,
        matrix_receiver,
        map,
        settings,
        matrix_senders,
        daily_reboot,
    );

    let webserver_sender = matrix_sender;
    std::thread::spawn(move || {
        webserver::run_webserver(webserver_sender, web_response_receiver, root_path);
    });
    info!("Starting matrix runner");
    matrix.run();
    Ok(())
}
