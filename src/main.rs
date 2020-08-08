use rpi_led_matrix;
use scoreboard_rust::{Matrix, ScreenId, ScreenProvider};
use std::collections::HashMap;
use std::sync::mpsc;

fn main() {
    // Set up original channel
    let (tx, rx) = mpsc::channel();

    // Setup matrix options
    let mut options = rpi_led_matrix::LedMatrixOptions::new();
    options.set_rows(32);
    options.set_cols(64);
    options.set_hardware_mapping("adafruit-hat-pwm");
    let led_matrix = rpi_led_matrix::LedMatrix::new(Some(options)).expect("Could not setup matrix");

    // Setup ScreenProvider map
    let map: HashMap<ScreenId, Box<dyn ScreenProvider>> = HashMap::new();

    // Setup the actual matrix and run it
    let matrix = Matrix::new(led_matrix, rx, map);
    matrix.run();
}
