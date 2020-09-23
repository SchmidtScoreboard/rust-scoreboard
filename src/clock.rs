use crate::common;
use crate::common::ScoreboardSettingsData;
use crate::matrix;

use chrono::Utc;
use chrono_tz::Tz;
use rpi_led_matrix;
use std::sync::mpsc;

pub struct Clock {
    sender: mpsc::Sender<common::MatrixCommand>,
    timezone: Tz,
    fonts: matrix::FontBook,
}

impl Clock {
    pub fn new(
        sender: mpsc::Sender<common::MatrixCommand>,
        timezone: String,
        root_path: &std::path::Path,
    ) -> Clock {
        Clock {
            sender,
            timezone: timezone.parse().expect("Failed to parse timezone"),
            fonts: matrix::FontBook::new(root_path),
        }
    }
}
impl matrix::ScreenProvider for Clock {
    fn activate(self: &mut Self) {
        info!("Activating Clock");

        self.sender
            .send(common::MatrixCommand::Display(common::ScreenId::Clock))
            .unwrap();
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
            canvas_height / 2 - text_dimensions.height / 2,
            &color,
            0,
            false,
        );
    }
}
