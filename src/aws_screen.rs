use crate::game;
use crate::matrix;


pub trait AWSScreenType {

    fn draw_screen(
        self: &Self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        pixels_book: &matrix::PixelBook,
        timezone: &str,
    );

    fn get_refresh_texts(self: &Self) -> Vec<&'static str>;

    fn involves_team(self: &Self, team_id: u32) -> bool;

    fn status(self: &Self) -> game::GameStatus;
}

