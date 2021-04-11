use crate::common;
use crate::common::ScoreboardSettingsData;
use crate::matrix;
use crate::scheduler;

use rpi_led_matrix;
use std::any::Any;
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::iter::FromIterator;
use rand;
use rand::distributions::{Distribution};
use rand_distr::Normal;

const GRAVITY: f64 = -1.0;
const PLAYER_START_POSITION: (f64, f64) = (8.0, 16.0);
const FIRST_BARRIER_START: f64 = 32.0;
const SCREEN_SPEED: f64 = 16.0; // pixels/second
const SCREEN_WIDTH: i32 = 64;
const SCREEN_HEIGHT: i32 = 32;
const BARRIER_WIDTH: i32 = 3;

struct Stats {
    rng: rand::rngs::ThreadRng,
    distribution: Normal<f64>
}
enum BarrierKind {
    TOP,
    BOTTOM
}

impl Stats {
    fn new(mean: f64, stddev: f64) -> Stats {
        Stats {
            rng: rand::thread_rng(),
            distribution: Normal::new(mean, stddev).unwrap(),
        }
    }

    fn sample_float(self: &mut Self) -> f64{
        self.distribution.sample(&mut self.rng) as f64
    }

    fn sample_int(self: &mut Self) -> u8 {
        self.sample_float() as u8
    }

    fn sample_barrier_kind(self: &mut Self) -> BarrierKind {
        match self.sample_int() {
            1 => BarrierKind::TOP,
            0 => BarrierKind::BOTTOM,
            _ => panic!()
        }
    }
}



struct Barrier {
    kind: BarrierKind,
    next_distance: u8,
    height: u8
}

impl Barrier {
    fn generate(kind_stats: &mut Stats, distance_stats: &mut Stats, height_stats: &mut Stats) -> Barrier {
        Barrier {
            kind: BarrierKind::BOTTOM,
            next_distance: distance_stats.sample_int(),
            height: height_stats.sample_int()
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
        let mut kind_stats = Stats::new(1.0, 1.0);
        let mut distance_stats = Stats::new(10.0, 5.0);
        let mut height_stats = Stats::new(12.0, 6.0);
        let barriers = (1..10).map(|_| Barrier::generate(&mut kind_stats, &mut distance_stats, &mut height_stats));
        Barriers {
            barriers: VecDeque::from_iter(barriers),
            kind_stats,
            distance_stats,
            height_stats
        }
    }

    fn get_first_barrier(self: &Self) -> &Barrier {
        self.barriers.front().unwrap()
    }

    fn pop_first_barrier(self: &mut Self) {
        self.barriers.pop_front().unwrap();
        self.barriers.push_back(Barrier::generate(&mut self.kind_stats, &mut self.distance_stats, &mut self.height_stats));
    }

}

enum FlappyState {
    Ready(), // Just opened, have not played a game yet
    Playing(), // In game
    GameOver(u32) // Throw the score in the game over screen
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
    state: FlappyState
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
        }
    }
}

impl Flappy {

    fn update_frame(self: &mut Self) {
        let now = Instant::now();
        if let Some(last_update) = self.last_update {
            let delta = now.duration_since(last_update).as_secs_f64();

            // First, move the barriers
            let barrier_movement = delta * SCREEN_SPEED;
            self.first_barrier_distance = self.first_barrier_distance - barrier_movement;

            // Remove barrier if it has fallen off the left of the screen
            // TODO adjust this value so barriers will be drawn as they go off screen 
            if self.first_barrier_distance < 0.0 {
                let barrier = self.barriers.get_first_barrier();
                self.first_barrier_distance = self.first_barrier_distance + barrier.next_distance as f64;
                self.barriers.pop_first_barrier();
            }

            //Calcualte new velocity and position for Flappy
            let new_velocity = delta * GRAVITY;
            let player_falling = delta * self.player_vertical_velocity + delta * delta * GRAVITY;
            
            // Move flappy
            self.player_vertical_velocity = new_velocity;
            self.player_position = (self.player_position.0, self.player_position.1 + player_falling)

        }
        self.last_update = Some(now);
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
        self.update_frame();
        let white = common::new_color(255, 255, 255);

        // Draw player
        let player = &self.assets.small_arrow;
        let (player_x, player_y) = self.player_position;
        matrix::draw_pixels(canvas, &player, (player_x as i32, player_y as i32));

        // Draw the barriers
        let mut barrier_x = self.first_barrier_distance as i32;
        self.barriers.barriers.iter().for_each(|barrier| { 
            match barrier.kind {
                BarrierKind::TOP => 
                    matrix::draw_rectangle(
                        canvas, 
                        (barrier_x, SCREEN_HEIGHT), 
                        (barrier_x + BARRIER_WIDTH, SCREEN_HEIGHT - (barrier.height as i32)), 
                        &white),
                BarrierKind::BOTTOM => 
                    matrix::draw_rectangle(
                        canvas, 
                        (barrier_x, barrier.height.into()), 
                        (barrier_x + BARRIER_WIDTH, 0), 
                        &white)  
            }
            barrier_x = barrier_x + (barrier.next_distance as i32);
            
        });

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
