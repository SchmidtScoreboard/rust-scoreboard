use crate::animation;
use crate::common;
use crate::matrix;
use std::any::Any;
use std::sync::mpsc;
use std::time::Duration;

// An ephermeral message screen, used to display
// progress of certain actions
pub struct MessageScreen {
    waves_anim: animation::WavesAnimation,
    message: String,
    sender: mpsc::Sender<common::MatrixCommand>,
    fonts: matrix::FontBook,
}

impl MessageScreen {
    pub fn new(
        message: String,
        sender: mpsc::Sender<common::MatrixCommand>,
        fonts: matrix::FontBook,
    ) -> MessageScreen {
        MessageScreen {
            waves_anim: animation::WavesAnimation::new(64),
            message,
            sender,
            fonts,
        }
    }
}

impl matrix::ScreenProvider for MessageScreen {
    fn activate(self: &mut Self) {}

    fn deactivate(self: &Self) {}

    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        matrix::draw_message(
            canvas,
            &self.fonts.font4x6,
            &self.message,
            &mut self.waves_anim,
        );
        self.send_draw_command(Some(Duration::from_millis(20)));
    }

    fn update_settings(self: &mut Self, _settings: common::ScoreboardSettingsData) {}

    fn get_screen_id(self: &Self) -> common::ScreenId {
        error!("Attempting to get the screen ID of the message screen. This should never happen");
        common::ScreenId::Message
    }

    fn get_sender(self: &Self) -> mpsc::Sender<common::MatrixCommand> {
        self.sender.clone()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
