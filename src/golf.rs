use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;

use chrono_tz::Tz;
use rpi_led_matrix;
use serde::Deserialize;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(Deserialize, Debug, Clone)]
pub struct Player {
    display_name: String,
    position: u32,
    score: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Golf {
    pub common: game::CommonGameData,
    pub name: String,
    pub players: Vec<Player>
}

impl Ord for Golf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.common.cmp(&other.common)
    }
}

impl PartialOrd for Golf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Golf {
    fn eq(&self, other: &Self) -> bool {
        (&self.common,) == (&other.common,)
    }
}

impl Eq for Golf {}

impl aws_screen::AWSScreenType for Golf {
    fn draw_screen(
        self: &Self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        _pixels_book: &matrix::PixelBook,
        timezone: &Tz,
    ) {
        let font = &font_book.font4x6; // Use the smallest font to fit the most info
        let green = common::new_color(0, 255, 0);

        canvas.draw_text(
            &font.led_font,
            &self.name.to_ascii_uppercase(),
            5,
            font.dimensions.height,
            &green,
            0,
            false);
        
        
    }
}
