use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;

use rpi_led_matrix;
use serde::Deserialize;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use chrono_tz::Tz;

#[derive(Deserialize, Debug, Clone)]
pub struct FootballData {
    pub time_remaining: String,
    pub ball_position: u8, // (0 - 50 is opponent's, 50+ is your own)
    pub down_string: String,
    pub home_possession: bool
}

#[derive(Deserialize, Debug, Clone)]
pub struct FootballGame {
    pub common: game::CommonGameData,
    pub extra_data: Option<FootballData>
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
    pub extra_data: Option<FootballData>
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
    football_data: &Option<FootballData>,
    canvas: &mut rpi_led_matrix::LedCanvas,
    font_book: &matrix::FontBook,
    pixels_book: &matrix::PixelBook,
    timezone: &Tz,
) {
    let font = &font_book.font4x6;
    let accent_box_width = if football_data.is_some() && common.is_active_game() { 7 } else { 2 };
    game::draw_scoreboard(canvas, &font, &common, 2, accent_box_width);

    // Draw the current period
    let white = common::new_color(255, 255, 255);
    let yellow = common::new_color(255, 255, 0);

    let mut draw_bottom_info = |text: &str, position: (i32, i32), color: &rpi_led_matrix::LedColor| {
        canvas.draw_text(
            &font.led_font,
            text,
            position.0,
            position.1 + font.dimensions.height,
            color,
            0,
            false,
        )
    };

    // Draw FINAL
    if common.status == game::GameStatus::END {
        draw_bottom_info("FINAL", (36, 23), &yellow);
        draw_bottom_info(&common.get_ordinal_text(timezone), (5, 23), &white);
    }

    if let Some(football_data) = football_data {
        let top_row_height = 17;
        let bottom_row_height = 25;
        let left_indent = 3;
        let right_indent = 64;
        // Left side
        draw_bottom_info(&common.get_ordinal_text(timezone), (left_indent, top_row_height), &white);
        draw_bottom_info(&football_data.time_remaining, (left_indent, bottom_row_height), &white);

        // Right side
        let ball_on_text = format!("{} {}", if football_data.ball_position > 50 { "Own" } else { "Opp" }, football_data.ball_position % 50);
        let down_string_dimensions = font.get_text_dimensions(&football_data.down_string);
        let ball_on_text_dimensions = font.get_text_dimensions(&ball_on_text);
        draw_bottom_info(&football_data.down_string, (right_indent- down_string_dimensions.width, top_row_height), &white);
        draw_bottom_info(&ball_on_text, (right_indent- ball_on_text_dimensions.width, bottom_row_height), &white);

        // Draw possession
        if common.is_active_game() {
            let football_height = if football_data.home_possession { 8 } else { 1 };
            matrix::draw_pixels(canvas, &pixels_book.football, (1, football_height));
        }
        
        
    } else {
        draw_bottom_info(&common.get_ordinal_text(timezone), (5, 23), &white);
    }
}

impl aws_screen::AWSScreenType for FootballGame {
    fn draw_screen(
        self: &Self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        pixels_book: &matrix::PixelBook,
        timezone: &Tz,
    ) {
        football_draw(&self.common, &self.extra_data, canvas, font_book, pixels_book, timezone);
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
        pixels_book: &matrix::PixelBook,
        timezone: &Tz,
    ) {
        football_draw(&self.common, &self.extra_data, canvas, font_book, pixels_book, timezone);
    }
}
impl game::Sport for CollegeFootballGame {
    fn get_common(self: &Self) -> &game::CommonGameData {
         &self.common
    }
}
