use chrono;
use common;
use rpi_led_matrix;
use serde::{de::Error, Deserialize, Deserializer};
use serde_json;
use std::collections::HashMap;
use std::sync::mpsc;
use std::time::Duration;

static HOCKEY_QUERY: &str = r#"
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
"#;

pub struct Hockey<'a> {
    sender: mpsc::Sender<common::MatrixCommand>,
    api_key: &'a str,
    data: Option<Vec<HockeyGame>>,
    last_refresh_time: Option<chrono::NaiveDateTime>,
}

impl<'a> Hockey<'a> {
    pub fn new(sender: mpsc::Sender<common::MatrixCommand>, api_key: &'a str) -> Hockey<'a> {
        Hockey {
            sender,
            api_key,
            data: None,
            last_refresh_time: None,
        }
    }
}
#[derive(Deserialize)]
struct HockeyGame {
    common: game::CommonGameData,
    away_powerplay: bool,
    home_powerplay: bool,
    away_players: u8,
    home_players: u8,
}
impl matrix::ScreenProvider for Hockey<'_> {
    fn activate(self: &Self) {
        let api_key = self.api_key.to_owned();

        let refresh_thread = std::thread::spawn(move || {
            let resp = game::fetch_games("nhl", &HOCKEY_QUERY, &api_key);
            let games: Vec<HockeyGame> =
                serde_json::from_str(&resp.into_string().unwrap()).unwrap();
        });
    }
    fn deactivate(self: &Self) {}

    fn next_draw(self: &Self) -> Duration {
        Duration::from_secs(5)
    }

    fn draw(self: &Self, canvas: rpi_led_matrix::LedCanvas) -> rpi_led_matrix::LedCanvas {
        canvas
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
