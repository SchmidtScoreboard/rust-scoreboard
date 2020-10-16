#![feature(proc_macro_hygiene, decl_macro)]

mod animation;
mod aws_screen;
mod baseball;
mod clock;
mod common;
mod game;
mod hockey;
mod matrix;
mod scoreboard_settings;
mod setup_screen;
mod webserver;
#[macro_use]
extern crate rust_embed;

#[macro_use]
extern crate log;

use animation::AnimationTestScreen;
use aws_screen::AWSScreen;
use baseball::BaseballGame;
use common::ScreenId;
use hockey::HockeyGame;
use matrix::{Matrix, ScreenProvider};
use rpi_led_matrix;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;

fn main() {
    let mut arguments = env::args();
    arguments.next(); // skip program name
    let root_path = match arguments.next() {
        Some(arg) => PathBuf::from(arg),
        None => PathBuf::from("/var/lib/scoreboard/"),
    };
    let log_dir = root_path.join("logs");
    let _create_dir_result = fs::create_dir(&log_dir);

    flexi_logger::Logger::with_env()
        .log_to_file()
        .directory(log_dir)
        .duplicate_to_stdout(flexi_logger::Duplicate::All)
        .format_for_stdout(flexi_logger::colored_default_format)
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(3),
        )
        .start()
        .unwrap();

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

    let mut settings = scoreboard_settings::ScoreboardSettings::new(settings_data, settings_path);

    if settings.get_settings_clone().setup_state == common::SetupState::Factory {
        settings.set_setup_state(&common::SetupState::Hotspot);
    }
    // TODO setup button listener with sender end of channel

    let (tx, rx) = mpsc::channel();

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

    let (web_response_sender, web_response_receiver) = mpsc::channel();
    let mut matrix = Matrix::new(led_matrix, rx, map, settings, web_response_sender);

    let webserver_sender = tx.clone();
    std::thread::spawn(move || {
        webserver::run_webserver(webserver_sender, web_response_receiver, root_path);
    });
    matrix.run();
}
