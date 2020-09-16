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
}

impl LoadingAnimation {
    fn new() -> LoadingAnimation {
        LoadingAnimation { frame: 0 }
    }
    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas, top_left: (i32, i32)) {
        println!("Drawing loading animation, frame is {}", self.frame);
        let (x_offset, y_offset) = top_left;

        let white = common::new_color(255, 255, 255);
        let black = common::new_color(0, 0, 0);

        for t in 0..4 {
            if t == self.frame {
                continue;
            }
            canvas.set(t, 0, &white);
        }
        for r in 0..4 {
            if r + 4 == self.frame {
                continue;
            }
            canvas.set(3, r, &white);
        }
        for b in 0..4 {
            if b + 8 == self.frame {
                continue;
            }
            canvas.set(3 - b, 3, &white);
        }
        for l in 0..4 {
            if l + 12 == self.frame {
                continue;
            }
            canvas.set(0, 3 - l, &white);
        }
        self.frame += 1;
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
        println!("Drawing loading animation");
        self.loading_anim.draw(canvas, (0, 0));
        let sender = self.sender.clone();
        let _next_draw_thread = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(1)); // TODO better calculate how long to wait
            sender
                .send(common::MatrixCommand::Display(common::ScreenId::Animation))
                .unwrap();
        });
    }
}
