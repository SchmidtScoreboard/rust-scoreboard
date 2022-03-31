use crate::common;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use std::thread::sleep;

use std::io;
use users::{get_current_uid, get_user_by_uid};
const LONG_PRESS_DURATION: Duration = Duration::from_secs(5);
const DOUBLE_PRESS_WINDOW: Duration = Duration::from_millis(250);
pub struct ButtonHandler {
    command_sender: mpsc::Sender<common::MatrixCommand>,
    pin: sysfs_gpio::Pin,
}

enum ButtonState {
    Pressed,
    Released,
}

impl ButtonHandler {
    pub fn new(command_sender: mpsc::Sender<common::MatrixCommand>) -> ButtonHandler {
        let pin = sysfs_gpio::Pin::new(25);
        if pin.is_exported() {
            info!("Unexporting pin");
            pin.unexport().expect("Faile to unexport pin");
            sleep(Duration::from_millis(500));
        }
        info!("Exporting pin");
        pin.export().expect("Failed to export pin");
        sleep(Duration::from_millis(500));
        ButtonHandler {
            command_sender,
            pin,
        }
    }
    fn get_pin_value(&self) -> Result<ButtonState, sysfs_gpio::Error> {
        match self.pin.get_value() {
            Ok(value) if value == 1 => Ok(ButtonState::Pressed),
            Ok(value) if value == 0 => Ok(ButtonState::Released),
            Err(e) => {
                let user = get_user_by_uid(get_current_uid()).unwrap();
                info!(
                    "Hello, {} {}!",
                    user.name().to_string_lossy(),
                    get_current_uid()
                );
                error!("Error reading from pin {:?}", e);
                Err(e)
            }
            _ => {
                error!("Got invalid value from pin");
                Err(sysfs_gpio::Error::from(io::Error::from(
                    io::ErrorKind::InvalidData,
                )))
            }
        }
    }

    pub fn run(&mut self) {
        // Main run thread
        info!("Entering button run");

        let mut button_value = self.get_pin_value().unwrap();
        let mut last_release_time: Option<Instant> = None;
        let mut last_press_time: Option<Instant> = None;
        loop {
            let now = Instant::now();
            match self.get_pin_value() {
                Ok(new_value) => {
                    match (&button_value, &new_value) {
                        (ButtonState::Pressed, ButtonState::Pressed)
                        | (ButtonState::Released, ButtonState::Released) => {}
                        (ButtonState::Released, ButtonState::Pressed) => {
                            info!("Button pressed!");
                            last_press_time = Some(now);
                        }
                        (ButtonState::Pressed, ButtonState::Released) => {
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
                    sleep(Duration::from_millis(10));
                }
                Err(_) => {
                    sleep(Duration::from_secs(5));
                }
            }
        }
    }

    // Reset scoreboard to factory settings
    fn handle_long_press(&self) {
        info!("LONG PRESS");
        self.command_sender
            .send(common::MatrixCommand::Reset {
                from_webserver: false,
            })
            .unwrap()
    }
    // Send display on/off command
    fn handle_single_press(&self) {
        info!("SHORT PRESS");
        self.command_sender
            .send(common::MatrixCommand::SetPower {
                source: common::CommandSource::Button(),
                power: None,
            })
            .unwrap();
    }

    // Send show sync command
    fn handle_double_press(&self) {
        info!("DOUBLE PRESS");
        self.command_sender
            .send(common::MatrixCommand::SyncCommand {
                from_webserver: false,
                show_sync: None,
            })
            .unwrap();
    }
}
