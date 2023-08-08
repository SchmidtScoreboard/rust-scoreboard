// Draw a sport
use crate::baseball::BaseballGame;
use crate::basketball::{BasketballGame, CollegeBasketballGame};
use crate::common;
use crate::football::{CollegeFootballGame, FootballGame};
use crate::golf::Golf;
use crate::hockey::HockeyGame;

use crate::animation;
use crate::game;
use crate::matrix;
use crate::scheduler;
use std::any::Any;
use std::collections::HashSet;
use std::sync::{mpsc, Arc};

use rand::seq::SliceRandom;
use serde::Deserialize;
use std::time::{Duration, Instant};

const DORMANT_REFRESH_TIME: Duration = Duration::from_secs(60 * 5); // 5 minutes
const ACTIVE_REFRESH_TIME: Duration = Duration::from_secs(60); // 1 minute
#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum SportData {
    Hockey(HockeyGame),
    Baseball(BaseballGame),
    CollegeBasketball(CollegeBasketballGame),
    Basketball(BasketballGame),
    CollegeFootball(CollegeFootballGame),
    Football(FootballGame),
    Golf(Golf),
}

impl SportData {
    fn get_inner(&self) -> &(dyn game::Sport) {
        match self {
            SportData::Hockey(hockey) => hockey,
            SportData::Baseball(baseball) => baseball,
            SportData::CollegeBasketball(college_basketball) => college_basketball,
            SportData::Basketball(basketball) => basketball,
            SportData::CollegeFootball(college_football) => college_football,
            SportData::Football(football) => football,
            SportData::Golf(golf) => golf,
        }
    }
}

struct AWSData {
    games: Vec<SportData>,
    filtered_games: Vec<usize>, // indices of important games in the games vec
    data_received_timestamp: Instant,
    last_cycle_timestamp: Option<Instant>,
    active_index: Option<usize>,
}

impl AWSData {
    pub fn new(games: Vec<SportData>) -> AWSData {
        AWSData {
            games,
            filtered_games: Vec::new(),
            data_received_timestamp: Instant::now(),
            last_cycle_timestamp: None,
            active_index: None,
        }
    }

    pub fn update(
        &mut self,
        new_data: AWSData,
        current_leagues: &HashSet<common::ScreenId>,
        favorite_teams: &[common::FavoriteTeam],
    ) {
        self.games = new_data.games;
        self.filter_games(current_leagues, favorite_teams);
        self.data_received_timestamp = new_data.data_received_timestamp;
    }

    pub fn filter_games(
        &mut self,
        current_leagues: &HashSet<common::ScreenId>,
        favorite_teams: &[common::FavoriteTeam],
    ) {
        self.filtered_games = {
            let (priority_games, other_games): (Vec<usize>, Vec<usize>) = self
                .games
                .iter()
                .enumerate()
                .filter(|(_, game)| current_leagues.contains(&game.get_inner().get_screen_id()))
                .map(|(i, _)| i)
                .partition(|i| {
                    favorite_teams.iter().any(|favorite_team| {
                        self.games[*i].get_inner().get_screen_id() == favorite_team.screen_id
                            && self.games[*i]
                                .get_inner()
                                .involves_team(favorite_team.team_id)
                            && self.games[*i].get_inner().should_focus()
                    })
                });

            if !priority_games.is_empty() {
                priority_games
            } else {
                other_games
            }
        };

        info!("Filtered games: {:?}", self.filtered_games);
    }

    pub fn try_rotate(&mut self, rotation_time: Duration) {
        let now = Instant::now();
        self.active_index = match self.filtered_games.len() {
            0 => None,
            games_length => {
                let should_rotate = match self.last_cycle_timestamp {
                    None => true,
                    Some(last_cycle_timestamp) => {
                        now.duration_since(last_cycle_timestamp) > rotation_time
                            || self.active_index.is_none()
                    }
                };
                if should_rotate {
                    self.last_cycle_timestamp = Some(now);
                    let new_index = match self.active_index {
                        Some(index) => Some((index + 1) % games_length),
                        None => Some(0),
                    };
                    info!(
                        "Old index: {:?}, new index: {:?}",
                        self.active_index, new_index
                    );
                    new_index
                } else {
                    self.active_index.map(|index| index % games_length)
                }
            }
        };
    }

    pub fn get_active_game(&self) -> Option<&SportData> {
        self.active_index
            .map(|index| &self.games[self.filtered_games[index]])
    }
}

