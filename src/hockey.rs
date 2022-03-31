use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;

use chrono_tz::Tz;
use serde::Deserialize;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(Deserialize, Debug, Clone)]
pub struct HockeyGame {
    pub common: game::CommonGameData,
    away_powerplay: bool,
    home_powerplay: bool,
    away_players: u8,
    home_players: u8,
}

impl Ord for HockeyGame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.common.cmp(&other.common)
    }
}

impl PartialOrd for HockeyGame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HockeyGame {
    fn eq(&self, other: &Self) -> bool {
        (&self.common,) == (&other.common,)
    }
}

impl Eq for HockeyGame {}

impl game::Sport for HockeyGame{
    fn get_common(&self) -> &game::CommonGameData {
         &self.common
    }
}
impl aws_screen::AWSScreenType for HockeyGame {
    fn draw_screen(
        &self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        _pixels_book: &matrix::PixelBook,
        timezone: &Tz,
    ) {
        let font = &font_book.font5x8;
        game::draw_scoreboard(canvas, font, &self.common, 2, (2, 2));

        // Draw the current period
        let white = common::new_color(255, 255, 255);
        let black = common::new_color(0, 0, 0);
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
        if self.common.status == game::GameStatus::End {
            canvas.draw_text(
                &font.led_font,
                "FINAL",
                32 + font.dimensions.width,
                29,
                &yellow,
                0,
                false,
            );
        } else {
            let stored : String; // Make sure a potentially created reference stays alive
            let powerplay_message: Option<&str> = {
                if self.away_powerplay {
                    Some(&self.common.away_team.abbreviation)
                } else if self.home_powerplay {
                    Some(&self.common.home_team.abbreviation)
                } else if self.away_players > 1
                    && self.away_players < 5
                    && self.home_players > 1
                    && self.home_players < 5
                {
                    // This is gross--store the format string here, and keep a reference in the message.
                    stored = format!("{}-{}", self.away_players, self.home_players);
                    Some(&stored)
                } else {
                    None
                }
            };

            if let Some(message) = powerplay_message {
                let text_dimensions = font.get_text_dimensions(message);
                let (canvas_width, _) = canvas.canvas_size();
                let right_point = canvas_width - text_dimensions.width - 4;
                matrix::draw_rectangle(
                    canvas,
                    (right_point, 21),
                    (right_point + text_dimensions.width + 2, 31),
                    &yellow,
                );
                canvas.draw_text(
                    &font.led_font,
                    message,
                    right_point + 2,
                    23 + font.dimensions.height,
                    &black,
                    0,
                    false,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hockey() {
        let data = r#"{
            "common": {
                "home_team": {
                    "id": "19",
                    "name": "Blues",
                    "city": "St. Louis",
                    "display_name": "Blues",
                    "abbreviation": "STL",
                    "primary_color": "002f87",
                    "secondary_color": "ffb81c"
                },
                "away_team": {
                    "id": "25",
                    "name": "Stars",
                    "city": "Dallas",
                    "display_name": "Stars",
                    "abbreviation": "DAL",
                    "primary_color": "006341",
                    "secondary_color": "a2aaad"
                },
                "away_score": 0,
                "home_score": 0,
                "status": "Pregame",
                "ordinal": "",
                "start_time": "2020-08-09T19:00:00Z",
                "id": "2019030016"
            },
            "away_powerplay": false,
            "home_powerplay": false,
            "away_players": 0,
            "home_players": 0
        }"#;

        let game: HockeyGame = serde_json::from_str(data).unwrap();
        assert_eq!(game.away_powerplay, false);
        assert_eq!(game.home_powerplay, false);
        assert_eq!(game.away_players, 0);
        assert_eq!(game.home_players, 0);
    }
}
