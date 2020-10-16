use rpi_led_matrix;

use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::error::Error;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Deserialize_repr, Serialize_repr, Copy)]
#[repr(u16)]
pub enum ScreenId {
    Hockey = 0,
    Baseball = 1,
    Clock = 50,
    Reboot = 99,
    Refresh = 100,
    Setup = 101,
    Error = 104,
    Animation = 1000,
}

pub enum MatrixCommand {
    SetActiveScreen(ScreenId),
    SetPower {
        from_webserver: bool,
        power: Option<bool>,
    },
    Display(ScreenId),
    UpdateSettings(ScoreboardSettingsData),

    // Setup Commands
    GetSettings(), // Fetch the settings
    Reboot {
        from_webserver: bool,
    }, // Reboot the scoreboard
    Reset {
        from_webserver: bool,
    }, // Reset the scoreboard to factory settings (Long Press)
    GotHotspotConnection(), // User connected to hotspot, waiting on wifi details
    GotWifiDetails {
        ssid: String,
        password: String,
    }, // User sent wifi details
    SyncCommand {
        from_webserver: bool,
        show_sync: Option<bool>,
    }, // Show sync, hide sync, or swap sync
}

pub enum WebserverResponse {
    UpdateSettingsResponse(ScoreboardSettingsData),
    SetPowerResponse(ScoreboardSettingsData),
    SetActiveScreenResponse(ScoreboardSettingsData),
    GetSettingsResponse(ScoreboardSettingsData),
    RebootResponse(Option<ScoreboardSettingsData>),
    ResetResponse(Option<ScoreboardSettingsData>),
    GotHotspotConnectionResponse(Option<ScoreboardSettingsData>),
    GotWifiDetailsResponse(Option<ScoreboardSettingsData>),
    SyncCommandRespones(Option<ScoreboardSettingsData>),
}

pub fn new_color(red: u8, green: u8, blue: u8) -> rpi_led_matrix::LedColor {
    rpi_led_matrix::LedColor { red, green, blue }
}
pub fn color_from_slice(slice: &[u8]) -> rpi_led_matrix::LedColor {
    rpi_led_matrix::LedColor {
        red: slice[0],
        green: slice[1],
        blue: slice[2],
    }
}

pub fn color_from_string(s: &str) -> Result<rpi_led_matrix::LedColor, Box<dyn Error>> {
    let get_value = |slice| u8::from_str_radix(slice, 16);
    let red = get_value(&s[0..2])?;
    let green = get_value(&s[2..4])?;
    let blue = get_value(&s[4..6])?;
    Ok(new_color(red, green, blue))
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
