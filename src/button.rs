use crate::common;
use std::sync::mpsc;
use sysfs_gpio;

pub struct ButtonHandler {
    command_sender: mpsc::Sender<common::MatrixCommand>,
}

impl ButtonHandler {
    pub fn new(command_sender: mpsc::Sender<common::MatrixCommand>) -> ButtonHandler {
        ButtonHandler { command_sender }
    }

    pub fn run(self: &Self) {
        // Main run thread
        let _input = sysfs_gpio::Pin::new(25);
    }

    // Send display on/off command
    fn handle_single_press(self: &Self) {
        self.command_sender
            .send(common::MatrixCommand::SetPower {
                from_webserver: false,
                power: None,
            })
            .unwrap();
    }

    // Send show sync command
    fn handle_double_press(self: &Self) {
        self.command_sender
            .send(common::MatrixCommand::SyncCommand {
                from_webserver: false,
                show_sync: None,
            })
            .unwrap();
    }

    // Reset scoreboard to factory settings
    fn handle_long_press(self: &Self) {
        self.command_sender
            .send(common::MatrixCommand::Reset {
                from_webserver: false,
            })
            .unwrap()
    }
}
