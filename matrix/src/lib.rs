use rpi_led_matrix;
use std::collections::HashMap;
use std::sync::mpsc;

pub struct Matrix<'a> {
    led_matrix: &'a rpi_led_matrix::LedMatrix,
    receiver: mpsc::Receiver<common::MatrixCommand>,
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
            screens_map: map,
        }
    }

    fn activateScreen(
        self: &'a Self,
        id: common::ScreenId,
    ) -> (common::ScreenId, &'a Box<dyn ScreenProvider + 'a>) {
        let screen = self
            .screens_map
            .get(&id)
            .expect("Could not find screen {id}");
        screen.activate();
        (id, screen)
    }
    // This is the main loop of the entire code
    // Call this after everything else is set up
    pub fn run(self: Self, active_id: common::ScreenId) {
        let (mut active_id, mut active_screen) = self.activateScreen(active_id);

        // TODO wait on the receiver, complete MatrixCommands
        loop {
            let command = self.receiver.recv().unwrap();
            // let command = command.unwrap(); // Get the actual command
            match command {
                common::MatrixCommand::SetActiveScreen(id) => {
                    let (id, screen) = self.activateScreen(id); // really wish I could bind this differently
                    active_id = id;
                    active_screen = screen;
                }
                common::MatrixCommand::SetPower(on) => {
                    // TODO set power to the matrix
                }
                common::MatrixCommand::Display(id, canvas) => {
                    if id == active_id {
                        // If the id received matches the active id, display the image
                        self.led_matrix.swap(canvas);
                        // Request a new screen
                    }
                }
            };
        }
    }
}

pub trait ScreenProvider {
    // Activate is called by the Display driver
    // Activate sets up whatever refreshing this screen needs
    // Use local reference to the matrix to get a canvas and fill it in on refreshes
    // Use push pipe to push the pipe back to the Display driver
    fn activate(self: &Self) {}

    // Cleanup any unused resources
    // Most screens won't have to do anything here
    // If there are owned threads, cancel them
    fn deactivate(self: &Self) {}

    // Request a filled in canvas at your earliest convenience
    fn draw(self: &Self, canvas: rpi_led_matrix::LedCanvas);
}
