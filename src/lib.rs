use chrono::{DateTime, NaiveDateTime, Utc};
use rpi_led_matrix;
use serde::{de::Error, Deserialize, Deserializer};
use std::collections::HashMap;
use std::sync::mpsc;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum ScreenId {
    Hockey = 0,
    Baseball = 1,
    Clock = 50,
    Reboot = 99,
    Refresh = 100,
    Hotspot = 101,
    WifiDetails = 102,
    Sync = 103,
    Error = 104,
}

pub enum MatrixCommand {
    SetActiveScreen {
        id: ScreenId,
    },
    SetPower {
        on: bool,
    },
    Display {
        id: ScreenId,
        canvas: rpi_led_matrix::LedCanvas,
    },
}

pub struct Matrix<'a> {
    led_matrix: &'a rpi_led_matrix::LedMatrix,
    receiver: mpsc::Receiver<MatrixCommand>,
    active_id: ScreenId,
    screens_map: HashMap<ScreenId, Box<dyn ScreenProvider + 'a>>,
}

impl<'a> Matrix<'a> {
    pub fn new(
        led_matrix: &'a rpi_led_matrix::LedMatrix,
        receiver: mpsc::Receiver<MatrixCommand>,
        map: HashMap<ScreenId, Box<dyn ScreenProvider + 'a>>,
    ) -> Matrix<'a> {
        Matrix {
            led_matrix,
            receiver,
            active_id: ScreenId::Hockey,
            screens_map: map,
        }
    }

    // This is the main loop of the entire code
    // Call this after everything else is set up
    pub fn run(self: Self) {
        let start_screen = self
            .screens_map
            .get(&self.active_id)
            .expect("Could not find screen {self.active_id}");

        // TODO reenable refresh screen
        // let refresh_screen = self
        //     .screens_map
        //     .get(&ScreenId::Refresh)
        //     .expect("Could not find refresh screen");

        // if start_screen.should_show_refresh_on_activate() {
        //     refresh_screen.activate();
        // }

        start_screen.activate();

        // TODO wait on the receiver, complete MatrixCommands
        // while let command = self.receiver.recv() {
        //     let command = command.unwrap(); // Get the actual command
        //                                     // mat
        // }
    }
}

pub trait ScreenProvider {
    // Activate is called by the Display driver
    // Activate sets up whatever refreshing this screen needs
    // Use local reference to the matrix to get a canvas and fill it in on refreshes
    // Use push pipe to push the pipe back to the Display driver
    fn activate(self: &Self);
    fn should_show_refresh_on_activate(self: &Self) -> bool {
        false
    }
}

pub struct Hockey<'a> {
    led_matrix: &'a rpi_led_matrix::LedMatrix,
    sender: mpsc::Sender<MatrixCommand>,
}

impl<'a> Hockey<'a> {
    pub fn new(
        led_matrix: &'a rpi_led_matrix::LedMatrix,
        sender: mpsc::Sender<MatrixCommand>,
    ) -> Hockey {
        Hockey { led_matrix, sender }
    }
}

impl ScreenProvider for Hockey<'_> {
    fn activate(self: &Self) {
        // Fetch scores from the web
        // Setup a timer to get scores every minute
        // Setup a timer to shift screens every Y seconds
        // For now, let's just get a canvas back
        let mut canvas = self.led_matrix.offscreen_canvas();
        let (width, height) = canvas.size();

        canvas.draw_line(0, 0, width, height, &new_color(255, 255, 255));

        self.sender
            .send(MatrixCommand::Display {
                id: ScreenId::Hockey,
                canvas: canvas,
            })
            .expect("Failed to send canvas");
    }
    fn should_show_refresh_on_activate(self: &Self) -> bool {
        // TODO, this should be true if the hockey data is out of date
        false
    }
}

pub trait ImageProvider {
    fn get_image(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas);
}

pub fn new_color(red: u8, green: u8, blue: u8) -> rpi_led_matrix::LedColor {
    rpi_led_matrix::LedColor { red, green, blue }
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum GameStatus {
    PREGAME,
    ACTIVE,
    INTERMISSION,
    FINAL,
}

#[derive(Deserialize)]
pub struct Team {
    #[serde(deserialize_with = "u32_from_string")]
    id: u32,
    display_name: String,
    abbreviation: String,
    #[serde(deserialize_with = "led_color_from_string")]
    primary_color: rpi_led_matrix::LedColor, // Color for background of the scoreboard
    #[serde(deserialize_with = "led_color_from_string")]
    secondary_color: rpi_led_matrix::LedColor, // Text color and accent color
}

fn u32_from_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<u32>().map_err(D::Error::custom)
}

fn led_color_from_string<'de, D>(deserializer: D) -> Result<rpi_led_matrix::LedColor, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let get_value = |slice| u8::from_str_radix(slice, 16).map_err(D::Error::custom);
    let red = get_value(&s[0..2])?;
    let green = get_value(&s[2..4])?;
    let blue = get_value(&s[4..6])?;
    Ok(new_color(red, green, blue))
}

#[derive(Deserialize)]
pub struct CommonGameData {
    home_team: Team,
    away_team: Team,
    home_score: u8,
    away_score: u8,
    status: GameStatus,
    ordinal: String,
    #[serde(deserialize_with = "datetime_from_string")]
    start_time: DateTime<Utc>,
}

fn datetime_from_string<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    println!("Deserialzing datetime {}", s);
    let naive_time =
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%SZ").map_err(D::Error::custom)?;
    Ok(DateTime::<Utc>::from_utc(naive_time, Utc))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
