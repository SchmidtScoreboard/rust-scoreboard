use crate::{common, matrix::{FontBook, ScreenProvider, draw_pixels}};
use std::sync::mpsc::Sender;
use crate::scheduler;
use std::time::Duration;


pub struct CustomMessageScreen {
    message: common::CustomMessage,
    fonts: FontBook,
    sender: Sender<scheduler::DelayedCommand>,
}


impl CustomMessageScreen {
    pub fn new(
        message: common::CustomMessage,
        sender: Sender<scheduler::DelayedCommand>,
        fonts: FontBook,
    ) -> CustomMessageScreen {
        CustomMessageScreen {
            message,
            sender,
            fonts,
        }
    }

    pub fn set_message(self: &mut Self, new_message: common::CustomMessage) {
        self.message = new_message;
    }

    pub fn get_message(self: &Self) -> common::CustomMessage {
        self.message.clone()
    }
}

impl ScreenProvider for CustomMessageScreen {
    fn activate(self: &mut Self) {
        self.send_draw_command(None);
    }
    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        // Start by drawing the pixels
        draw_pixels(canvas, &self.message.background, (0,0));

        let mut y = 0;
        self.message.texts.iter().for_each(|line| {
            let (font, padding) = match line.size {
                common::FontSize::Small => (&self.fonts.font4x6, 1),
                common::FontSize::Medium => (&self.fonts.font5x8, 2),
                common::FontSize::Large => (&self.fonts.font7x13, 2)
            };
            let text_dimensions = font.get_text_dimensions(&line.text);
            let x = 64 / 2 - text_dimensions.width / 2;
            canvas.draw_text(&font.led_font, &line.text, x, y + padding, &line.color, 0, false);
            y = y + padding * 2 + text_dimensions.height;
        });

        self.send_draw_command(Some(Duration::from_millis(20)));
    }

    fn get_screen_id(self: &Self) -> crate::common::ScreenId {
        crate::common::ScreenId::CustomMessage
    }

    fn get_sender(self: &Self) -> &Sender<scheduler::DelayedCommand> {
        &self.sender
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn has_priority(self: &mut Self, power_mode: &common::AutoPowerMode) -> bool {
        power_mode == &common::AutoPowerMode::CustomMessage
    }
}
