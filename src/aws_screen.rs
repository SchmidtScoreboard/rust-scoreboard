use crate::common;
use crate::common::ScoreboardSettingsData;
use crate::game;
use crate::matrix;
use crate::scoreboard_settings;

use rpi_led_matrix;
use serde::de::DeserializeOwned;
use serde::{de::Error, Deserialize, Deserializer};
use serde_json;
use std::collections::HashMap;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub trait AWSScreenType {
    fn get_endpoint() -> &'static str;

    fn get_query() -> &'static str;

    fn get_screen_id() -> common::ScreenId;

    fn draw_screen(
        self: &Self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
    );
}

struct AWSData<T> {
    games: Result<Vec<T>, String>,
    active_index: usize,
    data_received_timestamp: Instant,
    last_cycle_timestamp: Instant,
}

impl<T> AWSData<T> {
    pub fn new(games: Vec<T>) -> AWSData<T> {
        AWSData {
            games: Ok(games),
            active_index: 0,
            data_received_timestamp: Instant::now(),
            last_cycle_timestamp: Instant::now(),
        }
    }

    pub fn update(self: &mut Self, new_data: AWSData<T>) {
        self.games = new_data.games;
        self.data_received_timestamp = new_data.data_received_timestamp;
    }

    pub fn try_rotate(self: &mut Self) {
        let now = Instant::now();
        if let Ok(games) = &self.games {
            if now.duration_since(self.last_cycle_timestamp) > Duration::from_secs(10) {
                // Rotate the active index
                // TODO don't rotate if there is a favorite team set
                if games.len() > 0 {
                    self.active_index = (self.active_index + 1) % games.len();
                }
                self.last_cycle_timestamp = now;
                println!("Updating active index: {}", self.active_index)
            }
        }
    }

    pub fn error(error_message: &str) -> AWSData<T> {
        AWSData {
            games: Err(error_message.to_owned()),
            active_index: 0,
            data_received_timestamp: Instant::now(),
            last_cycle_timestamp: Instant::now(),
        }
    }
}

pub struct AWSScreen<T: AWSScreenType> {
    sender: mpsc::Sender<common::MatrixCommand>,
    api_key: String,
    data: Option<AWSData<T>>,
    data_pipe_sender: mpsc::Sender<AWSData<T>>,
    data_pipe_receiver: mpsc::Receiver<AWSData<T>>,
    refresh_control_sender: Option<mpsc::Sender<()>>,
    fonts: matrix::FontBook,
}

impl<T: AWSScreenType + std::fmt::Debug + serde::de::DeserializeOwned + std::marker::Send>
    AWSScreen<T>
{
    pub fn new(sender: mpsc::Sender<common::MatrixCommand>, api_key: String) -> AWSScreen<T> {
        let (data_pipe_sender, data_pipe_receiver) = mpsc::channel();
        AWSScreen {
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
    fn draw_error(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas, message: &str) {
        println!("Drawing error {}", message);
    }
    fn draw_no_games(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        println!("Drawing no games today");
    }

    fn run_refresh_thread(
        refresh_control_receiver: mpsc::Receiver<()>,
        api_key: String,
        data_sender: mpsc::Sender<AWSData<T>>,
    ) {
        loop {
            if let Ok(_) = refresh_control_receiver.try_recv() {
                break;
            } else {
                let resp = game::fetch_games(T::get_endpoint(), T::get_query(), &api_key);
                if resp.error() {
                    eprintln!(
                        "There was an error fetching games for endpoint {}",
                        T::get_endpoint()
                    );
                    data_sender.send(AWSData::error("Network Error")).unwrap();
                }
                if let Ok(resp_string) = resp.into_string() {
                    let result: Result<game::Response<T>, _> = serde_json::from_str(&resp_string);
                    if let Ok(response) = result {
                        println!("Successfully parsed response: {:?}", &response.data.games);
                        data_sender.send(AWSData::new(response.data.games)).unwrap();
                    } else {
                        eprintln!(
                            "Failed to parse response {}, reason: {}",
                            resp_string,
                            result.err().unwrap()
                        );
                        data_sender.send(AWSData::error("Invalid Data")).unwrap();
                    }
                } else {
                    data_sender
                        .send(AWSData::error("Invalid Response"))
                        .unwrap();
                }

                std::thread::sleep(Duration::from_secs(60));
            }
        }
    }
}
impl<
        T: 'static + AWSScreenType + std::fmt::Debug + serde::de::DeserializeOwned + std::marker::Send,
    > matrix::ScreenProvider for AWSScreen<T>
{
    fn activate(self: &mut Self) {
        let api_key = self.api_key.clone();

        let (refresh_control_sender, refresh_control_receiver) = mpsc::channel();
        self.refresh_control_sender = Some(refresh_control_sender);

        let data_sender = self.data_pipe_sender.clone();
        // TODO call refresh thread run on a thread
        let _refresh_thread = std::thread::spawn(move || {
            AWSScreen::run_refresh_thread(refresh_control_receiver, api_key, data_sender)
        });
        self.sender
            .send(common::MatrixCommand::Display(T::get_screen_id()))
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

    fn update_settings(self: &mut Self, settings: ScoreboardSettingsData) {}

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
                    < Duration::from_secs(120)
                {
                    match &current_data.games {
                        Ok(games) => {
                            if games.len() > 0 {
                                &games[current_data.active_index].draw_screen(canvas, &self.fonts);
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
                .send(common::MatrixCommand::Display(T::get_screen_id()))
                .unwrap();
        });
    }
}
