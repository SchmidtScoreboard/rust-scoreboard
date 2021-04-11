use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;

use rpi_led_matrix;
use serde::Deserialize;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

use chrono_tz::Tz;
#[derive(Deserialize, Debug, Clone)]
pub struct FootballGame {
    pub common: game::CommonGameData,
}

impl Ord for FootballGame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.common.cmp(&other.common)
    }
}

impl PartialOrd for FootballGame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FootballGame {
    fn eq(&self, other: &Self) -> bool {
        (&self.common,) == (&other.common,)
    }
}

impl Eq for FootballGame {}

#[derive(Deserialize, Debug, Clone)]
pub struct CollegeFootballGame {
    pub common: game::CommonGameData,
}

impl Ord for CollegeFootballGame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.common.cmp(&other.common)
    }
}

impl PartialOrd for CollegeFootballGame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CollegeFootballGame {
    fn eq(&self, other: &Self) -> bool {
        (&self.common,) == (&other.common,)
    }
}

impl Eq for CollegeFootballGame {}

fn football_draw(
    common: &game::CommonGameData,
    canvas: &mut rpi_led_matrix::LedCanvas,
    font_book: &matrix::FontBook,
    timezone: &Tz,
) {
    let font = &font_book.font4x6;
    game::draw_scoreboard(canvas, &font, &common, 2);

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
    if common.status == game::GameStatus::END {
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

impl aws_screen::AWSScreenType for FootballGame {
    fn draw_screen(
        self: &Self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        _pixels_book: &matrix::PixelBook,
        timezone: &Tz,
    ) {
        football_draw(&self.common, canvas, font_book, timezone);
    }
}

impl game::Sport for FootballGame {
    fn get_common(self: &Self) -> &game::CommonGameData {
         &self.common
    }
}

impl aws_screen::AWSScreenType for CollegeFootballGame {
    fn draw_screen(
        self: &Self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        _pixels_book: &matrix::PixelBook,
        timezone: &Tz,
    ) {
        football_draw(&self.common, canvas, font_book, timezone);
    }
}
impl game::Sport for CollegeFootballGame {
    fn get_common(self: &Self) -> &game::CommonGameData {
         &self.common
    }
}
