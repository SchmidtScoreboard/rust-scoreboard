use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;
use chrono_tz::Tz;

use serde::Deserialize;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(Deserialize, Debug, Clone)]
pub struct BasketballGame {
    pub common: game::CommonGameData,
}

impl Ord for BasketballGame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.common.cmp(&other.common)
    }
}

impl PartialOrd for BasketballGame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BasketballGame {
    fn eq(&self, other: &Self) -> bool {
        (&self.common,) == (&other.common,)
    }
}

impl Eq for BasketballGame {}

#[derive(Deserialize, Debug, Clone)]
pub struct CollegeBasketballGame {
    pub common: game::CommonGameData,
}

impl Ord for CollegeBasketballGame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.common.cmp(&other.common)
    }
}

impl PartialOrd for CollegeBasketballGame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CollegeBasketballGame {
    fn eq(&self, other: &Self) -> bool {
        (&self.common,) == (&other.common,)
    }
}

impl Eq for CollegeBasketballGame {}

fn basketball_draw(
    common: &game::CommonGameData,
    canvas: &mut rpi_led_matrix::LedCanvas,
    font_book: &matrix::FontBook,
    timezone: &Tz,
) {
    let font = &font_book.font4x6;
    game::draw_scoreboard(canvas, font, common, 2, (2,2));

    // Draw the current period
    let white = common::new_color(255, 255, 255);
    let yellow = common::new_color(255, 255, 0);

    canvas.draw_text(
        &font.led_font,
        &common.get_ordinal_text(timezone),
        5,
        23 + font.dimensions.height,
        &white,
        0,
        false,
    );

    // Draw FINAL
    if common.status == game::GameStatus::End {
        canvas.draw_text(
            &font.led_font,
            "FINAL",
            36 + font.dimensions.width,
            23 + font.dimensions.height,
            &yellow,
            0,
            false,
        );
    }
}

impl aws_screen::AWSScreenType for BasketballGame {
    fn draw_screen(
        &self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        _pixels_book: &matrix::PixelBook,
        timezone: &Tz,
    ) {
        basketball_draw(&self.common, canvas, font_book, timezone);
    }
}
impl game::Sport for BasketballGame{
    fn get_common(&self) -> &game::CommonGameData {
         &self.common
    }
}

impl aws_screen::AWSScreenType for CollegeBasketballGame {
    fn draw_screen(
        &self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        _pixels_book: &matrix::PixelBook,
        timezone: &Tz,
    ) {
        basketball_draw(&self.common, canvas, font_book, timezone);
    }
}
impl game::Sport for CollegeBasketballGame{
    fn get_common(&self) -> &game::CommonGameData {
         &self.common
    }
}
