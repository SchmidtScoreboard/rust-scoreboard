use crate::common;
use crate::common::ScoreboardSettingsData;
use crate::matrix;
use crate::scheduler;

use rand;
use rand::distributions::Distribution;
use rand_distr::Uniform;
use rpi_led_matrix;
use std::any::Any;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

const GRAVITY: f64 = 70.0;
const PLAYER_START_POSITION: (f64, f64) = (8.0, 16.0);
const FIRST_BARRIER_START: f64 = 32.0;
const SCREEN_SPEED: f64 = 16.0; // pixels/second
const SCREEN_WIDTH: i32 = 64;
const SCREEN_HEIGHT: i32 = 32;
const BARRIER_WIDTH: i32 = 3;
const MOMENTUM_ADD: f64 = -15.0;
const BARRIER_OPENING: i32 = 5;

struct Stats {
    rng: rand::rngs::ThreadRng,
    distribution: Uniform<f64>,
}

impl Stats {
    fn new(low: f64, high: f64) -> Stats {
        Stats {
            rng: rand::thread_rng(),
            distribution: Uniform::new(low, high),
        }
    }

    fn sample_float(self: &mut Self) -> f64 {
        self.distribution.sample(&mut self.rng) as f64
    }

    fn sample_int(self: &mut Self) -> u8 {
        self.sample_float() as u8
    }
}

#[derive(Debug)]
struct Barrier {
    next_distance: u8,
    height: u8,
}

impl Barrier {
    fn generate(distance_stats: &mut Stats, height_stats: &mut Stats) -> Barrier {
        Barrier {
            next_distance: distance_stats.sample_int(),
            height: height_stats.sample_int(),
        }
    }
}
struct Barriers {
    barriers: VecDeque<Barrier>,
    distance_stats: Stats,
    height_stats: Stats,
}

