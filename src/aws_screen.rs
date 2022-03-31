use crate::matrix;

use chrono_tz::Tz;
pub trait AWSScreenType {
    fn draw_screen(
        &self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        pixels_book: &matrix::PixelBook,
        timezone: &Tz,
    );
}
