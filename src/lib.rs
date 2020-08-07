use rpi_led_matrix;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

trait DataProvider<ImageProvider> {
    fn refresh() -> Vec<ImageProvider>;
}

trait ImageProvider {
    fn get_image(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas);
}

fn new_color(red: u8, green: u8, blue: u8) -> rpi_led_matrix::LedColor {
    rpi_led_matrix::LedColor { red, green, blue }
}

enum GameStatus {
    Pregame,
    Active { ordinal: String },
    Intermission { ordinal: String },
    Final { ordinal: String },
}
#[derive(Serialize, Deserialize)]
struct Team {
    id: u32,
    display_name: String,
    abbreviation: String,
    // primary_color: rpi_led_matrix::LedColor, // Color for background of the scoreboard
    // secondary_color: rpi_led_matrix::LedColor, // Text color and accent color
}

pub struct CommonGameData {
    home_team: Team,
    away_team: Team,
    home_score: u8,
    away_score: u8,
    game_status: GameStatus,
}

// impl CommonGameData {
//     pub fn new()
// }
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

        let v: Value = serde_json::from_str(data).unwrap();
        // let team: Team = serde_json::from_str(data);
    }
}
