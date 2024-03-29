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

    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }
    pub fn unset_message(&mut self) {
        self.message = None;
    }

    pub fn is_message_set(&self) -> bool {
        self.message.is_some()
    }
}

impl matrix::ScreenProvider for MessageScreen {
    fn activate(&mut self) {}

    fn deactivate(&mut self) {}

    fn draw(&mut self, canvas: &mut rpi_led_matrix::LedCanvas) {
        if let Some(message) = &self.message {
            matrix::draw_message(canvas, &self.fonts.font4x6, message, &mut self.waves_anim);
            self.send_draw_command(Some(Duration::from_millis(20)));
        }
    }

    fn get_screen_id(&self) -> common::ScreenId {
        common::ScreenId::Message
    }

    fn get_sender(&self) -> &mpsc::Sender<scheduler::DelayedCommand> {
        &self.sender
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
