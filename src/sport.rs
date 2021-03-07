// Draw a sport
use crate::hockey::HockeyGame;
use crate::basketball::BasketballGame;
use crate::baseball::BaseballGame;
use crate::college_basketball::CollegeBasketballGame;
use crate::aws_screen::AWSScreenType;
use crate::common;

use crate::game;
use crate::matrix;
use crate::rpi_led_matrix;
use crate::scheduler;
use crate::animation;
use std::sync::mpsc;
use std::any::Any;
use std::collections::HashSet;

use std::time::{Duration, Instant};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum SportData {
    Hockey(HockeyGame),
    Baseball(BaseballGame),
    CollegeBasketball(CollegeBasketballGame),
    Basketball(BasketballGame),
}

impl SportData {
    fn get_common(self: &Self) -> &game::CommonGameData {
        match self {
            SportData::Hockey(hockey) => &hockey.common,
            SportData::Baseball(baseball) => &baseball.common,
            SportData::CollegeBasketball(college_basketball) => &college_basketball.common,
            SportData::Basketball(basketball) => &basketball.common
        }
    }

    fn get_aws_screen(self: &Self) -> &dyn AWSScreenType {
        match self {
            SportData::Hockey(hockey) => hockey,
            SportData::Baseball(baseball) => baseball,
            SportData::CollegeBasketball(college_basketball) => college_basketball,
            SportData::Basketball(basketball) => basketball,
        }

    }
}



struct AWSData{
    games: Result<Vec<SportData>, String>,
    data_received_timestamp: Instant,
    last_cycle_timestamp: Instant,
    active_index: Option<usize>
}

impl AWSData {
    pub fn new(games: Vec<SportData>) -> AWSData {
        AWSData {
            games: Ok(games),
            data_received_timestamp: Instant::now(),
            last_cycle_timestamp: Instant::now(),
            active_index: None
        }
    }

    pub fn update(self: &mut Self, new_data: AWSData, current_leagues: &HashSet<common::ScreenId>, favorite_teams: &Vec<common::FavoriteTeam>) {
        self.games = new_data.games.map(|games| { 
            let (priority_games, other_games): (Vec<SportData>, Vec<SportData>) = games.into_iter()
            .filter(|game| current_leagues.contains(&game.get_common().sport_id))
            .partition(|game| favorite_teams.into_iter()
                .any(|favorite_team| &game.get_common().sport_id == &favorite_team.screen_id && game.get_common().involves_team(favorite_team.team_id) && game.get_common().is_active_game()));
            if priority_games.len() > 0 {
                priority_games
            } else {
                other_games
            }
        }
        );
        self.data_received_timestamp = new_data.data_received_timestamp;
    }

    pub fn try_rotate(
        self: &mut Self,
        rotation_time: Duration,
    ) {
        let now = Instant::now();
        if let Ok(games) = &self.games {
            if now.duration_since(self.last_cycle_timestamp) > rotation_time {
                self.last_cycle_timestamp = now;
            }
            if games.len() > 0 {
                self.active_index = match self.active_index {
                    Some(index) => Some((index + 1) % games.len()),
                    None => Some(0)
                }
            } else {
                self.active_index = None;
            }
        } else {
            self.active_index = None;
        }

    }

    pub fn error(error_message: &str) -> AWSData {
        AWSData {
            games: Err(error_message.to_owned()),
            data_received_timestamp: Instant::now(),
            last_cycle_timestamp: Instant::now(),
            active_index: None
        }
    }
}
pub struct AWSScreen {
    sender: mpsc::Sender<scheduler::DelayedCommand>,
    favorite_teams: Vec<common::FavoriteTeam>,
    current_leagues: HashSet<common::ScreenId>,
    rotation_time: Duration,
    timezone: String,
    data: Option<AWSData>,
    data_pipe_receiver: mpsc::Receiver<AWSData>,
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

impl
    AWSScreen
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
    ) -> AWSScreen {
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
            current_leagues: HashSet::new(),
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
                "Warming up"
                // let text = T::get_refresh_texts()
                //     .choose(&mut rand::thread_rng())
                //     .unwrap_or(&"Refreshing...")
                //     .to_string();
                // self.flavor_text = Some(text);
                // self.flavor_text.as_ref().unwrap()
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
        data_sender: mpsc::Sender<AWSData>,
    ) {
        let mut wait_time = Duration::from_secs(60 * 60); // Default to an hour
        loop {
            info!("Fetching games from {}", &base_url);
            let resp = game::fetch_games(&base_url, "all", "", &api_key);
            if resp.error() {
                error!(
                    "There was an error fetching games for endpoint",
                   
                );
                data_sender.send(AWSData::error("Network Error")).unwrap();
            }
            if let Ok(resp_string) = resp.into_string() {
                let result: Result<game::Response<SportData>, _> = serde_json::from_str(&resp_string);
                if let Ok(response) = result {
                    info!(
                        "Successfully parsed response: {:?}",
                        &response.data.games
                    );

                    info!("Response {}", &resp_string);

                    data_sender
                        .send(AWSData::new(
                            response.data.games
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

}
impl matrix::ScreenProvider for AWSScreen
{
    fn activate(self: &mut Self) {
        info!("Activating screen");
        self.refresh_control_sender
            .send(RefreshThreadState::ACTIVE)
            .unwrap();
        self.send_draw_command(None);
    }
    fn deactivate(self: &mut Self) {
        // Puts the refresh thread on hibernate
        info!("Deactivating AWS Screen");
        self.refresh_control_sender
            .send(RefreshThreadState::HIBERNATING)
            .unwrap();
    }

    fn update_settings(self: &mut Self, settings: common::ScoreboardSettingsData) {
        self.timezone = settings.timezone.parse().expect("Failed to parse timezone");
        self.favorite_teams = settings.favorite_teams.clone();
        self.rotation_time = Duration::from_secs(settings.rotation_time.into());
        self.current_leagues = match settings.active_screen {
            common::ScreenId::Smart => (vec![common::ScreenId::Hockey, common::ScreenId::Baseball, common::ScreenId::CollegeBasketball, common::ScreenId::Basketball]).into_iter().collect(),
            _ => (vec![settings.active_screen]).into_iter().collect()
        }
    }

    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        // Check if there is any new data. If there is, copy it in
        let now = Instant::now();
        if let Ok(new_data) = self.data_pipe_receiver.try_recv() {
            match &mut self.data {
                Some(current_data) => {
                    current_data.update(new_data, &self.current_leagues, &self.favorite_teams);
                }
                None => {
                    self.data = Some(new_data);
                }
            }
        }

        // if we need to change the displayed image, do that now
        match &mut self.data {
            Some(current_data) => {
                current_data.try_rotate( self.rotation_time);
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
                                match current_data.active_index {
                                    Some(active_index) => {
                                        &games[active_index].get_aws_screen().draw_screen(
                                            canvas,
                                            &self.fonts,
                                            &self.pixels,
                                            &self.timezone,
                                        );
                                    },
                                    None => {
                                        self.draw_no_games(canvas);
                                    }
                                }
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
        common::ScreenId::Smart
    }

    fn get_sender(self: &Self) -> mpsc::Sender<scheduler::DelayedCommand> {
        self.sender.clone()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn has_priority(self: &Self, _team_id: u32) -> bool {
        false
    }
}