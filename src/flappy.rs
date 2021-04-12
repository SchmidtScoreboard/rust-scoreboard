use crate::common;
use crate::common::ScoreboardSettingsData;
use crate::matrix;
use crate::scheduler;

use rand;
use rand::distributions::Distribution;
use rand_distr::Normal;
use rpi_led_matrix;
use std::any::Any;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

const GRAVITY: f64 = 3.0;
const PLAYER_START_POSITION: (f64, f64) = (8.0, 16.0);
const FIRST_BARRIER_START: f64 = 32.0;
const SCREEN_SPEED: f64 = 16.0; // pixels/second
const SCREEN_WIDTH: i32 = 64;
const SCREEN_HEIGHT: i32 = 32;
const BARRIER_WIDTH: i32 = 3;
const MOMENTUM_ADD: f64 = 3.0;

struct Stats {
    rng: rand::rngs::ThreadRng,
    distribution: Normal<f64>,
}

#[derive(Debug)]
enum BarrierKind {
    TOP,
    BOTTOM,
}

impl Stats {
    fn new(mean: f64, stddev: f64) -> Stats {
        Stats {
            rng: rand::thread_rng(),
            distribution: Normal::new(mean, stddev).unwrap(),
        }
    }

    fn sample_float(self: &mut Self) -> f64 {
        self.distribution.sample(&mut self.rng) as f64
    }

    fn sample_int(self: &mut Self) -> u8 {
        self.sample_float() as u8
    }

    fn sample_barrier_kind(self: &mut Self) -> BarrierKind {
        match self.sample_int() % 2 {
            1 => BarrierKind::TOP,
            0 => BarrierKind::BOTTOM,
            _ => panic!("Congrats, you broke math."),
        }
    }
}

#[derive(Debug)]
struct Barrier {
    kind: BarrierKind,
    next_distance: u8,
    height: u8,
}

impl Barrier {
    fn generate(
        kind_stats: &mut Stats,
        distance_stats: &mut Stats,
        height_stats: &mut Stats,
    ) -> Barrier {
        Barrier {
            kind: kind_stats.sample_barrier_kind(),
            next_distance: distance_stats.sample_int(),
            height: height_stats.sample_int(),
        }
    }
}
struct Barriers {
    barriers: VecDeque<Barrier>,
    kind_stats: Stats,
    distance_stats: Stats,
    height_stats: Stats,
}

impl Barriers {
    fn new() -> Barriers {
        let mut kind_stats = Stats::new(50.0, 50.0);
        let mut distance_stats = Stats::new(20.0, 5.0);
        let mut height_stats = Stats::new(12.0, 6.0);
        let barriers = (1..10)
            .map(|_| Barrier::generate(&mut kind_stats, &mut distance_stats, &mut height_stats));
        Barriers {
            barriers: VecDeque::from_iter(barriers),
            kind_stats,
            distance_stats,
            height_stats,
        }
    }

    fn get_first_barrier(self: &Self) -> &Barrier {
        self.barriers.front().unwrap()
    }

    fn pop_first_barrier(self: &mut Self) {
        self.barriers.pop_front().unwrap();
        self.barriers.push_back(Barrier::generate(
            &mut self.kind_stats,
            &mut self.distance_stats,
            &mut self.height_stats,
        ));
    }
}

enum FlappyState {
    Ready(),    // Just opened, have not played a game yet
    Playing(),  // In game
    GameOver(), // Throw the score in the game over screen
}

pub struct Flappy {
    sender: mpsc::Sender<scheduler::DelayedCommand>,
    settings: Arc<common::ScoreboardSettingsData>,
    fonts: matrix::FontBook,
    assets: matrix::PixelBook,
    player_position: (f64, f64),
    player_vertical_velocity: f64,
    first_barrier_distance: f64,
    barriers: Barriers,
    last_update: Option<Instant>,
    state: FlappyState,
    score: u32,
}

impl Flappy {
    pub fn new(
        sender: mpsc::Sender<scheduler::DelayedCommand>,
        settings: Arc<common::ScoreboardSettingsData>,
        fonts: matrix::FontBook,
        assets: matrix::PixelBook,
    ) -> Flappy {
        Flappy {
            sender,
            settings,
            fonts,
            assets,
            player_position: PLAYER_START_POSITION,
            player_vertical_velocity: 0.0,
            first_barrier_distance: FIRST_BARRIER_START,
            barriers: Barriers::new(),
            last_update: None,
            state: FlappyState::Ready(),
            score: 0,
        }
    }
}

fn check_rectangle_intersection(a: &((f64, f64), (f64, f64)), b: &((f64, f64), (f64, f64))) -> bool {
    if a.0.0 >= b.1.0 || b.0.0 >= a.1.0 {
        false
    } else if a.0.1 >= b.1.1 || b.0.1 >= a.1.1 {
        false
    } else {
        true
    }
}

