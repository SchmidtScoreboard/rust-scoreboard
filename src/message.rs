use crate::animation;
use crate::common;
use crate::matrix;
use crate::scheduler;
use std::any::Any;
use std::sync::mpsc;
use std::time::Duration;

// An ephermeral message screen, used to display
// progress of certain actions
pub struct MessageScreen {
    waves_anim: animation::WavesAnimation,
    message: Option<String>,
    sender: mpsc::Sender<scheduler::DelayedCommand>,
    fonts: matrix::FontBook,
}

impl MessageScreen {
    pub fn new(
        sender: mpsc::Sender<scheduler::DelayedCommand>,
        fonts: matrix::FontBook,
    ) -> MessageScreen {
        MessageScreen {
            waves_anim: animation::WavesAnimation::new(64),
            message: None,
            sender,
            fonts,
        }
    }

    pub fn set_message(self: &mut Self, message: String) {
        self.message = Some(message);
    }
    pub fn unset_message(self: &mut Self) {
        self.message = None;
    }

    pub fn is_message_set(self: &Self) -> bool {
        self.message.is_some()
    }
}

impl matrix::ScreenProvider for MessageScreen {
    fn activate(self: &mut Self) {}

    fn deactivate(self: &mut Self) {}

    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        if let Some(message) = &self.message {
            matrix::draw_message(canvas, &self.fonts.font4x6, message, &mut self.waves_anim);
            self.send_draw_command(Some(Duration::from_millis(20)));
        }
    }

    fn get_screen_id(self: &Self) -> common::ScreenId {
        common::ScreenId::Message
    }

    fn get_sender(self: &Self) -> mpsc::Sender<scheduler::DelayedCommand> {
        self.sender.clone()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
