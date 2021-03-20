use rpi_led_matrix;
use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::error::Error;
use std::io;
use std::net::Ipv4Addr;
use ureq;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Deserialize_repr, Serialize_repr, Copy)]
#[repr(u16)]
pub enum ScreenId {
    Hockey = 0,
    Baseball = 1,
    CollegeBasketball = 2,
    Basketball = 3,
    Football = 4,
    CollegeFootball = 5,
    Clock = 50,
    Reboot = 99,
    Refresh = 100,
    Setup = 101,
    Error = 104,
    Animation = 1000,
    Message = 1001,
    Smart = 10000,
}

impl ScreenId {
    pub fn get_base_id(self: &Self) -> &ScreenId {
        match self {
            ScreenId::Baseball
            | ScreenId::Basketball
            | ScreenId::Hockey
            | ScreenId::CollegeBasketball
            | ScreenId::CollegeFootball
            | ScreenId::Football => &ScreenId::Smart,
            _ => self,
        }
    }

    pub fn get_refresh_texts(self: &Self) -> Vec<&'static str> {
        let mut texts = vec!["Warming up!"];
        match self {
            ScreenId::Hockey => texts.extend(vec!["Calling Toronto!", "Icing"]),
            ScreenId::Baseball => texts.extend(vec!["Pitching change!", "Batter up!"]),
            ScreenId::CollegeBasketball | ScreenId::Basketball => {
                texts.extend(vec!["Taking a shot!"])
            }
            ScreenId::CollegeFootball | ScreenId::Football => {
                texts.extend(vec!["First down!", "Blue, 42..."])
            }
            _ => {}
        };
        texts
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum CommandSource {
    Webserver(),
    Button(),
    Task(),
}

pub enum MatrixCommand {
    SetActiveScreen(ScreenId),
    SetPower {
        source: CommandSource,
        power: Option<bool>,
    },
    AutoPower(bool),
    Display(ScreenId),
    UpdateSettings(ScoreboardSettingsData),

    // Setup Commands
    GetSettings(), // Fetch the settings
    Reboot(),
    Reset {
        from_webserver: bool,
    }, // Reset the scoreboard to factory settings (Long Press)
    GotHotspotConnection(), // User connected to hotspot, waiting on wifi details
    GotWifiDetails {
        ssid: String,
        password: String,
    }, // User sent wifi details
    FinishedWifiConnection(io::Result<()>), // True if successful, false otherwise
    FinishedReset(io::Result<()>),
    SyncCommand {
        from_webserver: bool,
        show_sync: Option<bool>,
    }, // Show sync, hide sync, or swap sync
}

pub enum WebserverResponse {
    UpdateSettingsResponse(ScoreboardSettingsData),
    SetPowerResponse(ScoreboardSettingsData),
    SetAutoPowerResponse(ScoreboardSettingsData),
    SetActiveScreenResponse(ScoreboardSettingsData),
    GetSettingsResponse(ScoreboardSettingsData),
    RebootResponse(Option<ScoreboardSettingsData>),
    ResetResponse(Option<ScoreboardSettingsData>),
    GotHotspotConnectionResponse(Option<ScoreboardSettingsData>),
    GotWifiDetailsResponse(Option<ScoreboardSettingsData>),
    SyncCommandResponse(Option<ScoreboardSettingsData>),
}

pub enum ShellCommand {
    Reboot {
        settings: Option<ScoreboardSettingsData>, // If there is a scoreboard settings, it's from the webserver and needs to be forwarded
    },
    Reset {
        from_matrix: bool,
        from_webserver: Option<ScoreboardSettingsData>, // If there is a scoreboard settings, it's from the webserver and needs to be forwarded
    },
    SetupWifi {
        ssid: String,
        password: String,
        settings: ScoreboardSettingsData,
    },
    SetHotspot(bool),
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

pub fn is_connected() -> bool {
    let response = ureq::get("http://clients3.google.com/generate_204").call();
    info!("Checking connection, status is {:?}", response.status());
    response.status() == 204
}

pub fn get_ip_address() -> Option<Ipv4Addr> {
    match Command::new("hostname").arg("-I").output() {
        Ok(output) if output.status.success() => {
            let string = std::str::from_utf8(&output.stdout).expect("Failed to parse hostname");
            let mut ips = string.split(" ");
            match ips.next() {
                Some(ip) => Some(ip.parse().unwrap()),
                None => None,
            }
        }
        _ => None,
    }
}

fn get_pair_for_octet(octet: u8) -> String {
    let offset: u8 = 'A' as u8;
    let first = octet / 26;
    let second = octet % 26;
    std::str::from_utf8(&[first + offset, second + offset])
        .unwrap()
        .to_owned()
}

pub fn get_sync_code() -> Option<String> {
    get_ip_address().map(|ip| ip.octets().map(|octet| get_pair_for_octet(octet)).join(""))
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
pub struct FavoriteTeam {
    pub screen_id: ScreenId,
    pub team_id: u32,
}

fn default_rotation_time() -> u32 {
    10
}

fn default_brightness() -> u8 {
    100
}

fn default_auto_power() -> bool {
    false
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ScoreboardSettingsData {
    pub timezone: String,
    pub setup_state: SetupState,
    pub active_screen: ScreenId,
    pub mac_address: String,
    pub name: String,
    pub screens: Vec<ScreenSettings>,
    pub screen_on: bool,
    #[serde(default = "default_auto_power")]
    pub auto_power: bool,
    pub version: u32,

    #[serde(default)]
    pub favorite_teams: Vec<FavoriteTeam>,
    #[serde(default = "default_rotation_time")]
    pub rotation_time: u32,

    #[serde(default = "default_brightness")]
    pub brightness: u8,
}
