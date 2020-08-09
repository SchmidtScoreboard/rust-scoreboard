use rpi_led_matrix;
use std::collections::HashMap;
use std::sync::mpsc;

pub struct Matrix<'a> {
    led_matrix: &'a rpi_led_matrix::LedMatrix,
    receiver: mpsc::Receiver<common::MatrixCommand>,
    active_id: common::ScreenId,
    screens_map: HashMap<common::ScreenId, Box<dyn ScreenProvider + 'a>>,
}

impl<'a> Matrix<'a> {
    pub fn new(
        led_matrix: &'a rpi_led_matrix::LedMatrix,
        receiver: mpsc::Receiver<common::MatrixCommand>,
        map: HashMap<common::ScreenId, Box<dyn ScreenProvider + 'a>>,
    ) -> Matrix<'a> {
        Matrix {
            led_matrix,
            receiver,
            active_id: common::ScreenId::Hockey, // TODO build this from settings/constructor
            screens_map: map,
        }
    }

    // This is the main loop of the entire code
    // Call this after everything else is set up
    pub fn run(self: Self) {
        let start_screen = self
            .screens_map
            .get(&self.active_id)
            .expect("Could not find screen {self.active_id}");

        // TODO reenable refresh screen
        // let refresh_screen = self
        //     .screens_map
        //     .get(&ScreenId::Refresh)
        //     .expect("Could not find refresh screen");

        // if start_screen.should_show_refresh_on_activate() {
        //     refresh_screen.activate();
        // }

        start_screen.activate();

        // TODO wait on the receiver, complete MatrixCommands
        // while let command = self.receiver.recv() {
        //     let command = command.unwrap(); // Get the actual command
        //                                     // mat
        // }
    }
}

pub trait ScreenProvider {
    // Activate is called by the Display driver
    // Activate sets up whatever refreshing this screen needs
    // Use local reference to the matrix to get a canvas and fill it in on refreshes
    // Use push pipe to push the pipe back to the Display driver
    fn activate(self: &Self);
    fn should_show_refresh_on_activate(self: &Self) -> bool {
        false
    }
}
