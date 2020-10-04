use crate::common;
use crate::matrix;

use rpi_led_matrix;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub struct AnimationTestScreen {
    sender: mpsc::Sender<common::MatrixCommand>,
    loading_anim: LoadingAnimation,
}

pub struct LoadingAnimation {
    frame: i32,
    last_update: Option<Instant>,
}

impl LoadingAnimation {
    pub fn new() -> LoadingAnimation {
        LoadingAnimation {
            frame: 0,
            last_update: None,
        }
    }
    pub fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas, top_left: (i32, i32)) {
        let (x_offset, y_offset) = top_left;

        let white = common::new_color(255, 255, 255);

        for t in 0..4 {
            if t == self.frame {
                continue;
            }
            canvas.set(x_offset + t, y_offset + 0, &white);
        }
        for r in 1..4 {
            if r + 3 == self.frame {
                continue;
            }
            canvas.set(x_offset + 3, y_offset + r, &white);
        }
        for b in 1..4 {
            if b + 6 == self.frame {
                continue;
            }
            canvas.set(x_offset + 3 - b, y_offset + 3, &white);
        }
        for l in 1..3 {
            if l + 9 == self.frame {
                continue;
            }
            canvas.set(x_offset + 0, y_offset + 3 - l, &white);
        }
        let now = Instant::now();
        if let Some(last_update) = self.last_update {
            if now.duration_since(last_update) > Duration::from_millis(120) {
                self.frame = (self.frame + 1) % 12;
                self.last_update = Some(now);
            }
        } else {
            self.last_update = Some(now);
        }
    }
}

impl AnimationTestScreen {
    pub fn new(sender: mpsc::Sender<common::MatrixCommand>) -> AnimationTestScreen {
        AnimationTestScreen {
            sender,
            loading_anim: LoadingAnimation::new(),
        }
    }
}

impl matrix::ScreenProvider for AnimationTestScreen {
    fn activate(self: &mut Self) {
        self.sender
            .send(common::MatrixCommand::Display(common::ScreenId::Animation))
            .unwrap();
        self.send_draw_command(None);
    }
    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        self.loading_anim.draw(canvas, (0, 0));
        self.send_draw_command(Some(Duration::from_secs(30)));
    }
    fn update_settings(self: &mut Self, _settings: common::ScoreboardSettingsData) {}

    fn get_sender(self: &Self) -> mpsc::Sender<common::MatrixCommand> {
        self.sender.clone()
    }
    fn get_screen_id(self: &Self) -> common::ScreenId {
        common::ScreenId::Animation
    }
}
