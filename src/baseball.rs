use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;

use chrono_tz::Tz;
use serde::Deserialize;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(Deserialize, Debug, Clone)]
pub struct BaseballGame {
    pub common: game::CommonGameData,
    // inning: u8,
    is_inning_top: bool,
    balls: u8,
    outs: u8,
    strikes: u8,
    on_first: bool,
    on_second: bool,
    on_third: bool,
}
impl Ord for BaseballGame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.common.cmp(&other.common)
    }
}

impl PartialOrd for BaseballGame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BaseballGame {
    fn eq(&self, other: &Self) -> bool {
        (&self.common,) == (&other.common,)
    }
}
impl Eq for BaseballGame {}

impl game::Sport for BaseballGame {
    fn get_common(&self) -> &game::CommonGameData {
        &self.common
    }
}

fn get_base_asset(on_base: bool, pixels_book: &matrix::PixelBook) -> &common::Pixels {
    match on_base {
        true => &pixels_book.filled_base,
        false => &pixels_book.empty_base,
    }
}

impl aws_screen::AWSScreenType for BaseballGame {
    fn draw_screen(
        &self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        pixels_book: &matrix::PixelBook,
        timezone: &Tz,
    ) {
        let font = &font_book.font4x6;
        game::draw_scoreboard(canvas, font, &self.common, 1, (2, 2));
        let white = common::new_color(255, 255, 255);
        let ordinal_text = &self.common.get_ordinal_text(timezone);
        let ordinal_text_dimensions = font.get_text_dimensions(ordinal_text);
        let (canvas_width, _) = canvas.canvas_size();
        let font = &font_book.font5x8;

        let text_width = ordinal_text_dimensions.width;
        let down_arrow_width = 7;
        let ordinal_width = text_width + down_arrow_width;

        // left edge of on base indicator is 35 pixels from right edge
        let ordinal_x_offset = if ordinal_text_dimensions.width < 16 {
            5
        } else {
            26 - ordinal_width
        };

        canvas.draw_text(
            &font.led_font,
            ordinal_text,
            ordinal_x_offset,
            20 + font.dimensions.height,
            &white,
            0,
            false,
        );

        if self.common.status == game::GameStatus::Active {
            if self.is_inning_top {
                let up_arrow = &pixels_book.small_arrow.flip_vertical();
                matrix::draw_pixels(
                    canvas,
                    up_arrow,
                    (ordinal_text_dimensions.width + ordinal_x_offset + 4, 20),
                );
            } else {
                let down_arrow = &pixels_book.small_arrow;
                matrix::draw_pixels(
                    canvas,
                    down_arrow,
                    (ordinal_text_dimensions.width + ordinal_x_offset + 4, 23),
                );
            }

            let font = &font_book.font4x6;
            let balls_strikes = format!("{}-{}", self.balls, self.strikes);
            let balls_strikes_dimensions = font.get_text_dimensions(&balls_strikes);
            canvas.draw_text(
                &font.led_font,
                &balls_strikes,
                61 - balls_strikes_dimensions.width,
                18 + balls_strikes_dimensions.height,
                &white,
                0,
                false,
            );
            // TODO make the background transparent, not black
            let first_base = get_base_asset(self.on_first, pixels_book);
            let second_base = get_base_asset(self.on_second, pixels_book);
            let third_base = get_base_asset(self.on_third, pixels_book);

            // TODO correct these position values
            let start_x = 29;
            let start_y = 22;
            matrix::draw_pixels(canvas, third_base, (start_x, start_y));
            matrix::draw_pixels(canvas, second_base, (start_x + 5, start_y - 5));
            matrix::draw_pixels(canvas, first_base, (start_x + 10, start_y));

            for i in 0..3 {
                let x = 61 - balls_strikes_dimensions.width + i * 4;
                let y = 19 + balls_strikes_dimensions.height;
                if self.outs as i32 > i {
                    matrix::draw_pixels(canvas, &pixels_book.filled_square, (x, y));
                } else {
                    matrix::draw_pixels(canvas, &pixels_book.empty_square, (x, y));
                }
            }
        } else if self.common.status == game::GameStatus::End {
            let yellow = common::new_color(255, 255, 0);
            let message = "FINAL";
            canvas.draw_text(
                &font.led_font,
                message,
                canvas_width - font.get_text_dimensions(message).width - 3,
                20 + font.dimensions.height,
                &yellow,
                0,
                false,
            );
        }
    }
}
