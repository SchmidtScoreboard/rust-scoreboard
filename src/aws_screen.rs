use crate::animation;
use crate::common;
use crate::common::ScoreboardSettingsData;
use crate::game;
use crate::matrix;
use crate::scheduler;

use itertools::Itertools;
use rand::seq::SliceRandom;
use rpi_led_matrix;
use serde_json;
use std::any::Any;
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
        pixels_book: &matrix::PixelBook,
        timezone: &str,
    );

    fn get_refresh_texts() -> Vec<&'static str>;

    fn involves_team(self: &Self, team_id: u32) -> bool;

    fn status(self: &Self) -> game::GameStatus;
}

struct AWSData<T: Clone + Ord + AWSScreenType> {
    games: Result<Vec<T>, String>,
    active_index: usize,
    data_received_timestamp: Instant,
    last_cycle_timestamp: Instant,
}

impl<T: Clone + Ord + AWSScreenType> AWSData<T> {
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

    pub fn try_rotate(
        self: &mut Self,
        favorite_teams: &Vec<common::FavoriteTeam>,
        rotation_time: Duration,
    ) {
        let now = Instant::now();
        if let Ok(games) = &self.games {
            if now.duration_since(self.last_cycle_timestamp) > rotation_time {
                let mut priority_games = favorite_teams
                    .iter()
                    .filter(|team| team.screen_id == T::get_screen_id())
                    .map(|team| {
                        games
                            .iter()
                            .enumerate()
                            .filter(move |&(_, g)| {
                                g.involves_team(team.team_id)
                                    && g.status() == game::GameStatus::ACTIVE
                            })
                            .map(|(i, _)| i)
                    })
                    .flatten();
                if let Some(priority_index) = priority_games.next() {
                    self.active_index = priority_index;
                } else if games.len() > 0 {
                    self.active_index = (self.active_index + 1) % games.len();
                }

                self.last_cycle_timestamp = now;
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

pub struct AWSScreen<T: AWSScreenType + Clone + Ord> {
    sender: mpsc::Sender<scheduler::DelayedCommand>,
    favorite_teams: Vec<common::FavoriteTeam>,
    rotation_time: Duration,
    timezone: String,
    data: Option<AWSData<T>>,
    data_pipe_receiver: mpsc::Receiver<AWSData<T>>,
    refresh_control_sender: mpsc::Sender<RefreshThreadState>,
    loading_animation: animation::WavesAnimation,
    fonts: matrix::FontBook,
    pixels: matrix::PixelBook,
    flavor_text: Option<String>,
}

enum RefreshThreadState {
    ACTIVE,
    HIBERNATING,
}

impl<
        T: 'static
            + AWSScreenType
            + std::fmt::Debug
            + serde::de::DeserializeOwned
            + std::marker::Send
            + Ord
            + Clone,
    > AWSScreen<T>
{
    pub fn new(
        sender: mpsc::Sender<scheduler::DelayedCommand>,
        base_url: String,
        rotation_time_secs: u32,
        favorite_teams: Vec<common::FavoriteTeam>,
        api_key: String,
        timezone: String,
        fonts: matrix::FontBook,
        pixels: matrix::PixelBook,
    ) -> AWSScreen<T> {
        let (data_pipe_sender, data_pipe_receiver) = mpsc::channel();

        let (refresh_control_sender, refresh_control_receiver) = mpsc::channel();

        let _refresh_thread = std::thread::spawn(move || {
            AWSScreen::run_refresh_thread(
                base_url,
                refresh_control_receiver,
                api_key,
                data_pipe_sender,
            )
        });
        AWSScreen {
            sender,
            favorite_teams,
            rotation_time: Duration::from_secs(rotation_time_secs.into()),
            timezone,
            data: None,
            data_pipe_receiver,
            refresh_control_sender: refresh_control_sender,
            loading_animation: animation::WavesAnimation::new(64),
            fonts: fonts,
            pixels: pixels,
            flavor_text: None,
        }
    }

    fn draw_refresh(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        let flavor_text = {
            if let Some(text) = &self.flavor_text {
                text
            } else {
                let text = T::get_refresh_texts()
                    .choose(&mut rand::thread_rng())
                    .unwrap_or(&"Refreshing...")
                    .to_string();
                self.flavor_text = Some(text);
                self.flavor_text.as_ref().unwrap()
            }
        };
        matrix::draw_message(
            canvas,
            &self.fonts.font4x6,
            flavor_text,
            &mut self.loading_animation,
        );

        self.loading_animation.draw(canvas);
    }
    fn draw_error(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        let font = &self.fonts.font4x6;
        let red = common::new_color(255, 0, 0);
        canvas.draw_text(
            &font.led_font,
            "Connection Error",
            1,
            1 + font.dimensions.height,
            &red,
            0,
            false,
        );
    }
    fn draw_no_games(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        let font = &self.fonts.font4x6;
        let white = common::new_color(255, 255, 255);
        canvas.draw_text(
            &font.led_font,
            "No games today",
            1,
            1 + font.dimensions.height,
            &white,
            0,
            false,
        );
    }

    fn run_refresh_thread(
        base_url: String,
        refresh_control_receiver: mpsc::Receiver<RefreshThreadState>,
        api_key: String,
        data_sender: mpsc::Sender<AWSData<T>>,
    ) {
        let mut wait_time = Duration::from_secs(60 * 60); // Default to an hour
        loop {
            info!("Fetching games from {}", &base_url);
            let resp = game::fetch_games(&base_url, T::get_endpoint(), T::get_query(), &api_key);
            if resp.error() {
                error!(
                    "There was an error fetching games for endpoint {}",
                    T::get_endpoint()
                );
                data_sender.send(AWSData::error("Network Error")).unwrap();
            }
            if let Ok(resp_string) = resp.into_string() {
                let result: Result<game::Response<T>, _> = serde_json::from_str(&resp_string);
                if let Ok(response) = result {
                    info!(
                        "Successfully parsed response for endpoint {}: {:?}",
                        T::get_endpoint(),
                        &response.data.games
                    );

                    data_sender
                        .send(AWSData::new(
                            response.data.games.into_iter().sorted().collect(),
                        ))
                        .unwrap();
                } else {
                    error!(
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
            if let Ok(state) = refresh_control_receiver.recv_timeout(wait_time) {
                match state {
                    RefreshThreadState::ACTIVE => {
                        wait_time = Duration::from_secs(60);
                    }
                    RefreshThreadState::HIBERNATING => {
                        wait_time = Duration::from_secs(60 * 60);
                        continue; // go wait again, now for about an hour
                    }
                }
            }
        }
    }

    pub fn get_games(self: &Self) -> Option<&Vec<T>> {
        self.data.as_ref().and_then(|d| d.games.as_ref().ok())
    }
}
impl<
        T: 'static
            + AWSScreenType
            + std::fmt::Debug
            + serde::de::DeserializeOwned
            + std::marker::Send
            + Ord
            + Clone,
    > matrix::ScreenProvider for AWSScreen<T>
{
    fn activate(self: &mut Self) {
        info!("Activating screen {}", T::get_endpoint());
        self.refresh_control_sender
            .send(RefreshThreadState::ACTIVE)
            .unwrap();
        self.send_draw_command(None);
    }
    fn deactivate(self: &mut Self) {
        // Puts the refresh thread on hibernate
        info!("Deactivating AWS Screen {:?}", T::get_endpoint());
        self.refresh_control_sender
            .send(RefreshThreadState::HIBERNATING)
            .unwrap();
    }

    fn update_settings(self: &mut Self, settings: ScoreboardSettingsData) {
        self.timezone = settings.timezone.parse().expect("Failed to parse timezone");
        self.favorite_teams = settings.favorite_teams.clone();
        self.rotation_time = Duration::from_secs(settings.rotation_time.into());
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
                current_data.try_rotate(&self.favorite_teams, self.rotation_time);
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
                                &games[current_data.active_index].draw_screen(
                                    canvas,
                                    &self.fonts,
                                    &self.pixels,
                                    &self.timezone,
                                );
                            } else {
                                self.draw_no_games(canvas);
                            }
                        }
                        Err(_message) => {
                            self.draw_error(canvas);
                        }
                    }
                } else {
                    self.draw_refresh(canvas); // Data is out of date, draw refresh
                }
            }
            None => {
                self.draw_refresh(canvas);
            }
        }

        // Schedule the next draw
        self.send_draw_command(Some(Duration::from_millis(20)));
    }

    fn get_screen_id(self: &Self) -> common::ScreenId {
        T::get_screen_id()
    }

    fn get_sender(self: &Self) -> mpsc::Sender<scheduler::DelayedCommand> {
        self.sender.clone()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn has_priority(self: &Self, team_id: u32) -> bool {
        self.get_games()
            .filter(|games| {
                games
                    .iter()
                    .filter(|g| g.status() == game::GameStatus::ACTIVE && g.involves_team(team_id))
                    .next()
                    .is_some()
            })
            .is_some()
    }
}
