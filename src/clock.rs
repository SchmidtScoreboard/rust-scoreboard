use crate::common;
use crate::common::ScoreboardSettingsData;
use crate::matrix;

use chrono::Utc;
use chrono_tz::Tz;
use rpi_led_matrix;
use std::any::Any;
use std::sync::mpsc;
use std::time::Duration;

pub struct Clock {
    sender: mpsc::Sender<common::MatrixCommand>,
    timezone: Tz,
    fonts: matrix::FontBook,
}

impl Clock {
    pub fn new(
        sender: mpsc::Sender<common::MatrixCommand>,
        timezone: String,
        fonts: matrix::FontBook,
    ) -> Clock {
        Clock {
            sender,
            timezone: timezone.parse().expect("Failed to parse timezone"),
            fonts,
        }
    }
}
impl matrix::ScreenProvider for Clock {
    fn activate(self: &mut Self) {
        info!("Activating Clock");
        self.send_draw_command(None);
    }

    fn update_settings(self: &mut Self, _settings: ScoreboardSettingsData) {}

    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        let now = Utc::now();
        let clock_text = format!("{}", now.with_timezone(&self.timezone).format("%-I:%M %p"));
        let font = &self.fonts.font7x13;
        let text_dimensions = font.get_text_dimensions(&clock_text);
        let (canvas_width, canvas_height) = canvas.canvas_size();
        let color = common::new_color(255, 255, 255);
        canvas.draw_text(
            &font.led_font,
            &clock_text,
            canvas_width / 2 - text_dimensions.width / 2,
            canvas_height / 2 + text_dimensions.height / 2,
            &color,
            0,
            false,
        );
        self.send_draw_command(Some(Duration::from_secs(30)));
    }

    fn get_sender(self: &Self) -> mpsc::Sender<common::MatrixCommand> {
        self.sender.clone()
    }

    fn get_screen_id(self: &Self) -> common::ScreenId {
        common::ScreenId::Clock
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
