use common::ScreenId;
use hockey::Hockey;
use matrix::{Matrix, ScreenProvider};
use rpi_led_matrix;
use std::collections::HashMap;
use std::sync::mpsc;

fn main() {
    // Set up original channel
    let (tx, rx) = mpsc::channel();

    // TODO setup webserver with sender end of channel

    // TODO setup button listener with sender end of channel

    // Setup matrix options
    let mut options = rpi_led_matrix::LedMatrixOptions::new();
    options.set_rows(32);
    options.set_cols(64);
    options.set_hardware_mapping("adafruit-hat-pwm");
    let led_matrix: rpi_led_matrix::LedMatrix =
        rpi_led_matrix::LedMatrix::new(Some(options)).expect("Could not setup matrix");

    // Setup ScreenProvider map
    let mut map: HashMap<ScreenId, Box<dyn ScreenProvider>> = HashMap::new();

    // Hockey
    let hockey = Hockey::new(&led_matrix, tx);
    map.insert(ScreenId::Hockey, Box::new(hockey));

    // TODO add Baseball

    // TODO add refresh and all setup screens

    // Setup the actual matrix and run it
    let matrix = Matrix::new(&led_matrix, rx, map);
    matrix.run();
}
