use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;

use rpi_led_matrix;
use serde::Deserialize;

static BASEBALL_QUERY: &str = r#"
{
    games {
        common {
            home_team {
                id
                name
                city
                display_name
                abbreviation
                primary_color
                secondary_color
            }
            away_team {
                id
                name
                city
                display_name
                abbreviation
                primary_color
                secondary_color
            }
            away_score
            home_score
            status
            ordinal
            start_time
            id
        }
        inning
        is_inning_top
        balls
        outs
        strikes
    }
}
"#;

#[derive(Deserialize, Debug)]
pub struct BaseballGame {
    common: game::CommonGameData,
    inning: u8,
    is_inning_top: bool,
    balls: u8,
    outs: u8,
    strikes: u8,
}

impl aws_screen::AWSScreenType for BaseballGame {
    fn get_endpoint() -> &'static str {
        "mlb"
    }

    fn get_query() -> &'static str {
        BASEBALL_QUERY
    }

    fn get_screen_id() -> common::ScreenId {
        common::ScreenId::Baseball
    }

    fn get_refresh_texts() -> Vec<&'static str> {
        return vec![
            "Warming up",
            "Pitching change",
            "Loading bases",
            "Batter up!",
        ];
    }

    fn draw_screen(
        self: &Self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
        pixels_book: &matrix::PixelBook,
        timezone: &str,
    ) {
        let font = &font_book.font4x6;
        game::draw_scoreboard(canvas, &font, &self.common, 1);

        let white = common::new_color(255, 255, 255);
        let ordinal_dimensions = font.get_text_dimensions(&self.common.ordinal);
        canvas.draw_text(
            &font.led_font,
            &self.common.get_ordinal_text(timezone),
            5,
            23 + font.dimensions.height,
            &white,
            0,
            false,
        );

        if self.common.status == game::GameStatus::ACTIVE {
            if self.is_inning_top {
                let up_arrow = &pixels_book.small_arrow.flip_vertical();
                matrix::draw_pixels(canvas, &up_arrow, (ordinal_dimensions.width + 1, 20));
            } else {
                let down_arrow = &pixels_book.small_arrow;
                matrix::draw_pixels(canvas, &down_arrow, (ordinal_dimensions.width + 1, 23));
            }
            let balls_strikes = format!("{}-{}", self.balls, self.strikes);
            let balls_strikes_dimensions = font.get_text_dimensions(&balls_strikes);
            canvas.draw_text(
                &font.led_font,
                &balls_strikes,
                61 - balls_strikes_dimensions.width,
                18,
                &white,
                0,
                false,
            );

            for i in 0..3 {
                let x = 61 - balls_strikes_dimensions.width + i * 4;
                let y = 19 + balls_strikes_dimensions.height + 3;
                if self.outs as i32 > i {
                    matrix::draw_pixels(canvas, &pixels_book.filled_square, (x, y));
                } else {
                    matrix::draw_pixels(canvas, &pixels_book.empty_square, (x, y));
                }
            }
        }
    }
}
