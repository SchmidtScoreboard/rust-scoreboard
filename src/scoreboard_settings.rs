use crate::common;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_repr::*;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug, Clone)]
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
    id: common::ScreenId,
    always_rotate: bool,
    name: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ScoreboardSettingsData {
    pub timezone: String,
    pub setup_state: SetupState,
    pub active_screen: common::ScreenId,
    pub mac_address: String,
    pub screens: Vec<ScreenSettings>,
    pub screen_on: bool,
    pub version: u32,
}

#[derive(PartialEq, Debug)]
pub struct ScoreboardSettings {
    pub data: ScoreboardSettingsData,
    pub file_path: PathBuf,
}

impl ScoreboardSettings {
    pub fn new(data: ScoreboardSettingsData, file_path: PathBuf) -> ScoreboardSettings {
        ScoreboardSettings { data, file_path }
    }

    pub fn update_settings(
        self: &mut Self,
        new_settings: ScoreboardSettingsData,
    ) -> &ScoreboardSettingsData {
        self.data = new_settings;
        fs::write(
            &self.file_path,
            serde_json::to_string_pretty(&self.data).unwrap(),
        )
        .unwrap();
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let data = r#"{
    "timezone": "US/Central",
    "setup_state": 10,
    "active_screen": 0,
    "mac_address": "b8:27:eb:6b:64:25",
    "screens": [
        {
            "rotation_time": 10,
            "subtitle": "View scores from professional hockey",
            "focus_teams": [],
            "id": 0,
            "always_rotate": true,
            "name": "Hockey "
        },
        {
            "rotation_time": 10,
            "subtitle": "View scores from professional baseball",
            "focus_teams": [],
            "id": 1,
            "always_rotate": false,
            "name": "Baseball"
        },
        {
            "rotation_time": 10,
            "subtitle": "Show the current time",
            "focus_teams": [],
            "id": 50,
            "always_rotate": false,
            "name": "Clock"
        }
    ],
    "screen_on": true,
    "version": 1

        }"#;

        let settings: ScoreboardSettingsData = serde_json::from_str(data).unwrap();
    }
}
