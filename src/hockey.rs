use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;

use rpi_led_matrix;
use serde::Deserialize;

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
        HOCKEY_QUERY
    }

    fn get_screen_id() -> common::ScreenId {
        common::ScreenId::Hockey
    }
    fn get_refresh_texts() -> Vec<&'static str> {
        return vec!["Warming up", "Icing", "Calling Toronto"];
    }

    fn draw_screen(
        self: &Self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        timezone: &str,
    ) {
        let font = &font_book.font5x8;
        game::draw_scoreboard(canvas, &font, &self.common, 2);

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
        if self.common.status == game::GameStatus::END {
            canvas.draw_text(
                &font.led_font,
                "FINAL",
                37,
                23 + font.dimensions.width,
                &yellow,
                0,
                false,
            );
        } else {
            let mut powerplay_message: Option<String> = None;
            if self.away_powerplay {
                powerplay_message = Some(self.common.away_team.abbreviation.clone());
            }
            if self.home_powerplay {
                powerplay_message = Some(self.common.home_team.abbreviation.clone());
            }
            if self.away_players > 1
                && self.away_players < 5
                && self.home_players > 1
                && self.home_players < 5
            {
                powerplay_message = Some(format!("{}-{}", self.away_players, self.home_players));
            }

            if let Some(message) = powerplay_message {
                let text_dimensions = font.get_text_dimensions("message");
                let (canvas_width, _) = canvas.canvas_size();
                let right_point = canvas_width - 4;
                matrix::draw_rectangle(
                    canvas,
                    (right_point, 22),
                    (right_point + text_dimensions.width + 2, 31),
                    &yellow,
                );
                canvas.draw_text(
                    &font.led_font,
                    &message,
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
