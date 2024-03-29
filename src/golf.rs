use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;

use chrono_tz::Tz;
use serde::Deserialize;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(Deserialize, Debug, Clone)]
pub struct Player {
    display_name: String,
    // position: u32,
    score: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Golf {
    pub common: game::CommonGameData,
    pub name: String,
    pub players: Vec<Player>,
}

impl Ord for Golf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.common.cmp(&other.common)
    }
}

impl PartialOrd for Golf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Golf {
    fn eq(&self, other: &Self) -> bool {
        (&self.common,) == (&other.common,)
    }
}

impl Eq for Golf {}

fn draw_player(
    player: &Player,
    y_offset: &mut i32,
    canvas: &mut rpi_led_matrix::LedCanvas,
    font: &matrix::Font,
    name_color: &rpi_led_matrix::LedColor,
    score_color: &rpi_led_matrix::LedColor,
) {
    let baseline = *y_offset + font.dimensions.height;
    let score_width = font.get_text_dimensions(&player.score).width;

    canvas.draw_text(
        &font.led_font,
        &player.score,
        64 - score_width,
        baseline,
        score_color,
        0,
        false,
    );
    canvas.draw_text(
        &font.led_font,
        &player.display_name.to_ascii_uppercase(),
        1,
        baseline,
        name_color,
        0,
        false,
    );

    *y_offset = *y_offset + font.dimensions.height + 1;
}
impl game::Sport for Golf {
    fn get_common(&self) -> &game::CommonGameData {
        &self.common
    }

    fn involves_team(&self, _target_team: u32) -> bool {
        true
    }
}

impl aws_screen::AWSScreenType for Golf {
    fn draw_screen(
        &self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        _pixels_book: &matrix::PixelBook,
        timezone: &Tz,
    ) {
        let font = &font_book.font4x6; // Use the smallest font to fit the most info
        let green = common::new_color(52, 162, 35);
        let white = common::new_color(255, 255, 255);
        canvas.draw_text(
            &font.led_font,
            &self.name,
            32 - (font.get_text_dimensions(&self.name).width / 2),
            font.dimensions.height + 1,
            &green,
            0,
            false,
        );

        match self.common.status {
            game::GameStatus::Pregame => {
                // Draw the start time here
                let big_font = &font_book.font5x8;
                let text = format!(
                    "{}",
                    self.common
                        .start_time
                        .with_timezone(timezone)
                        .format("%-I:%M %p")
                );
                let dimensions = big_font.get_text_dimensions(&text);
                canvas.draw_text(
                    &big_font.led_font,
                    &text,
                    32 - dimensions.width / 2,
                    16 + dimensions.height / 2,
                    &white,
                    0,
                    false,
                );
            }
            game::GameStatus::Intermission | game::GameStatus::End | game::GameStatus::Active => {
                let num_players = 4;
                let mut player_offset = font.dimensions.height + 3;
                self.players.iter().take(num_players).for_each(|player| {
                    draw_player(player, &mut player_offset, canvas, font, &white, &green);
                });
            }
        }
    }
}
