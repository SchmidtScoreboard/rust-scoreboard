use common;
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
struct HockeyData {
    games: Result<Vec<HockeyGame>, String>,
    timestamp: Instant,
}

impl HockeyData {
    pub fn new(games: Vec<HockeyGame>) -> HockeyData {
        HockeyData {
            games: Ok(games),
            timestamp: Instant::now(),
        }
    }

    pub fn error(error_message: &str) -> HockeyData {
        HockeyData {
            games: Err(error_message.to_owned()),
            timestamp: Instant::now(),
        }
    }
}
pub struct Hockey<'a> {
    sender: mpsc::Sender<common::MatrixCommand>,
    api_key: &'a str,
    data: Option<HockeyData>,
    data_pipe_sender: mpsc::Sender<HockeyData>,
    data_pipe_receiver: mpsc::Receiver<HockeyData>,
    refresh_control_sender: Option<mpsc::Sender<()>>,
    fonts: matrix::FontBook,
}

impl<'a> Hockey<'a> {
    pub fn new(sender: mpsc::Sender<common::MatrixCommand>, api_key: &'a str) -> Hockey<'a> {
        let (data_pipe_sender, data_pipe_receiver) = mpsc::channel();
        Hockey {
            sender,
            api_key,
            data: None,
            data_pipe_sender,
            data_pipe_receiver,
            refresh_control_sender: None,
            fonts: matrix::FontBook::new(),
        }
    }
}
#[derive(Deserialize, Debug)]
struct HockeyGame {
    common: game::CommonGameData,
    away_powerplay: bool,
    home_powerplay: bool,
    away_players: u8,
    home_players: u8,
}

impl matrix::ScreenProvider for Hockey<'_> {
    fn activate(self: &mut Self) {
        let api_key = self.api_key.to_owned();

        let (refresh_control_sender, refresh_control_receiver) = mpsc::channel();
        self.refresh_control_sender = Some(refresh_control_sender);

        let data_sender = self.data_pipe_sender.clone();
        let _refresh_thread = std::thread::spawn(move || loop {
            if let Ok(_) = refresh_control_receiver.try_recv() {
                break;
            } else {
                let resp = game::fetch_games("nhl", &HOCKEY_QUERY, &api_key);
                if resp.error() {
                    eprintln!("There was an error fetching NHL games");
                    data_sender
                        .send(HockeyData::error("Network Error"))
                        .unwrap();
                }
                if let Ok(resp_string) = resp.into_string() {
                    let result: Result<game::Response<HockeyGame>, _> =
                        serde_json::from_str(&resp_string);
                    if let Ok(response) = result {
                        println!(
                            "Successfully parsed hockey response: {:?}",
                            &response.data.games
                        );
                        data_sender
                            .send(HockeyData::new(response.data.games))
                            .unwrap();
                    } else {
                        eprintln!(
                            "Failed to parse response {}, reason: {}",
                            resp_string,
                            result.err().unwrap()
                        );
                        data_sender.send(HockeyData::error("Invalid Data")).unwrap();
                    }
                } else {
                    data_sender
                        .send(HockeyData::error("Invalid Response"))
                        .unwrap();
                }

                std::thread::sleep(Duration::from_secs(60));
            }
        });

        self.sender
            .send(common::MatrixCommand::Display(common::ScreenId::Hockey))
            .unwrap();
    }
    fn deactivate(self: &Self) {
        // Sends a deactivate command to the refresh thread
        self.refresh_control_sender
            .as_ref()
            .unwrap()
            .send(())
            .unwrap();
    }

    fn draw(self: &mut Self, canvas: rpi_led_matrix::LedCanvas) -> rpi_led_matrix::LedCanvas {
        // Check if there is any new data. If there is, copy it in
        let now = Instant::now();
        if let Ok(data) = self.data_pipe_receiver.try_recv() {
            self.data = Some(data);
        }

        if self.data.as_ref().map_or(false, |data| {
            now.duration_since(data.timestamp) < Duration::from_secs(120)
        }) {
            match &self.data.as_ref().unwrap().games {
                Ok(games) => {
                    // Draw Hockey
                }
                Err(message) => {
                    // Display the error message
                }
            }
        } else {
            // draw refresh
        }

        let sender = self.sender.clone();
        let _next_draw_thread = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(5)); // TODO better calculate how long to wait
            sender
                .send(common::MatrixCommand::Display(common::ScreenId::Hockey))
                .unwrap();
        });

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
