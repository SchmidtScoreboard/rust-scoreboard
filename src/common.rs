use rpi_led_matrix;

use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_repr::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Deserialize_repr, Serialize_repr, Copy)]
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
    UpdateSettings(ScoreboardSettingsData),
}

pub fn new_color(red: u8, green: u8, blue: u8) -> rpi_led_matrix::LedColor {
    rpi_led_matrix::LedColor { red, green, blue }
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum SetupState {
    Factory = 0,
    Hotspot = 1,
    WifiConnect = 2,
    Sync = 3,
    Ready = 10,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ScreenSettings {
    rotation_time: u32,
    subtitle: String,
    focus_teams: Vec<u32>,
    id: ScreenId,
    always_rotate: bool,
    name: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ScoreboardSettingsData {
    pub timezone: String,
    pub setup_state: SetupState,
    pub active_screen: ScreenId,
    pub mac_address: String,
    pub screens: Vec<ScreenSettings>,
    pub screen_on: bool,
    pub version: u32,
}
