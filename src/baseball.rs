use crate::aws_screen;
use crate::common;
use crate::game;
use crate::matrix;

use rpi_led_matrix;
use serde::{de::Error, Deserialize, Deserializer};
use serde_json;
use std::collections::HashMap;
use std::sync::mpsc;
use std::time::{Duration, Instant};

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

    fn draw_screen(
        self: &Self,
        canvas: &mut rpi_led_matrix::LedCanvas,
        font_book: &matrix::FontBook,
    ) {
        println!("Drawing baseball");
        let font = &font_book.font4x6;
        game::draw_scoreboard(canvas, &font, &self.common, 1);
    }
}
