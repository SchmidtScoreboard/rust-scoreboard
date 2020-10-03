use crate::animation;
use crate::common;
use crate::game;
use crate::matrix;

use rpi_led_matrix;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub struct SetupScreen {
    sender: mpsc::Sender<common::MatrixCommand>,
    loading_anim: LoadingAnimation,
    state: common::SetupState,
}

impl SetupScreen {
    pub fn new(sender: mpsc::Sender<common::MatrixCommand>) -> SetupScreen {
        SetupScreen {
            sender,
            loading_anim: LoadingAnimation::new(),
        }
    }
}

impl matrix::ScreenProvider for SetupScreen {
    fn activate(self: &mut Self) {
        self.sender
            .send(common::MatrixCommand::Display(common::ScreenId::Setup))
            .unwrap();
    }
    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        match self.state {
            common::SetupState::Hotspot => {
                dbg!("Drawing hotspot");
            }
            common::SetupState::WifiConnect => {
                dbg!("Drawing wifi screen");
            }
            common::SetupState::Sync => {
                dbg!("Drawing wifi screen");
            }
            common::SetupState::Ready => {
                error!("Should not display setup screen while ready");
            }
        }
    }
    fn update_settings(self: &mut Self, _settings: common::ScoreboardSettingsData) {}
}
