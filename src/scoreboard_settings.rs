use crate::common::{ScoreboardSettingsData, ScreenId, SetupState};
use std::fs;
use std::path::PathBuf;

#[derive(PartialEq, Debug)]
pub struct ScoreboardSettings {
    data: ScoreboardSettingsData,
    pub file_path: PathBuf,
}

impl ScoreboardSettings {
    pub fn new(data: ScoreboardSettingsData, file_path: PathBuf) -> ScoreboardSettings {
        ScoreboardSettings { data, file_path }
    }

    pub fn get_settings(self: &Self) -> &ScoreboardSettingsData {
        &self.data
    }

    pub fn get_settings_clone(self: &Self) -> ScoreboardSettingsData {
        self.data.clone()
    }

    fn write_settings(self: &Self) {
        fs::write(
            &self.file_path,
            serde_json::to_string_pretty(&self.data).unwrap(),
        )
        .unwrap();
    }

    pub fn update_settings(self: &mut Self, new_settings: ScoreboardSettingsData) {
        self.data = new_settings;
        self.write_settings();
    }

    pub fn get_active_screen(self: &Self) -> &ScreenId {
        &self.data.active_screen
    }
    pub fn get_power(self: &Self) -> &bool {
        &self.data.screen_on
    }
    pub fn get_setup_state(self: &Self) -> &SetupState {
        &self.data.setup_state
    }

    pub fn set_active_screen(self: &mut Self, id: &ScreenId) {
        self.data.active_screen = *id;
        self.write_settings();
    }

    pub fn set_power(self: &mut Self, screen_on: &bool) {
        self.data.screen_on = *screen_on;
        self.write_settings();
    }

    pub fn set_setup_state(self: &mut Self, setup_state: &SetupState) {
        self.data.setup_state = *setup_state;
        self.write_settings();
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

        let _settings: ScoreboardSettingsData = serde_json::from_str(data).unwrap();
    }
}
