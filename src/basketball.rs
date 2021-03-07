use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;

use rpi_led_matrix;
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

impl aws_screen::AWSScreenType for BasketballGame {
    fn get_refresh_texts(self: &Self) -> Vec<&'static str> {
        return vec!["Warming up", "Alley oop", "Taking a shot"];
    }
    fn involves_team(self: &Self, team_id: u32) -> bool {
        return self.common.home_team.id == team_id || self.common.away_team.id == team_id;
    }

    fn status(self: &Self) -> game::GameStatus {
        return self.common.status;
    }

    fn draw_screen(
        self: &Self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        _pixels_book: &matrix::PixelBook,
        timezone: &str,
    ) {
        let font = &font_book.font4x6;
        game::draw_scoreboard(canvas, &font, &self.common, 2);

        // Draw the current period
        let white = common::new_color(255, 255, 255);
        let yellow = common::new_color(255, 255, 0);

        canvas.draw_text(
            &font.led_font,
            &self.common.get_ordinal_text(timezone),
            5,
            23 + font.dimensions.height,
            &white,
            0,
            false,
        );

        // Draw FINAL
        if self.common.status == game::GameStatus::END {
            canvas.draw_text(
                &font.led_font,
                "FINAL",
                34 + font.dimensions.width,
                23 + font.dimensions.height,
                &yellow,
                0,
                false,
            );
        }
    }
}
