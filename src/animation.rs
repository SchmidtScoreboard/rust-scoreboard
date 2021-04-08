use crate::common;
use crate::matrix;
use crate::scheduler;
use rand;
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;

use rpi_led_matrix;
use std::any::Any;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub struct LoadingAnimation {
    frame: i32,
    last_update: Option<Instant>,
    size: i32,
}

impl LoadingAnimation {
    pub fn new(size: i32) -> LoadingAnimation {
        LoadingAnimation {
            frame: 0,
            last_update: None,
            size,
        }
    }
    pub fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas, top_left: (i32, i32)) {
        let (x_offset, y_offset) = top_left;

        let white = common::new_color(255, 255, 255);
        let size = self.size;
        let length = self.size - 1;

        for t in 0..size {
            if t == self.frame {
                continue;
            }
            canvas.set(x_offset + t, y_offset, &white);
        }
        for r in 1..size {
            if r + length == self.frame {
                continue;
            }
            canvas.set(x_offset + length, y_offset + r, &white);
        }
        for b in 1..size {
            if b + length * 2 == self.frame {
                continue;
            }
            canvas.set(x_offset + length - b, y_offset + length, &white);
        }
        for l in 1..length {
            if l + length * 3 == self.frame {
                continue;
            }
            canvas.set(x_offset, y_offset + length - l, &white);
        }
        let now = Instant::now();
        if let Some(last_update) = self.last_update {
            if now.duration_since(last_update) > Duration::from_millis(120) {
                self.frame = (self.frame + 1) % (length * 4);
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
    sender: mpsc::Sender<scheduler::DelayedCommand>,
    loading_anim: LoadingAnimation,
    waves_anim: WavesAnimation,
}

impl AnimationTestScreen {
    pub fn new(sender: mpsc::Sender<scheduler::DelayedCommand>) -> AnimationTestScreen {
        AnimationTestScreen {
            sender,
            loading_anim: LoadingAnimation::new(5),
            waves_anim: WavesAnimation::new(64),
        }
    }
}

impl matrix::ScreenProvider for AnimationTestScreen {
    fn activate(self: &mut Self) {
        self.send_draw_command(None);
    }
    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        self.loading_anim.draw(canvas, (0, 0));
        self.waves_anim.draw(canvas);
        self.send_draw_command(Some(Duration::from_millis(20)));
    }

    fn get_sender(self: &Self) -> mpsc::Sender<scheduler::DelayedCommand> {
        self.sender.clone()
    }
    fn get_screen_id(self: &Self) -> common::ScreenId {
        common::ScreenId::Animation
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
