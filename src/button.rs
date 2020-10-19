use crate::common;
use std::sync::mpsc;
use std::time::{Duration, Instant};

// use rust_gpiozero;
use std::thread::sleep;
use sysfs_gpio;

use std::io;
const LONG_PRESS_DURATION: Duration = Duration::from_secs(5);
const DOUBLE_PRESS_WINDOW: Duration = Duration::from_millis(250);
pub struct ButtonHandler {
    command_sender: mpsc::Sender<common::MatrixCommand>,
    pin: sysfs_gpio::Pin,
    logged_error: bool,
}

enum ButtonState {
    PRESSED,
    RELEASED,
}

impl ButtonHandler {
    pub fn new(command_sender: mpsc::Sender<common::MatrixCommand>) -> ButtonHandler {
        let pin = sysfs_gpio::Pin::new(25);
        ButtonHandler {
            command_sender,
            pin,
            logged_error: false,
        }
    }
    fn get_pin_value(self: &Self) -> Result<ButtonState, sysfs_gpio::Error> {
        match self.pin.get_value() {
            Ok(value) if value == 1 => Ok(ButtonState::PRESSED),
            Ok(value) if value == 0 => Ok(ButtonState::RELEASED),
            Err(e) => {
                if !self.logged_error {
                    error!("Error reading from pin {:?}", e);
                }
                Err(e)
            }
            _ => {
                if !self.logged_error {
                    error!("Got invalid value from pin");
                }
                Err(sysfs_gpio::Error::from(io::Error::from(
                    io::ErrorKind::InvalidData,
                )))
            }
        }
    }

    pub fn run(self: &mut Self) {
        // Main run thread
        info!("Entering button run");

        // let mut input = rust_gpiozero::Button::new_with_pulldown(22);
        let mut button_value = self.get_pin_value().unwrap();
        let mut last_release_time: Option<Instant> = None;
        let mut last_press_time: Option<Instant> = None;
        loop {
            let new_value = self.get_pin_value().unwrap();
            let now = Instant::now();
            match (&button_value, &new_value) {
                (ButtonState::PRESSED, ButtonState::PRESSED)
                | (ButtonState::RELEASED, ButtonState::RELEASED) => {}
                (ButtonState::RELEASED, ButtonState::PRESSED) => {
                    info!("Button pressed!");
                    last_press_time = Some(now);
                }
                (ButtonState::PRESSED, ButtonState::RELEASED) => {
                    info!("Button released!");
                    if let Some(lpt) = last_press_time {
                        if now.duration_since(lpt) > LONG_PRESS_DURATION {
                            self.handle_long_press();
                            last_release_time = None;
                        } else if let Some(lrt) = last_release_time {
                            if now.duration_since(lrt) < DOUBLE_PRESS_WINDOW {
                                self.handle_double_press();
                                last_release_time = None;
                            }
                        } else {
                            last_release_time = Some(now);
                        }
                    }
                }
            }

            if let Some(lrt) = last_release_time {
                if now.duration_since(lrt) > DOUBLE_PRESS_WINDOW {
                    self.handle_single_press();
                    last_release_time = None;
                }
            }
            button_value = new_value;
        }
    }

    // Reset scoreboard to factory settings
    fn handle_long_press(self: &Self) {
        info!("LONG PRESS");
        self.command_sender
            .send(common::MatrixCommand::Reset {
                from_webserver: false,
            })
            .unwrap()
    }
    // Send display on/off command
    fn handle_single_press(self: &Self) {
        info!("SHORT PRESS");
        self.command_sender
            .send(common::MatrixCommand::SetPower {
                from_webserver: false,
                power: None,
            })
            .unwrap();
    }

    // Send show sync command
    fn handle_double_press(self: &Self) {
        info!("DOUBLE PRESS");
        self.command_sender
            .send(common::MatrixCommand::SyncCommand {
                from_webserver: false,
                show_sync: None,
            })
            .unwrap();
    }
}