enum ReceivedData {
    Valid(AWSData, u8),
    Error,
    None,
}
pub struct AWSScreen {
    sender: mpsc::Sender<scheduler::DelayedCommand>,
    current_leagues: HashSet<common::ScreenId>,
    data: ReceivedData,
    settings: Arc<common::ScoreboardSettingsData>,
    data_pipe_receiver: mpsc::Receiver<Result<AWSData, String>>,
    refresh_control_sender: mpsc::Sender<RefreshThreadState>,
    loading_animation: animation::WavesAnimation,
    fonts: matrix::FontBook,
    pixels: matrix::PixelBook,
    flavor_text: Option<String>,
}

enum RefreshThreadState {
    Active,
    Hibernating,
}

impl AWSScreen {
    pub fn new(
        sender: mpsc::Sender<scheduler::DelayedCommand>,
        base_url: String,
        api_key: String,
        settings: Arc<common::ScoreboardSettingsData>,
        fonts: matrix::FontBook,
        pixels: matrix::PixelBook,
        matrix_mode: matrix::MatrixMode,
    ) -> AWSScreen {
        let (data_pipe_sender, data_pipe_receiver) = mpsc::channel();

        let (refresh_control_sender, refresh_control_receiver) = mpsc::channel();

        let _refresh_thread = std::thread::spawn(move || {
            AWSScreen::run_refresh_thread(
                base_url,
                refresh_control_receiver,
                api_key,
                data_pipe_sender,
                matrix_mode,
            )
        });
        AWSScreen {
            sender,
            current_leagues: HashSet::new(),
            data: ReceivedData::None,
            settings,
            data_pipe_receiver,
            refresh_control_sender,
            loading_animation: animation::WavesAnimation::new(64),
            fonts,
            pixels,
            flavor_text: None,
        }
    }

