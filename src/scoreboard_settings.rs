use crate::common::{self, ScoreboardSettingsData, ScreenId, SetupState};
use chrono_tz::Tz;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

#[derive(PartialEq, Debug)]
pub struct ScoreboardSettings {
    data: Arc<ScoreboardSettingsData>,
    pub file_path: PathBuf,
}

impl ScoreboardSettings {
    pub fn new(data: Arc<ScoreboardSettingsData>, file_path: PathBuf) -> ScoreboardSettings {
        ScoreboardSettings { data, file_path }
    }

    pub fn get_settings(&self) -> Arc<ScoreboardSettingsData> {
        self.data.clone()
    }

    fn write_settings(&self) {
        fs::write(
            &self.file_path,
            serde_json::to_string_pretty(&self.data).unwrap(),
        )
        .unwrap();
    }

    pub fn update_settings(&mut self, new_settings: ScoreboardSettingsData) {
        self.data = Arc::from(self.data.update_settings(new_settings));
        self.write_settings();
    }

    pub fn get_active_screen(&self) -> &ScreenId {
        &self.data.active_screen
    }
    pub fn get_power(&self) -> &bool {
        &self.data.screen_on
    }
    pub fn get_auto_power(&self) -> &bool {
        &self.data.auto_power
    }
    pub fn get_setup_state(&self) -> &SetupState {
        &self.data.setup_state
    }

    pub fn get_rotation_time(&self) -> Duration {
        self.data.rotation_time
    }
    pub fn get_startup_power(&self) -> &Option<bool> {
        &self.data.startup_power
    }
    pub fn get_startup_auto_power(&self) -> &Option<bool> {
        &self.data.startup_auto_power
    }
    pub fn get_auto_power_mode(&self) -> &common::AutoPowerMode {
        &self.data.auto_power_mode
    }

    pub fn set_rotation_time(&mut self, rotation_time: Duration) {
        let mut copy: ScoreboardSettingsData = self.data.as_ref().clone();
        copy.rotation_time = rotation_time;
        self.data = Arc::from(copy);
        self.write_settings();
    }

    pub fn get_brightness(&self) -> u8 {
        self.data.brightness
    }

    pub fn set_active_screen(&mut self, id: &ScreenId) {
        let mut copy: ScoreboardSettingsData = self.data.as_ref().clone();
        copy.active_screen = *id;
        self.data = Arc::from(copy);
        self.write_settings();
    }

    pub fn set_power(&mut self, screen_on: &bool) {
        let mut copy: ScoreboardSettingsData = self.data.as_ref().clone();
        copy.screen_on = *screen_on;
        self.data = Arc::from(copy);
        self.write_settings();
    }
    pub fn set_auto_power(&mut self, auto_power: &bool) {
        let mut copy: ScoreboardSettingsData = self.data.as_ref().clone();
        copy.auto_power = *auto_power;
        self.data = Arc::from(copy);
        self.write_settings();
    }
    pub fn set_startup_settings(
        &mut self,
        startup_power: Option<bool>,
        startup_auto_power: Option<bool>,
    ) {
        let mut copy: ScoreboardSettingsData = self.data.as_ref().clone();
        copy.startup_power = startup_power;
        copy.startup_auto_power = startup_auto_power;
        self.data = Arc::from(copy);
        self.write_settings();
    }

    pub fn set_setup_state(&mut self, setup_state: &SetupState) {
        let mut copy: ScoreboardSettingsData = self.data.as_ref().clone();
        copy.setup_state = *setup_state;
        self.data = Arc::from(copy);
        self.write_settings();
    }

    pub fn set_version(&mut self, version: u32) {
        let mut copy: ScoreboardSettingsData = self.data.as_ref().clone();
        copy.version = version;
        self.data = Arc::from(copy);
        self.write_settings();
    }

    pub fn get_timezone(&self) -> &Tz {
        &self.data.timezone
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weird_name() {
        let name = "asf'@`😡";
        let data = format!(
            r#"{{
    "timezone": "US/Central",
    "setup_state": 10,
    "active_screen": 0,
    "mac_address": "b8:27:eb:6b:64:25",
    "screens": [
        {{
            "rotation_time": 10,
            "subtitle": "View scores from professional hockey",
            "focus_teams": [],
            "id": 0,
            "always_rotate": true,
            "name": "Hockey "
        }},
        {{
            "rotation_time": 10,
            "subtitle": "View scores from professional baseball",
            "focus_teams": [],
            "id": 1,
            "always_rotate": false,
            "name": "Baseball"
        }},
        {{
            "rotation_time": 10,
            "subtitle": "Show the current time",
            "focus_teams": [],
            "id": 50,
            "always_rotate": false,
            "name": "Clock"
        }}
    ],
    "screen_on": true,
    "version": 1,
    "name": "{}"

        }}"#,
            name
        );

        let settings: ScoreboardSettingsData = serde_json::from_str(&data).unwrap();
        assert_eq!(name, settings.name);
    }
}
