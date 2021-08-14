use crate::aws_screen;
use crate::common::{self, led_color_from_string};
use crate::matrix;


use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use chrono_tz::Tz;
use rpi_led_matrix;
use serde::{de::Error, Deserialize, Deserializer};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use ureq;

#[derive(Deserialize)]
pub struct Response<T> {
    pub data: ResponseData<T>,
}

#[derive(Deserialize)]
pub struct ResponseData<T> {
    pub games: Vec<T>,
}

#[derive(Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum GameStatus {
    PREGAME,
    ACTIVE,
    INTERMISSION,
    END,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Team {
    #[serde(deserialize_with = "u32_from_string")]
    pub id: u32,
    pub display_name: String,
    pub abbreviation: String,
    #[serde(deserialize_with = "led_color_from_string")]
    pub primary_color: rpi_led_matrix::LedColor, // Color for background of the scoreboard
    #[serde(deserialize_with = "led_color_from_string")]
    pub secondary_color: rpi_led_matrix::LedColor, // Text color and accent color
}

fn u32_from_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<u32>().map_err(D::Error::custom)
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommonGameData {
    pub home_team: Team,
    pub away_team: Team,
    pub home_score: u8,
    pub away_score: u8,
    pub status: GameStatus,
    pub ordinal: String,
    pub id: usize,
    pub sport_id: common::ScreenId,
    #[serde(deserialize_with = "datetime_from_string")]
    pub start_time: DateTime<Utc>,
}
impl Ord for CommonGameData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_time.cmp(&other.start_time)
    }
}

impl PartialOrd for CommonGameData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CommonGameData {
    fn eq(&self, other: &Self) -> bool {
        self.start_time == other.start_time
    }
}

impl Eq for CommonGameData {}

impl CommonGameData {
    pub fn get_ordinal_text(self: &Self, timezone: &Tz) -> String {
        if self.status == GameStatus::PREGAME {
            format!(
                "{}",
                self.start_time.with_timezone(timezone).format("%-I:%M %p")
            )
        } else {
            self.ordinal.clone()
        }
    }

    pub fn involves_team(self: &Self, team_id: u32) -> bool {
        self.home_team.id == team_id || self.away_team.id == team_id
    }

    pub fn is_active_game(self: &Self) -> bool {
        self.status == GameStatus::ACTIVE || self.status == GameStatus::INTERMISSION
    }

    pub fn should_focus(self: &Self) -> bool {
        let now = Utc::now();
        let diff = now - self.start_time; // Get the duration between now and the start of the game. Positive == game started, Negative game to start
        (self.status == GameStatus::END && diff > Duration::seconds(0) && diff < Duration::hours(4))
            || (self.status == GameStatus::PREGAME && diff > -Duration::minutes(30))
            || self.is_active_game()
    }
}

fn datetime_from_string<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let naive_time = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%SZ")
        .or(NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%MZ"))
        .map_err(D::Error::custom)?;
    Ok(DateTime::<Utc>::from_utc(naive_time, Utc))
}

pub fn fetch_games(base_url: &str, endpoint: &str, api_key: &str) -> ureq::Response {
    let url = format!("{}{}", base_url, endpoint);
    ureq::get(&url).set("X-API-KEY", api_key).call()
}

fn draw_team_box(
    canvas: &mut rpi_led_matrix::LedCanvas,
    font: &matrix::Font,
    team: &Team,
    score: u8,
    y_offset: i32,
    spacing: i32,
    accent_box_width: i32
) -> i32 {
    let (width, _height) = canvas.canvas_size();
    let box_height = font.dimensions.height + 2 * spacing;

    // Draw outer box
    matrix::draw_rectangle(
        canvas,
        (0, y_offset),
        (width, box_height + y_offset),
        &team.primary_color,
    );
    // Draw accent box
    matrix::draw_rectangle(
        canvas,
        (0, y_offset),
        (accent_box_width, box_height + y_offset),
        &team.secondary_color,
    );
    // Draw team name
    canvas.draw_text(
        &font.led_font,
        &team.display_name.to_ascii_uppercase(),
        accent_box_width + 3,
        font.dimensions.height + y_offset + spacing,
        &team.secondary_color,
        0,
        false,
    );
    // Draw score
    let score_message = score.to_string();
    let score_dimensions = font.get_text_dimensions(&score_message);
    canvas.draw_text(
        &font.led_font,
        &score_message,
        width - 3 - score_dimensions.width,
        font.dimensions.height + y_offset + spacing,
        &team.secondary_color,
        0,
        false,
    );
    box_height
}

