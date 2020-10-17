use crate::common;
use crate::matrix;
use rand;
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;

use rpi_led_matrix;
use std::sync::mpsc;
use std::time::{Duration, Instant};

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

pub struct WavesAnimation {
    last_update: Option<Instant>,
    columns: Vec<(i32, Instant, Duration)>,
}
fn get_random_duration(mean: f32, stddev: f32) -> Duration {
    let mut rng = rand::thread_rng(); // TODO keep this as a class constnat
    let distribution = Normal::new(mean, stddev).unwrap();
    Duration::from_millis(distribution.sample(&mut rng) as u64)
}

impl WavesAnimation {
    pub fn new(canvas_width: usize) -> WavesAnimation {
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0..5);
        let columns: Vec<(i32, Instant, Duration)> = (0..canvas_width)
            .map(|_| {
                (
                    range.sample(&mut rng),
                    Instant::now(),
                    get_random_duration(120.0, 50.0),
                )
            })
            .collect();
        WavesAnimation {
            last_update: None,
            columns,
        }
    }

    pub fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        let color = common::new_color(255, 255, 255);
        let (_width, height) = canvas.canvas_size();

        let now = Instant::now();
        if let Some(last_update) = self.last_update {
            if now.duration_since(last_update) > Duration::from_millis(20) {
                let mut rng = rand::thread_rng();
                // Update the columns
                self.columns = self
                    .columns
                    .iter()
                    .map(|(height, column_last_update, next_update)| {
                        if now.duration_since(*column_last_update) > *next_update {
                            let distribution = Normal::new(3.5 - *height as f32, 0.5).unwrap();
                            let modified =
                                *height as f32 + distribution.sample(&mut rng).max(-1.0).min(1.0);
                            let final_height = modified.max(0.0) as i32;
                            (
                                final_height,
                                Instant::now(),
                                get_random_duration(250.0, 50.0),
                            )
                        } else {
                            (*height, *column_last_update, *next_update)
                        }
                    })
                    .collect();
                self.last_update = Some(now);
            }
        } else {
            self.last_update = Some(now);
        }

        // Now, actually draw the columns
        for (x, (column_height, _, _)) in self.columns.iter().enumerate() {
            for y in 0..*column_height {
                canvas.set(x as i32, height - y as i32, &color);
            }
        }
    }
}

pub struct AnimationTestScreen {
    sender: mpsc::Sender<common::MatrixCommand>,
    loading_anim: LoadingAnimation,
    waves_anim: WavesAnimation,
}

impl AnimationTestScreen {
    pub fn new(sender: mpsc::Sender<common::MatrixCommand>) -> AnimationTestScreen {
        AnimationTestScreen {
            sender,
            loading_anim: LoadingAnimation::new(),
            waves_anim: WavesAnimation::new(64),
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
        self.waves_anim.draw(canvas);
        self.send_draw_command(Some(Duration::from_millis(20)));
    }
    fn update_settings(self: &mut Self, _settings: common::ScoreboardSettingsData) {}

    fn get_sender(self: &Self) -> mpsc::Sender<common::MatrixCommand> {
        self.sender.clone()
    }
    fn get_screen_id(self: &Self) -> common::ScreenId {
        common::ScreenId::Animation
    }
}
