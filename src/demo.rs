mod animation;
mod aws_screen;
mod baseball;
mod basketball;
mod clock;
mod common;
mod custom_message;
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
mod sport;
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

fn main() {
    let matches = clap::App::new("Schmidt Scoreboard Demo")
        .version(self_update::cargo_crate_version!())
        .author("Mark Schmidt <mark.schmidt@hey.com>")
        .about("Runs a demo of the Schmidt Scoreboard that does not require WiFi or perform setup")
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

    info!("Starting up demo");

    patch_notes::log_patch_notes();
    let settings_path = root_path.join("scoreboard_settings.json");

    let settings_data: common::ScoreboardSettingsData =
        serde_json::from_str(include_str!("../assets/demo/scoreboard_settings.json"))
            .expect("Could not parse scoreboard settings from demo json");
    let settings_data = Arc::from(settings_data);

    let (matrix_sender, matrix_receiver) = mpsc::channel();
    let (scheduler_sender, scheduler_receiver) = mpsc::channel();
    let (web_response_sender, _web_response_receiver) = mpsc::channel();
    let (shell_sender, _shell_receiver) = mpsc::channel();

    let settings = scoreboard_settings::ScoreboardSettings::new(settings_data, settings_path);

    // Setup ScreenProvider map
    let mut map: HashMap<ScreenId, Box<dyn ScreenProvider>> = HashMap::new();

    let sports: AWSScreen = AWSScreen::new(
        scheduler_sender.clone(),
        "".to_string(),
        "".to_string(),
        settings.get_settings(),
        matrix::FontBook::new(&root_path),
        matrix::PixelBook::new(&root_path),
        matrix::MatrixMode::Demo,
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
    let custom_message = custom_message::CustomMessageScreen::new(
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

    let matrix_senders = matrix::Senders {
        webserver_responder: web_response_sender,
        shell_sender,
        scheduler_sender,
    };

    let mut matrix = Matrix::new(
        led_matrix,
        message_screen,
        matrix_receiver,
        map,
        settings,
        matrix_senders,
        None,
        matrix::MatrixMode::Demo,
    );

    info!("Starting matrix runner");
    matrix.run();
}
