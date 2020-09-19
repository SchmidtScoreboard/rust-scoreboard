use rpi_led_matrix;

use serde::{de::Error, Deserialize, Serialize, Deserializer};
use serde_repr::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u16)]
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
    Animation = 1000,
}

pub enum MatrixCommand {
    SetActiveScreen(ScreenId),
    SetPower(bool),
    Display(ScreenId),
}

pub fn new_color(red: u8, green: u8, blue: u8) -> rpi_led_matrix::LedColor {
    rpi_led_matrix::LedColor { red, green, blue }
}
