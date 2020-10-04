use crate::animation;
use crate::common;
use crate::game;
use crate::matrix;

use rpi_led_matrix;
use std::convert::TryInto;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub struct SetupScreen {
    sender: mpsc::Sender<common::MatrixCommand>,
    loading_anim: animation::LoadingAnimation,
    state: common::SetupState,
    fonts: matrix::FontBook,
    pixels: matrix::PixelBook,
}

impl SetupScreen {
    pub fn new(
        sender: mpsc::Sender<common::MatrixCommand>,
        state: common::SetupState,
        fonts: matrix::FontBook,
        pixels: matrix::PixelBook,
    ) -> SetupScreen {
        SetupScreen {
            sender,
            loading_anim: animation::LoadingAnimation::new(),
            state,
            fonts,
            pixels,
        }
    }
}

impl matrix::ScreenProvider for SetupScreen {
    fn activate(self: &mut Self) {
        self.send_draw_command(None);
    }
    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        match self.state {
            common::SetupState::Hotspot => {
                debug!("Drawing hotspot");

                let (_canvas_width, canvas_height) = canvas.canvas_size();
                let phone_frame_size = &self.pixels.phone_frame.size();
                let wifi_size = &self.pixels.wifi.size();
                matrix::draw_pixels(
                    canvas,
                    &self.pixels.phone_frame,
                    (4, (canvas_height / 2) - (phone_frame_size.height / 2)),
                );
                matrix::draw_pixels(
                    canvas,
                    &self.pixels.wifi,
                    (5, (canvas_height / 2) - (wifi_size.height / 2)),
                );
                let white = common::new_color(255, 255, 255);
                let font = &self.fonts.font4x6;

                for (i, text) in vec!["Connect to", "wifi:", "Scoreboard42"]
                    .iter()
                    .enumerate()
                {
                    let index: i32 = i.try_into().unwrap();
                    canvas.draw_text(
                        &font.led_font,
                        &text,
                        phone_frame_size.width + 4,
                        8 + (index * &font.dimensions.height),
                        &white,
                        0,
                        false,
                    );
                }
            }
            common::SetupState::WifiConnect => {
                debug!("Drawing wifi screen");
            }
            common::SetupState::Sync => {
                debug!("Drawing wifi screen");
            }
            common::SetupState::Ready => {
                error!("Should not display setup screen while ready");
            }
            common::SetupState::Factory => error!("Do not display factory state"),
        }
        self.send_draw_command(Some(Duration::from_secs(5)));
    }
    fn update_settings(self: &mut Self, settings: common::ScoreboardSettingsData) {
        self.state = settings.setup_state;
    }
    fn get_sender(self: &Self) -> mpsc::Sender<common::MatrixCommand> {
        self.sender.clone()
    }
    fn get_screen_id(self: &Self) -> common::ScreenId {
        common::ScreenId::Setup
    }
}
