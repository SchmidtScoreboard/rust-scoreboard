// Draw a sport
use crate::hockey::HockeyGame;
use crate::basketball::BasketballGame;
use crate::baseball::BaseballGame;
use crate::college_basketball::CollegeBasketballGame;
use crate::common;
use crate::game;
use crate::matrix;
use crate::rpi_led_matrix;
use crate::scheduler;
use crate::animation;
use std::sync::mpsc;
use std::any::Any;

use std::time::{Duration, Instant};

enum SportData {
    Hockey(HockeyGame),
    Baseball(BaseballGame),
    CollegeBasketball(CollegeBasketballGame),
    Basketball(BasketballGame),
}


struct AWSData{
    games: Result<Vec<SportData>, String>,
    data_received_timestamp: Instant,
    last_cycle_timestamp: Instant,
}

impl AWSData {
    pub fn new(games: Vec<SportData>) -> AWSData {
        AWSData {
            games: Ok(games),
            data_received_timestamp: Instant::now(),
            last_cycle_timestamp: Instant::now(),
        }
    }

    pub fn update(self: &mut Self, new_data: AWSData) {
        self.games = new_data.games;
        self.data_received_timestamp = new_data.data_received_timestamp;
    }

    pub fn try_rotate(
        self: &mut Self,
        favorite_teams: &Vec<common::FavoriteTeam>,
        rotation_time: Duration,
    ) {
    }

    pub fn error(error_message: &str) -> AWSData {
        AWSData {
            games: Err(error_message.to_owned()),
            data_received_timestamp: Instant::now(),
            last_cycle_timestamp: Instant::now(),
        }
    }
}
pub struct AWSScreen {
    sender: mpsc::Sender<scheduler::DelayedCommand>,
    favorite_teams: Vec<common::FavoriteTeam>,
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
                let result: Result<game::Response<AWSData>, _> = serde_json::from_str(&resp_string);
                if let Ok(response) = result {
                    info!(
                        "Successfully parsed response: {:?}",
                        &response.data.games
                    );

                    info!("Response {}", &resp_string);

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
        common::ScreenId::Smart
    }

    fn get_sender(self: &Self) -> mpsc::Sender<scheduler::DelayedCommand> {
        self.sender.clone()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn has_priority(self: &Self, team_id: u32) -> bool {
        false
    }
}