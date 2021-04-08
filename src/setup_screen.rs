use crate::animation;
use crate::common;
use crate::matrix;
use crate::scheduler;

use rpi_led_matrix;
use std::any::Any;
use std::sync::{mpsc, Arc};
use std::time::Duration;

enum WifiScreenSubState {
    WaitingForConnection(),
    AttemptingConnection(),
    ConnectionFailed(),
}

pub struct SetupScreen {
    sender: mpsc::Sender<scheduler::DelayedCommand>,
    loading_anim: animation::LoadingAnimation,
    wave_anim: animation::WavesAnimation,
    state: common::SetupState,
    fonts: matrix::FontBook,
    pixels: matrix::PixelBook,
    wifi_state: WifiScreenSubState,
    sync_code: Option<String>,
}

impl SetupScreen {
    pub fn new(
        sender: mpsc::Sender<scheduler::DelayedCommand>,
        state: common::SetupState,
        fonts: matrix::FontBook,
        pixels: matrix::PixelBook,
    ) -> SetupScreen {
        SetupScreen {
            sender,
            loading_anim: animation::LoadingAnimation::new(5),
            wave_anim: animation::WavesAnimation::new(64),
            state,
            fonts,
            pixels,
            wifi_state: WifiScreenSubState::WaitingForConnection(),
            sync_code: common::get_sync_code(),
        }
    }

    pub fn attempting_connection(self: &mut Self) {
        self.wifi_state = WifiScreenSubState::AttemptingConnection();
    }
    pub fn failed_connection(self: &mut Self) {
        self.wifi_state = WifiScreenSubState::ConnectionFailed();
    }

    pub fn set_sync_code(self: &mut Self, code: Option<String>) {
        self.sync_code = code;
    }
}

impl matrix::ScreenProvider for SetupScreen {
    fn activate(self: &mut Self) {
        self.send_draw_command(None);
    }
    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        match self.state {
            common::SetupState::Hotspot => {
                let (canvas_width, canvas_height) = canvas.canvas_size();
                let phone_frame_size = &self.pixels.phone_frame.size();
                let wifi_size = &self.pixels.wifi.size();
                matrix::draw_pixels(
                    canvas,
                    &self.pixels.phone_frame,
                    (2, (canvas_height / 2) - (phone_frame_size.height / 2)),
                );
                matrix::draw_pixels(
                    canvas,
                    &self.pixels.wifi,
                    (5, (canvas_height / 2) - (wifi_size.height / 2)),
                );
                let white = common::new_color(255, 255, 255);
                let font = &self.fonts.font4x6;

                matrix::draw_lines(
                    canvas,
                    &vec!["Connect to", "wifi:", "SSB42"],
                    phone_frame_size.width + 4,
                    font,
                    &white,
                );

                self.loading_anim
                    .draw(canvas, (canvas_width - 5, canvas_height - 5));
            }
            common::SetupState::WifiConnect => {
                let (_canvas_width, canvas_height) = canvas.canvas_size();
                let phone_frame_size = &self.pixels.phone_frame.size();
                matrix::draw_pixels(
                    canvas,
                    &self.pixels.phone_frame,
                    (2, (canvas_height / 2) - (phone_frame_size.height / 2)),
                );
                let white = common::new_color(255, 255, 255);
                let font = &self.fonts.font4x6;
                let lines: Vec<&'static str>;
                match self.wifi_state {
                    WifiScreenSubState::WaitingForConnection() => {
                        let check_size = &self.pixels.green_check.size();
                        matrix::draw_pixels(
                            canvas,
                            &self.pixels.green_check,
                            (5, (canvas_height / 2) - (check_size.height / 2)),
                        );
                        lines = vec!["Connected!", "Send your", "wifi info"];
                    }
                    WifiScreenSubState::AttemptingConnection() => {
                        self.loading_anim.draw(canvas, (7, (canvas_height / 2) - 2));
                        lines = vec!["Connecting", "to wifi"];
                    }
                    WifiScreenSubState::ConnectionFailed() => {
                        let x_size = &self.pixels.red_x.size();
                        matrix::draw_pixels(
                            canvas,
                            &self.pixels.red_x,
                            (6, (canvas_height / 2) - (x_size.height / 2)),
                        );
                        lines = vec!["Failed to", "connect,", "try again"]
                    }
                }
                matrix::draw_lines(canvas, &lines, phone_frame_size.width + 4, font, &white);
            }
            common::SetupState::Sync => {
                let offset: i32 = 9;
                let spacing: i32 = 10;
                let help_font = &self.fonts.font4x6;
                let sync_code_font = &self.fonts.font7x13;
                let white = common::new_color(255, 255, 255);

                if let Some(code) = &self.sync_code {
                    let help_text = "Sync Code:";
                    matrix::draw_text_centered_horizontally(
                        canvas, help_text, offset, help_font, &white,
                    );
                    matrix::draw_text_centered_horizontally(
                        canvas,
                        code,
                        offset + spacing,
                        sync_code_font,
                        &white,
                    );
                } else {
                    let help_text = "Error:";
                    let message = "Failed to connect";
                    matrix::draw_text_centered_horizontally(
                        canvas, help_text, offset, help_font, &white,
                    );
                    matrix::draw_text_centered_horizontally(
                        canvas,
                        message,
                        offset + spacing,
                        sync_code_font,
                        &white,
                    );
                }

                self.wave_anim.draw(canvas);
            }
            common::SetupState::Ready => {
                error!("Should not display setup screen while ready");
            }
            common::SetupState::Factory => error!("Do not display factory state"),
        }
        self.send_draw_command(Some(Duration::from_millis(16)));
    }
    fn update_settings(self: &mut Self, settings: Arc<common::ScoreboardSettingsData>) {
        self.state = settings.setup_state;
    }
    fn get_sender(self: &Self) -> mpsc::Sender<scheduler::DelayedCommand> {
        self.sender.clone()
    }
    fn get_screen_id(self: &Self) -> common::ScreenId {
        common::ScreenId::Setup
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