pub fn draw_scoreboard(
    canvas: &mut rpi_led_matrix::LedCanvas,
    font: &matrix::Font,
    game: &CommonGameData,
    spacing: i32,
    accent_box_widths: (i32, i32)
) {
    // draw away box
    let (away_width, home_width) = accent_box_widths;
    let box_height = draw_team_box(canvas, font, &game.away_team, game.away_score, 0, spacing, away_width);

    // draw home box
    draw_team_box(
        canvas,
        font,
        &game.home_team,
        game.home_score,
        box_height,
        spacing,
        home_width
    );
}

pub trait Sport: aws_screen::AWSScreenType {
    fn get_common(self: &Self) -> &CommonGameData;

    fn involves_team(self: &Self, target_team: u32) -> bool {
        let common = self.get_common();
        common.involves_team(target_team)
    }

    fn should_focus(self: &Self) -> bool {
        let common = self.get_common();
        common.should_focus()
    }

    fn get_screen_id(self: &Self) -> common::ScreenId {
        let common = self.get_common();
        common.sport_id
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team() {
        let data = r#"
        {
            "id": "120",
            "name": "Nationals",
            "city": "Washington",
            "display_name": "Nationals",
            "abbreviation": "WSH",
            "primary_color": "ab0003",
            "secondary_color": "14225a"
        }"#;

        let team: Team = serde_json::from_str(data).unwrap();

        assert_eq!(team.id, 120);
        assert_eq!(team.display_name, "Nationals");
        assert_eq!(team.abbreviation, "WSH");
        assert_eq!(team.primary_color.red, 171);
        assert_eq!(team.primary_color.green, 0);
        assert_eq!(team.primary_color.blue, 3);
        assert_eq!(team.secondary_color.red, 20);
        assert_eq!(team.secondary_color.green, 34);
        assert_eq!(team.secondary_color.blue, 90);
    }

    #[test]
    fn test_invalid_color() {
        let data = r#"
        {
            "id": "120",
            "name": "Nationals",
            "city": "Washington",
            "display_name": "Nationals",
            "abbreviation": "WSH",
            "primary_color": "abzz03",
            "secondary_color": "14225a"
        }"#;

        let team: Result<Team, serde_json::Error> = serde_json::from_str(data);
        assert!(team.is_err());
    }

    #[test]
    fn test_invalid_id() {
        let data = r#"
        {
            "id": 120,
            "name": "Nationals",
            "city": "Washington",
            "display_name": "Nationals",
            "abbreviation": "WSH",
            "primary_color": "abzz03",
            "secondary_color": "14225a"
        }"#;

        let team: Result<Team, serde_json::Error> = serde_json::from_str(data);
        assert!(team.is_err());
    }

    #[test]
    fn test_common_game_data() {
        let data = r#"{
                    "home_team": {
                        "id": "120",
                        "name": "Nationals",
                        "city": "Washington",
                        "display_name": "Nationals",
                        "abbreviation": "WSH",
                        "primary_color": "ab0003",
                        "secondary_color": "14225a"
                    },
                    "away_team": {
                        "id": "110",
                        "name": "Orioles",
                        "city": "Baltimore",
                        "display_name": "Orioles",
                        "abbreviation": "BAL",
                        "primary_color": "df4601",
                        "secondary_color": "27251f"
                    },
                    "away_score": 0,
                    "home_score": 0,
                    "status": "PREGAME",
                    "ordinal": "",
                    "start_time": "2020-08-07T22:05:00Z",
                    "id": "630879"
                }"#;
        let game: CommonGameData = serde_json::from_str(data).unwrap();
        assert_eq!(game.away_score, 0);
        assert_eq!(game.home_score, 0);
        assert_eq!(game.status, GameStatus::PREGAME);
        assert_eq!(game.ordinal, "");
        // let date = game.start_time.with
        // assert_eq!(game.start_time.)
        assert_eq!(game.home_team.display_name, "Nationals");
        assert_eq!(game.away_team.display_name, "Orioles");
    }
}
