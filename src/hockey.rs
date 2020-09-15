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

// A struct representing both the current owned data and active index, and new data received
struct HockeyData {
    games: Result<Vec<HockeyGame>, String>,
    active_index: usize,
    data_received_timestamp: Instant,
    last_cycle_timestamp: Instant,
}

impl HockeyData {
    pub fn new(games: Vec<HockeyGame>) -> HockeyData {
        HockeyData {
            games: Ok(games),
            active_index: 0,
            data_received_timestamp: Instant::now(),
            last_cycle_timestamp: Instant::now(),
        }
    }

    pub fn update(self: &mut Self, new_data: HockeyData) {
        self.games = new_data.games;
        self.data_received_timestamp = new_data.data_received_timestamp;
    }

    pub fn try_rotate(self: &mut Self) {
        let now = Instant::now();
        if let Ok(games) = &self.games {
            if now.duration_since(self.last_cycle_timestamp) > Duration::from_secs(10) {
                // Rotate the active index
                // TODO don't rotate if there is a favorite team set
                self.active_index = (self.active_index + 1) % games.len();
                self.last_cycle_timestamp = now;
                println!("Updating hockey active index: {}", self.active_index)
            }
        }
    }

    pub fn error(error_message: &str) -> HockeyData {
        HockeyData {
            games: Err(error_message.to_owned()),
            active_index: 0,
            data_received_timestamp: Instant::now(),
            last_cycle_timestamp: Instant::now(),
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

    fn draw_refresh(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        println!("Drawing refresh");
    }
    fn draw_hockey(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas, hockey_game: &HockeyGame) {
        println!("Drawing hockey");
        game::draw_scoreboard(canvas, &self.fonts.font5x8, &hockey_game.common);

        // Draw the current period
        let white = common::new_color(255, 255, 255);
        let black = common::new_color(0, 0, 0);
        let yellow = common::new_color(255, 255, 0);
        canvas.draw_text(
            &self.fonts.font5x8.led_font,
            &hockey_game.common.ordinal,
            5,
            22,
            &white,
            0,
            false,
        );

        // Draw FINAL
        if hockey_game.common.status == game::GameStatus::END {
            canvas.draw_text(
                &self.fonts.font5x8.led_font,
                "FINAL",
                37,
                22,
                &yellow,
                0,
                false,
            );
        } else {
            let mut powerplay = false;
            let mut message: &str;
            if hockey_game.away_powerplay {
                powerplay = true;
                message = &hockey_game.common.away_team.abbreviation
            }
            if hockey_game.home_powerplay {
                powerplay = true;
                message = &hockey_game.common.home_team.abbreviation
            }
            if hockey_game.away_players > 1
                && hockey_game.away_players < 5
                && hockey_game.home_players > 1
                && hockey_game.home_players < 5
            {
                powerplay = true;
                message = &format!("{}-{}", hockey_game.away_players, hockey_game.home_players)
            }
            if powerplay {
                let text_dimensions = &self.fonts.font5x8.get_text_dimensions("message");
                let (canvas_width, _) = canvas.canvas_size();
                let rightPoint = canvas_width - 4;
                matrix::draw_rectangle(
                    canvas,
                    (rightPoint, 21),
                    (rightPoint + text_dimensions.width + 2, 30),
                    &yellow,
                );
                canvas.draw_text(
                    &self.fonts.font5x8.led_font,
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
    fn draw_error(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas, message: &str) {
        println!("Drawing error {}", message);
    }
    fn draw_no_games(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        println!("Drawing no games today");
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

    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        // Check if there is any new data. If there is, copy it in
        let now = Instant::now();
        if let Ok(new_data) = self.data_pipe_receiver.try_recv() {
            match &mut self.data {
                Some(current_data) => {
                    current_data.update(new_data);
                }
                None => {
                    self.data = Some(new_data);
                }
            }
        }

        // if we need to change the displayed image, do that now
        match &mut self.data {
            Some(current_data) => {
                current_data.try_rotate();
            }
            None => (),
        }

        // Actually draw the data
        match &self.data {
            Some(current_data) => {
                if now.duration_since(current_data.data_received_timestamp)
                    > Duration::from_secs(120)
                {
                    self.draw_refresh(canvas);
                } else {
                    match &current_data.games {
                        Ok(games) => {
                            if games.len() > 0 {
                                self.draw_hockey(canvas, &games[current_data.active_index]);
                            } else {
                                self.draw_no_games(canvas);
                            }
                        }
                        Err(message) => {
                            self.draw_error(canvas, &message);
                        }
                    }
                }
            }
            None => {
                self.draw_refresh(canvas);
            }
        }

        // Schedule the next draw
        let sender = self.sender.clone();
        let _next_draw_thread = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(5)); // TODO better calculate how long to wait
            sender
                .send(common::MatrixCommand::Display(common::ScreenId::Hockey))
                .unwrap();
        });
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