impl Barriers {
    fn new() -> Barriers {
        let mut distance_stats = Stats::new(10.0, 25.0);
        let mut height_stats = Stats::new(5.0, 27.0);
        let barriers = (1..10).map(|_| Barrier::generate(&mut distance_stats, &mut height_stats));
        Barriers {
            barriers: VecDeque::from_iter(barriers),
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
    _settings: Arc<common::ScoreboardSettingsData>,
    fonts: matrix::FontBook,
    assets: matrix::PixelBook,
    player_position: (f64, f64),
    player_vertical_velocity: f64,
    first_barrier_distance: f64,
    barriers: Barriers,
    last_update: Option<Instant>,
    state: FlappyState,
    score: f64,
    player: matrix::Pixels,
}

impl Flappy {
    pub fn new(
        sender: mpsc::Sender<scheduler::DelayedCommand>,
        _settings: Arc<common::ScoreboardSettingsData>,
        fonts: matrix::FontBook,
        assets: matrix::PixelBook,
    ) -> Flappy {
        let player = (&assets.empty_square).replace_color(
            &common::new_color(255, 255, 255),
            &common::new_color(8, 146, 208),
        );
        Flappy {
            sender,
            _settings,
            fonts,
            assets,
            player_position: PLAYER_START_POSITION,
            player_vertical_velocity: MOMENTUM_ADD,
            first_barrier_distance: FIRST_BARRIER_START,
            barriers: Barriers::new(),
            last_update: None,
            state: FlappyState::Ready(),
            score: 0.0,
            player,
        }
    }

    pub fn reset(self: &mut Self) {
        self.state = FlappyState::Playing();
        self.score = 0.0;
        self.player_vertical_velocity = MOMENTUM_ADD;
        self.player_position = PLAYER_START_POSITION;
        self.first_barrier_distance = FIRST_BARRIER_START;
        self.barriers = Barriers::new();
        self.last_update = None;
    }
}

fn check_rectangle_intersection(
    a: &((f64, f64), (f64, f64)),
    b: &((f64, f64), (f64, f64)),
) -> bool {
    let ((ax1, ay1), (ax2, ay2)) = a;
    let ((bx1, by1), (bx2, by2)) = b;
    ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1
}

impl Flappy {
    pub fn touch(self: &mut Self) {
        info!("Flappy received touch!");
        match self.state {
            FlappyState::Ready() => {
                self.reset();
            }
            FlappyState::Playing() => {
                let new_velocity = self.player_vertical_velocity + MOMENTUM_ADD;
                self.player_vertical_velocity = if new_velocity < MOMENTUM_ADD {
                    new_velocity
                } else {
                    MOMENTUM_ADD
                };
                // Fuckin maybe?
            }
            FlappyState::GameOver() => {
                self.reset();
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
            self.score = self.score + barrier_movement / 10.0;

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
            let player_dimensions = self.player.size();
            let player_bounding_box = (
                self.player_position,
                (
                    self.player_position.0 + player_dimensions.width as f64,
                    self.player_position.1 + player_dimensions.height as f64,
                ),
            );
            // Check if touching a barrier
            for barrier in &self.barriers.barriers {
                // casting nightmare incoming
                let barrier_box = (
                    (barrier_pos, 0.0),
                    (
                        barrier_pos + BARRIER_WIDTH as f64,
                        barrier.height as f64 - BARRIER_OPENING as f64,
                    ),
                );

                if check_rectangle_intersection(&player_bounding_box, &barrier_box) {
                    info!("INTERSECTION TOP");
                    return false;
                }

                let barrier_box = (
                    (
                        barrier_pos,
                        (barrier.height as i32 + BARRIER_OPENING) as f64,
                    ),
                    (barrier_pos + BARRIER_WIDTH as f64, SCREEN_HEIGHT as f64),
                );

                if check_rectangle_intersection(&player_bounding_box, &barrier_box) {
                    info!("INTERSECTION BOTTOM");
                    return false;
                }
                barrier_pos = barrier_pos + barrier.next_distance as f64;
            }

            // Check if touching top or bottom
            if self.player_position.1 < 0.0
                || self.player_position.1 + self.player.size().height as f64 > SCREEN_HEIGHT.into()
            {
                return false;
            }
        }
        self.last_update = Some(now);
        true
    }
    fn draw_play_message(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas, baseline: i32) {
        let white = common::new_color(255, 255, 255);
        let font = &self.fonts.font4x6;
        let tap_text = "Tap ";
        let play_button = &self.assets.play_button;
        let play_text = " to play";
        let tap_dimensions = font.get_text_dimensions(&tap_text);
        let play_dimensions = font.get_text_dimensions(&play_text);

        let total_width = tap_dimensions.width + play_button.size().width + play_dimensions.width;

        let start = SCREEN_WIDTH / 2 - total_width / 2;

        canvas.draw_text(&font.led_font, &tap_text, start, baseline, &white, 0, false);
        matrix::draw_pixels(
            canvas,
            play_button,
            (
                start + tap_dimensions.width,
                baseline - play_button.size().height + 1,
            ),
        );
        canvas.draw_text(
            &font.led_font,
            &play_text,
            start + tap_dimensions.width + play_button.size().width,
            baseline,
            &white,
            0,
            false,
        );
    }
}

impl matrix::ScreenProvider for Flappy {
    fn activate(self: &mut Self) {
        info!("Activating Flappy ");
        self.send_draw_command(None);
    }

    fn deactivate(self: &mut Self) {
        info!("Deactiving Flappy");
        self.state = FlappyState::Ready();
    }

    fn update_settings(self: &mut Self, settings: Arc<ScoreboardSettingsData>) {
        self._settings = settings;
    }

    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        let white = common::new_color(255, 255, 255);
        let blue = common::new_color(8, 146, 208);
        match self.state {
            FlappyState::Ready() => {
                let big_font = &self.fonts.font7x13;
                let text = "FLAPPY";
                let dimensions = big_font.get_text_dimensions(&text);
                canvas.draw_text(
                    &big_font.led_font,
                    &text,
                    SCREEN_WIDTH / 2 - (dimensions.width / 2),
                    dimensions.height + 7,
                    &blue,
                    0,
                    false,
                );
                self.draw_play_message(canvas, SCREEN_HEIGHT - 3);
            }
            FlappyState::Playing() => {
                if !self.update_frame() {
                    self.state = FlappyState::GameOver();
                } else {
                    // Draw player
                    let player = &self.player;
                    let (player_x, player_y) = self.player_position;
                    matrix::draw_pixels(canvas, &player, (player_x as i32, player_y as i32));

                    // Draw the barriers
                    let mut barrier_x = self.first_barrier_distance as i32;
                    self.barriers.barriers.iter().for_each(|barrier| {
                        matrix::draw_rectangle(
                            canvas,
                            (barrier_x, 0),
                            (
                                barrier_x + BARRIER_WIDTH,
                                barrier.height as i32 - BARRIER_OPENING,
                            ),
                            &white,
                        );
                        matrix::draw_rectangle(
                            canvas,
                            (barrier_x, barrier.height as i32 + BARRIER_OPENING),
                            (barrier_x + BARRIER_WIDTH, SCREEN_HEIGHT),
                            &white,
                        );
                        barrier_x = barrier_x + (barrier.next_distance as i32);
                    });
                    // Draw the score
                    let font = &self.fonts.font4x6;
                    let text = format!("{}", self.score as u32);
                    let dimensions = font.get_text_dimensions(&text);
                    canvas.draw_text(
                        &font.led_font,
                        &text,
                        SCREEN_WIDTH - dimensions.width - 2,
                        dimensions.height + 2,
                        &blue,
                        0,
                        false,
                    );
                }
            }
            FlappyState::GameOver() => {
                let big_font = &self.fonts.font7x13;
                let text = "GAME OVER";
                let dimensions = big_font.get_text_dimensions(&text);
                canvas.draw_text(
                    &big_font.led_font,
                    &text,
                    SCREEN_WIDTH / 2 - (dimensions.width / 2),
                    dimensions.height + 1,
                    &white,
                    0,
                    false,
                );

                let font = &self.fonts.font5x8;
                let score_text = "Score: ";
                let number_text = format!("{}", self.score as u32);
                let score_dimensions = font.get_text_dimensions(&score_text);
                let number_dimensions = font.get_text_dimensions(&number_text);
                let baseline = SCREEN_HEIGHT / 2 + score_dimensions.height / 2;
                let start =
                    SCREEN_WIDTH / 2 - (score_dimensions.width + number_dimensions.width) / 2;
                canvas.draw_text(
                    &font.led_font,
                    &score_text,
                    start,
                    baseline,
                    &white,
                    0,
                    false,
                );
                canvas.draw_text(
                    &font.led_font,
                    &number_text,
                    start + score_dimensions.width,
                    baseline,
                    &blue,
                    0,
                    false,
                );
                self.draw_play_message(canvas, SCREEN_HEIGHT - 3);
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
