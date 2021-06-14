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
    pub ball_position: String,
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
    let accent_box_width = if football_data.is_some() && common.is_active_game() { 8 } else { 2 };
    game::draw_scoreboard(canvas, &font, &common, 1, accent_box_width);

    // Draw the current period
    let white = common::new_color(255, 255, 255);
    let black = common::new_color(0, 0, 0);
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
        let down_string_dimensions = font.get_text_dimensions(&football_data.down_string);
        let ball_on_text_dimensions = font.get_text_dimensions(&football_data.ball_position);
        draw_bottom_info(&football_data.down_string, (right_indent- down_string_dimensions.width, top_row_height), &white);
        draw_bottom_info(&football_data.ball_position, (right_indent- ball_on_text_dimensions.width, bottom_row_height), &white);

        // Draw possession
        if common.is_active_game() {
            let (football_height, football_color, background_color)  = if football_data.home_possession { 
                (8, common.home_team.primary_color, common.home_team.secondary_color) } else { (1, common.away_team.primary_color, common.away_team.secondary_color) };
            let football_image = pixels_book.football
                .replace_color(&white, &football_color )
                .replace_color(&black, &background_color);
            matrix::draw_pixels(canvas, &football_image, (1, football_height));
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
