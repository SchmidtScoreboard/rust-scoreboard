use crate::common;
use crate::game;
use crate::matrix;

use rpi_led_matrix;
use std::collections::HashMap;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub struct AnimationTestScreen {
    sender: mpsc::Sender<common::MatrixCommand>,
    api_key: String,
    fonts: matrix::FontBook,
    loading_anim: LoadingAnimation,
}

pub struct LoadingAnimation {
    frame: i32,
    last_update: Option<Instant>,
}

impl LoadingAnimation {
    fn new() -> LoadingAnimation {
        LoadingAnimation {
            frame: 0,
            last_update: None,
        }
    }
    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas, top_left: (i32, i32)) {
        let (x_offset, y_offset) = top_left;

        let white = common::new_color(255, 255, 255);
        let black = common::new_color(0, 0, 0);

        for t in 0..4 {
            // println!("t = {}, point is ({}, {}), index is {}", t, t, 0, t);
            if t == self.frame {
                continue;
            }
            canvas.set(t, 0, &white);
        }
        for r in 1..4 {
            // println!("r = {}, point is ({},{}), index is {}", r, 3, r, r+3);
            if r + 3 == self.frame {
                continue;
            }
            canvas.set(3, r, &white);
        }
        for b in 1..4 {
            // println!("b = {}, point is ({},{}), index is {}", b, 3-b, 3, b+6);
            if b + 6 == self.frame {
                continue;
            }
            canvas.set(3 - b, 3, &white);
        }
        for l in 1..3 {
            // println!("l = {}, point is ({}, {}), index is {}", l, 0, 3-l, l+9);
            if l + 9 == self.frame {
                continue;
            }
            canvas.set(0, 3 - l, &white);
        }
        let now = Instant::now();
        if let Some(last_update) = self.last_update {
            let duration = now.duration_since(last_update);
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
    pub fn new(
        sender: mpsc::Sender<common::MatrixCommand>,
        api_key: String,
    ) -> AnimationTestScreen {
        AnimationTestScreen {
            sender,
            api_key,
            fonts: matrix::FontBook::new(),
            loading_anim: LoadingAnimation::new(),
        }
    }
}

impl matrix::ScreenProvider for AnimationTestScreen {
    fn activate(self: &mut Self) {
        self.sender
            .send(common::MatrixCommand::Display(common::ScreenId::Animation))
            .unwrap();
    }
    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        println!("Drawing animation screen");
        self.loading_anim.draw(canvas, (0, 0));
        let sender = self.sender.clone();
        let _next_draw_thread = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(20));
            sender
                .send(common::MatrixCommand::Display(common::ScreenId::Animation))
                .unwrap();
        });
    }
}
