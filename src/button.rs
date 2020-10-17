use crate::common;
use std::sync::mpsc;

pub struct ButtonHandler {
    commandSender: mpsc::Sender<common::MatrixCommand>,
}

impl ButtonHandler {
    pub fn new(commandSender: mpsc::Sender<common::MatrixCommand>) -> ButtonHandler {
        ButtonHandler { commandSender }
    }

    pub fn run(self: &Self) {
        // Main run thread
    }

    // Send display on/off command
    fn handle_single_press(self: &Self) {
        self.commandSender
            .send(common::MatrixCommand::SetPower {
                from_webserver: false,
                power: None,
            })
            .unwrap();
    }

    // Send show sync command
    fn handle_double_press(self: &Self) {
        self.commandSender
            .send(common::MatrixCommand::SyncCommand {
                from_webserver: false,
                show_sync: None,
            })
            .unwrap();
    }

    // Reset scoreboard to factory settings
    fn handle_long_press(self: &Self) {
        self.commandSender.
            .send(common::MatrixCommand::Reset {
                from_webserver: false,
            })
            .unwrap();
    }
}
