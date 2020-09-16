use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;

use rpi_led_matrix;
use serde::{de::Error, Deserialize, Deserializer};
use serde_json;
use std::collections::HashMap;
use std::sync::mpsc;
use std::time::{Duration, Instant};

static HOCKEY_QUERY: &str = r#"
{
    games {
        common {
            home_team {
                id
                name
                city
                display_name
                abbreviation
                primary_color
                secondary_color
            }
            away_team {
                id
                name
                city
                display_name
                abbreviation
                primary_color
                secondary_color
            }
            away_score
            home_score
            status
            ordinal
            start_time
            id
        }
        away_powerplay
        home_powerplay
        away_players
        home_players
    }
}
"#;

#[derive(Deserialize, Debug)]
pub struct HockeyGame {
    common: game::CommonGameData,
    away_powerplay: bool,
    home_powerplay: bool,
    away_players: u8,
    home_players: u8,
}

impl aws_screen::AWSScreenType for HockeyGame {
    fn get_endpoint() -> &'static str {
        "nhl"
    }

    fn get_query() -> &'static str {
        "blah"
    }

    fn get_screen_id() -> common::ScreenId {
        common::ScreenId::Hockey
    }

    fn draw_screen(
        self: &Self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
    ) {
        println!("Drawing hockey");
        let font = &font_book.font5x8;
        game::draw_scoreboard(canvas, &font, &self.common);

        // Draw the current period
        let white = common::new_color(255, 255, 255);
        let black = common::new_color(0, 0, 0);
        let yellow = common::new_color(255, 255, 0);
        canvas.draw_text(
            &font.led_font,
            &self.common.ordinal,
            5,
            22,
            &white,
            0,
            false,
        );

        // Draw FINAL
        if self.common.status == game::GameStatus::END {
            canvas.draw_text(&font.led_font, "FINAL", 37, 22, &yellow, 0, false);
        } else {
            let mut powerplay = false;
            let mut message: &str;
            if self.away_powerplay {
                powerplay = true;
                message = &self.common.away_team.abbreviation
            }
            if self.home_powerplay {
                powerplay = true;
                message = &self.common.home_team.abbreviation
            }
            if self.away_players > 1
                && self.away_players < 5
                && self.home_players > 1
                && self.home_players < 5
            {
                powerplay = true;
                message = &format!("{}-{}", self.away_players, self.home_players)
            }
            if powerplay {
                let text_dimensions = font.get_text_dimensions("message");
                let (canvas_width, _) = canvas.canvas_size();
                let rightPoint = canvas_width - 4;
                matrix::draw_rectangle(
                    canvas,
                    (rightPoint, 21),
                    (rightPoint + text_dimensions.width + 2, 30),
                    &yellow,
                );
                canvas.draw_text(
                    &font.led_font,
                    "message",
                    rightPoint + 2,
                    22,
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
                "status": "PREGAME",
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
