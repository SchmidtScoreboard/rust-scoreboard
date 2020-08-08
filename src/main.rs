use rpi_led_matrix;
use scoreboard_rust::{Hockey, Matrix, ScreenId, ScreenProvider};
use std::collections::HashMap;
use std::sync::mpsc;

fn get_options() -> rpi_led_matrix::LedMatrixOptions {
    let mut options = rpi_led_matrix::LedMatrixOptions::new();
    options.set_rows(32);
    options.set_cols(64);
    options.set_hardware_mapping("adafruit-hat-pwm");
    options
}

fn main() {
    // Set up original channel
    let (tx, rx) = mpsc::channel();

    // Setup matrix options
    let options = get_options();
    let led_matrix: rpi_led_matrix::LedMatrix =
        rpi_led_matrix::LedMatrix::new(Some(options)).expect("Could not setup matrix");

    // Setup ScreenProvider map
    let mut map: HashMap<ScreenId, Box<dyn ScreenProvider>> = HashMap::new();
    let hockey = Hockey::new(&led_matrix, tx);
    map.insert(ScreenId::Hockey, Box::new(hockey));

    let map = map;

    // Setup the actual matrix and run it
    let matrix = Matrix::new(&led_matrix, rx, map);
    matrix.run();
}