impl Flappy {
    pub fn touch(self: &mut Self) {
        info!("Flappy received touch!");
        match self.state {
            FlappyState::Ready() => {
                self.state = FlappyState::Playing();
                self.score = 0;
            }
            FlappyState::Playing() => {
                self.player_vertical_velocity = self.player_vertical_velocity + MOMENTUM_ADD;
                // Fuckin maybe?
            }
            FlappyState::GameOver() => {
                self.state = FlappyState::Playing();
                self.score = 0;
            }
        }
    }


    fn update_frame(self: &mut Self) -> bool {
        let now = Instant::now();
        if let Some(last_update) = self.last_update {
            let delta = now.duration_since(last_update).as_secs_f64();

            // First, move the barriers
            let barrier_movement = delta * SCREEN_SPEED;
            self.first_barrier_distance = self.first_barrier_distance - barrier_movement;

            // Remove barrier if it has fallen off the left of the screen
            if self.first_barrier_distance < -4.0 {
                let barrier = self.barriers.get_first_barrier();
                self.first_barrier_distance =
                    self.first_barrier_distance + barrier.next_distance as f64;
                self.barriers.pop_first_barrier();
            }
            self.score = self.score + barrier_movement as u32;

            //Calcualte new velocity and position for Flappy
            let new_velocity = self.player_vertical_velocity + delta * GRAVITY;
            let player_falling = delta * self.player_vertical_velocity + delta * delta * GRAVITY;
            // Move flappy
            self.player_vertical_velocity = new_velocity;
            self.player_position = (
                self.player_position.0,
                self.player_position.1 + player_falling,
            );

            let mut barrier_pos = self.first_barrier_distance;
            let player_bounding_box = (self.player_position, (self.player_position.0 + 2.0, self.player_position.1 + 2.0));
            // Check if touching a barrier
            for barrier in &self.barriers.barriers {
                if barrier_pos + BARRIER_WIDTH as f64 > self.player_position.0 {
                    break;
                }
                // casting nightmare incoming
                let barrier_box = match barrier.kind {
                    BarrierKind::TOP => ((barrier_pos, 0.0), (barrier_pos + BARRIER_WIDTH as f64, barrier.height as f64)),
                    BarrierKind::BOTTOM => ((barrier_pos, (SCREEN_HEIGHT - (barrier.height as i32)) as f64), (barrier_pos + BARRIER_WIDTH as f64, SCREEN_HEIGHT as f64))
                };

                if check_rectangle_intersection(&player_bounding_box, &barrier_box) {
                    return false;
                }
                barrier_pos = barrier_pos + barrier.next_distance as f64;
            }

            // Check if touching top or bottom
            if self.player_position.1 < 0.0 || self.player_position.1 > SCREEN_HEIGHT.into() {
                return false;
            }

        }
        self.last_update = Some(now);
        true
    }
}

impl matrix::ScreenProvider for Flappy {
    fn activate(self: &mut Self) {
        info!("Activating Flappy ");
        self.send_draw_command(None);
    }

    fn update_settings(self: &mut Self, settings: Arc<ScoreboardSettingsData>) {
        self.settings = settings;
    }

    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        match self.state {
            FlappyState::Ready() | 
            FlappyState::Playing() => {
                if !self.update_frame() {
                    self.state = FlappyState::GameOver();
                } else {
                    let white = common::new_color(255, 255, 255);
                    let blue = common::new_color(0, 0, 255);
                    // Draw player
                    let player = self.assets.small_arrow.replace_color(&white, &blue);
                    let (player_x, player_y) = self.player_position;
                    matrix::draw_pixels(canvas, &player, (player_x as i32, player_y as i32));

                    // Draw the barriers
                    let mut barrier_x = self.first_barrier_distance as i32;
                    self.barriers.barriers.iter().for_each(|barrier| {
                        match barrier.kind {
                            BarrierKind::TOP => matrix::draw_rectangle(
                                canvas,
                                (barrier_x, 0),
                                (barrier_x + BARRIER_WIDTH, barrier.height as i32),
                                &white,
                            ),
                            BarrierKind::BOTTOM => matrix::draw_rectangle(
                                canvas,
                                (barrier_x, SCREEN_HEIGHT - (barrier.height as i32)),
                                (barrier_x + BARRIER_WIDTH, SCREEN_HEIGHT),
                                &white,
                            ),
                        }
                        barrier_x = barrier_x + (barrier.next_distance as i32);
                    });
                }
            },
            FlappyState::GameOver() => {
                let font = &self.fonts.font7x13;
                let text = "GAME OVER";
                let dimensions = font.get_text_dimensions(&text);
                let white = common::new_color(255, 255, 255);
                canvas.draw_text(&font.led_font, &text, SCREEN_WIDTH / 2 - (dimensions.width / 2), SCREEN_HEIGHT / 2 - (dimensions.height / 2), &white, 0, false);
            }

        }

        self.send_draw_command(Some(Duration::from_millis(20)));
    }

    fn get_sender(self: &Self) -> &mpsc::Sender<scheduler::DelayedCommand> {
        &self.sender
    }

    fn get_screen_id(self: &Self) -> common::ScreenId {
        common::ScreenId::Flappy
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
