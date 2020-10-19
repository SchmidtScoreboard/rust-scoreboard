use crate::common;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use rust_gpiozero;

const LONG_PRESS_DURATION: Duration = Duration::from_secs(5);
const DOUBLE_PRESS_WINDOW: Duration = Duration::from_millis(600);
const DOUBLE_PRESS_DEBOUNCE: Duration = Duration::from_millis(500);
pub struct ButtonHandler {
    command_sender: mpsc::Sender<common::MatrixCommand>,
}
// Send display on/off command
fn handle_single_press(command_sender: mpsc::Sender<common::MatrixCommand>) {
    command_sender
        .send(common::MatrixCommand::SetPower {
            from_webserver: false,
            power: None,
        })
        .unwrap();
}

// Send show sync command
fn handle_double_press(command_sender: mpsc::Sender<common::MatrixCommand>) {
    command_sender
        .send(common::MatrixCommand::SyncCommand {
            from_webserver: false,
            show_sync: None,
        })
        .unwrap();
}

impl ButtonHandler {
    pub fn new(command_sender: mpsc::Sender<common::MatrixCommand>) -> ButtonHandler {
        ButtonHandler { command_sender }
    }

    pub fn run(self: &Self) {
        // Main run thread

        let mut input = rust_gpiozero::Button::new_with_pulldown(25);
        let mut last_release_context: Option<(Instant, mpsc::Sender<()>)> = None;
        loop {
            input.wait_for_press(None);
            let press_time = Instant::now();

            input.wait_for_release(None);
            let release_time = Instant::now();

            // First, check if this is a long press
            if release_time.duration_since(press_time) > LONG_PRESS_DURATION {
                self.handle_long_press();
            }

            match last_release_context {
                Some((last_release_time, tx))
                    if release_time.duration_since(last_release_time) < DOUBLE_PRESS_DEBOUNCE =>
                {
                    tx.send(()).unwrap();
                    last_release_context = None;
                }
                _ => {
                    let command_sender = self.command_sender.clone();
                    let (tx, rx) = mpsc::channel();
                    last_release_context = Some((release_time, tx));
                    std::thread::spawn(move || match rx.recv_timeout(DOUBLE_PRESS_WINDOW) {
                        Ok(_) => handle_double_press(command_sender),
                        Err(_) => handle_single_press(command_sender),
                    });
                }
            }
        }
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
