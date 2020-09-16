mod animation_test;
mod aws_screen;
mod baseball;
mod common;
mod game;
mod hockey;
mod matrix;

use animation_test::AnimationTestScreen;
use aws_screen::AWSScreen;
use baseball::BaseballGame;
use common::ScreenId;
use hockey::HockeyGame;
use matrix::{Matrix, ScreenProvider};
use rpi_led_matrix;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::mpsc;

fn main() {
    let mut arguments = env::args();
    arguments.next(); // skip program name
    let secrets = match arguments.next() {
        Some(arg) => arg,
        None => String::from("/home/pi/rust-scoreboard/secrets.txt"),
    };

    println!("Loading secrets from {}", secrets);

    let api_key = fs::read_to_string(secrets).unwrap();
    // TODO read Scoreboard Settings

    // TODO read secrets.txt

    // Set up original channel
    let (tx, rx) = mpsc::channel();

    // TODO setup webserver with sender end of channel

    // TODO setup button listener with sender end of channel

    // Setup ScreenProvider map
    let mut map: HashMap<ScreenId, Box<dyn ScreenProvider>> = HashMap::new();

    // Hockey
    let hockey: AWSScreen<HockeyGame> = AWSScreen::new(tx.clone(), api_key.clone());
    map.insert(ScreenId::Hockey, Box::new(hockey));

    // Baseball
    let baseball: AWSScreen<BaseballGame> = AWSScreen::new(tx.clone(), api_key.clone());
    map.insert(ScreenId::Baseball, Box::new(baseball));

    // Animation Test
    let animation = AnimationTestScreen::new(tx.clone(), api_key.clone());
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

    let mut matrix = Matrix::new(led_matrix, rx, map);
    // matrix.run(ScreenId::Baseball);
    // matrix.run(ScreenId::Baseball);
    matrix.run(ScreenId::Animation);
}
