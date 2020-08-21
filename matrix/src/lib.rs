use rpi_led_matrix;
use std::collections::HashMap;
use std::sync::mpsc;
use std::time::Duration;

pub struct Matrix<'a> {
    led_matrix: rpi_led_matrix::LedMatrix,
    receiver: mpsc::Receiver<common::MatrixCommand>,
    screens_map: HashMap<common::ScreenId, Box<dyn ScreenProvider + 'a>>,
}

impl<'a> Matrix<'a> {
    pub fn new(
        led_matrix: rpi_led_matrix::LedMatrix,
        receiver: mpsc::Receiver<common::MatrixCommand>,
        map: HashMap<common::ScreenId, Box<dyn ScreenProvider + 'a>>,
    ) -> Matrix<'a> {
        Matrix {
            led_matrix,
            receiver,
            screens_map: map,
        }
    }

    fn get_mut_screen(self: &mut Self, id: &common::ScreenId) -> &mut Box<dyn ScreenProvider + 'a> {
        self.screens_map
            .get_mut(id)
            .expect("Could not find screen {id}")
    }
    fn get_screen(self: &Self, id: &common::ScreenId) -> &Box<dyn ScreenProvider + 'a> {
        self.screens_map
            .get(id)
            .expect("Could not find screen {id}")
    }

    fn activate_screen(self: &mut Self, id: common::ScreenId) -> common::ScreenId {
        let screen = self.get_mut_screen(&id);
        screen.activate();
        id
    }
    // This is the main loop of the entire code
    // Call this after everything else is set up
    pub fn run(self: &mut Self, active_id: common::ScreenId) {
        let mut active_id = self.activate_screen(active_id);

        loop {
            let command = self.receiver.recv().unwrap();
            // let command = command.unwrap(); // Get the actual command
            match command {
                common::MatrixCommand::SetActiveScreen(id) => {
                    active_id = self.activate_screen(id);
                }
                common::MatrixCommand::SetPower(on) => {
                    // TODO set power to the matrix
                }
                common::MatrixCommand::Display(id) => {
                    if id == active_id {
                        // If the id received matches the active id, display the image
                        let canvas = self.led_matrix.offscreen_canvas();
                        self.get_mut_screen(&active_id).draw(canvas);
                    }
                }
            };
        }
    }
}

pub trait ScreenProvider {
    // Activate is called by the Display driver
    // Activate sets up whatever refreshing this screen needs
    fn activate(self: &mut Self) {}

    // Cleanup any unused resources
    // Most screens won't have to do anything here
    // If there are owned threads, cancel them
    fn deactivate(self: &Self) {}

    // Draw is not blocking--fills in the canvas and returns it immediately
    // Draw can check for new data on an internal try_recv, and update internal variables, but
    // it must not issue any network requests or perform any other asynchronous action
    // Asynchronous actions must be driven by a refresh thread set up in `activate`
    fn draw(self: &mut Self, canvas: rpi_led_matrix::LedCanvas) -> rpi_led_matrix::LedCanvas;
}