    fn draw_refresh(&mut self, canvas: &mut rpi_led_matrix::LedCanvas) {
        let flavor_text = {
            if let Some(text) = &self.flavor_text {
                text
            } else {
                let texts = match self.current_leagues.len() {
                    0 => panic!("No screens set"),
                    1 => self
                        .current_leagues
                        .iter()
                        .next()
                        .expect("Could not get current league")
                        .get_refresh_texts(),
                    _ => common::ScreenId::Smart.get_refresh_texts(),
                };
                let text = texts
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
    fn draw_error(&self, canvas: &mut rpi_led_matrix::LedCanvas) {
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
    fn draw_no_games(&self, canvas: &mut rpi_led_matrix::LedCanvas) {
        let font = &self.fonts.font4x6;
        let white = common::new_color(255, 255, 255);
        canvas.draw_text(
            &font.led_font,
            "No events today",
            2,
            2 + font.dimensions.height,
            &white,
            0,
            false,
        );
    }
    fn process(&mut self) {
        if let Ok(data_or_error) = self.data_pipe_receiver.try_recv() {
            match data_or_error {
                Ok(mut new_data) => match &mut self.data {
                    ReceivedData::Valid(current_data, error_count) => {
                        current_data.update(
                            new_data,
                            &self.current_leagues,
                            &self.settings.favorite_teams,
                        );
                        current_data.try_rotate(self.settings.rotation_time);
                        *error_count = 0;
                    }
                    _ => {
                        new_data.filter_games(&self.current_leagues, &self.settings.favorite_teams);
                        self.data = ReceivedData::Valid(new_data, 0);
                    }
                },
                Err(e) => {
                    info!("Received error: {}", e);
                    match &mut self.data {
                        ReceivedData::Valid(_, error_count) => {
                            *error_count += 1;
                            info!("Error count is {}", *error_count);
                            if *error_count > 5 {
                                self.data = ReceivedData::Error
                            }
                        }
                        _ => self.data = ReceivedData::Error,
                    }
                }
            }
            self.flavor_text = None; // Clear the flavor text
        }

        // if we need to change the displayed image, do that now
        if let ReceivedData::Valid(current_data, _error_count) = &mut self.data {
            current_data.try_rotate(self.settings.rotation_time);
        }
    }

    fn run_refresh_thread(
        base_url: String,
        refresh_control_receiver: mpsc::Receiver<RefreshThreadState>,
        api_key: String,
        data_sender: mpsc::Sender<Result<AWSData, String>>,
        matrix_mode: matrix::MatrixMode,
    ) {
        let mut wait_time = DORMANT_REFRESH_TIME;
        let mut skip_flag = false;
        loop {
            if !skip_flag {
                match matrix_mode {
                    matrix::MatrixMode::Production => {
                        info!("Fetching games from {}", &base_url);
                        let resp = game::fetch_games(&base_url, "all_v4", &api_key);
                        if resp.error() {
                            error!("There was an error fetching games for endpoint",);
                            data_sender.send(Err("Network Error".to_owned())).unwrap();
                        } else {
                            info!("{:#?}", resp);
                            match resp.into_string() {
                                Ok(resp_string) => {
                                    process_data_string(&resp_string, &data_sender);
                                }
                                Err(e) => {
                                    error!("Failed to convert response into a string {:?}", e);
                                    data_sender
                                        .send(Err("Invalid Response".to_string()))
                                        .unwrap();
                                }
                            }
                        }
                    }
                    matrix::MatrixMode::Demo => {
                        info!("Loading demo games");
                        let data_string = include_str!("../assets/demo/demo_scores.json");
                        process_data_string(data_string, &data_sender);
                    }
                }
            }
            skip_flag = false;
            if let Ok(state) = refresh_control_receiver.recv_timeout(wait_time) {
                match state {
                    RefreshThreadState::Active => {
                        wait_time = ACTIVE_REFRESH_TIME;
                    }
                    RefreshThreadState::Hibernating => {
                        wait_time = DORMANT_REFRESH_TIME;
                        skip_flag = true;
                    }
                }
            }
        }
    }
}

fn process_data_string(resp_string: &str, data_sender: &mpsc::Sender<Result<AWSData, String>>) {
    let result: Result<game::Response<SportData>, _> = serde_json::from_str(resp_string);
    if let Ok(response) = result {
        info!("Successfully parsed response",);
        data_sender
            .send(Ok(AWSData::new(response.data.games)))
            .unwrap();
    } else {
        error!(
            "Failed to parse response {}, reason: {}",
            resp_string,
            result.err().unwrap()
        );
        data_sender.send(Err("Invalid Data".to_owned())).unwrap();
    }
}
impl matrix::ScreenProvider for AWSScreen {
    fn activate(&mut self) {
        info!("Activating screen");
        self.refresh_control_sender
            .send(RefreshThreadState::Active)
            .unwrap();
        self.send_draw_command(None);
    }
    fn deactivate(&mut self) {
        // Puts the refresh thread on hibernate
        info!("Deactivating AWS Screen");
        self.refresh_control_sender
            .send(RefreshThreadState::Hibernating)
            .unwrap();
    }

    fn update_settings(&mut self, settings: Arc<common::ScoreboardSettingsData>) {
        self.settings = settings;
        self.current_leagues = match self.settings.active_screen {
            common::ScreenId::Hockey
            | common::ScreenId::Baseball
            | common::ScreenId::CollegeBasketball
            | common::ScreenId::CollegeFootball
            | common::ScreenId::Basketball
            | common::ScreenId::Football
            | common::ScreenId::Golf => (vec![self.settings.active_screen]).into_iter().collect(),
            _ => (vec![
                common::ScreenId::Hockey,
                common::ScreenId::Baseball,
                common::ScreenId::CollegeBasketball,
                common::ScreenId::Basketball,
                common::ScreenId::CollegeFootball,
                common::ScreenId::Football,
                common::ScreenId::Golf,
            ])
            .into_iter()
            .collect(),
        };
        if let ReceivedData::Valid(data, _error_count) = &mut self.data {
            data.filter_games(&self.current_leagues, &self.settings.favorite_teams);
            data.try_rotate(self.settings.rotation_time);
        }
    }

    fn draw(&mut self, canvas: &mut rpi_led_matrix::LedCanvas) {
        // Check if there is any new data. If there is, copy it in
        self.process();
        let now = Instant::now();
        // Actually draw the data
        match &self.data {
            ReceivedData::Valid(current_data, _error_count) => {
                if now.duration_since(current_data.data_received_timestamp)
                    < Duration::from_secs(60 * 5)
                {
                    match current_data.get_active_game() {
                        Some(active_game) => {
                            active_game.get_inner().draw_screen(
                                canvas,
                                &self.fonts,
                                &self.pixels,
                                &self.settings.timezone,
                            );
                        }
                        None => {
                            self.draw_no_games(canvas);
                        }
                    }
                } else {
                    self.draw_refresh(canvas); // Data is out of date, draw refresh
                }
            }
            ReceivedData::Error => {
                self.draw_error(canvas);
            }
            ReceivedData::None => {
                self.draw_refresh(canvas);
            }
        }

        // Schedule the next draw
        self.send_draw_command(Some(Duration::from_millis(20)));
    }

    fn get_screen_id(&self) -> common::ScreenId {
        common::ScreenId::Smart
    }

    fn get_sender(&self) -> &mpsc::Sender<scheduler::DelayedCommand> {
        &self.sender
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn has_priority(&mut self, _power_mode: &common::AutoPowerMode) -> bool {
        self.process(); // Ensure we fetch any updated games
        match &self.data {
            ReceivedData::Valid(data, _error_count) => {
                data.games
                    .iter()
                    .filter(|game| {
                        self.current_leagues
                            .contains(&game.get_inner().get_screen_id())
                    })
                    .count()
                    > data.filtered_games.len()
            }
            ReceivedData::Error => true,
            _ => false,
        }
    }
}
