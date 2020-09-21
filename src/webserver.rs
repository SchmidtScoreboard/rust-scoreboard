use crate::common::{MatrixCommand, ScoreboardSettingsData, ScreenId, SetupState};
use crate::scoreboard_settings::ScoreboardSettings;
use rocket::config::{Config, Environment};
use rocket::{get, post, routes, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use std::sync::mpsc;
use std::sync::Mutex;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct PowerRequest {
    screen_on: bool,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct SportRequest {
    sport: ScreenId,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct WifiRequest {
    ssid: String,
    psk: String,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RebootRequest {
    restart: Option<String>,
    reboot_message: Option<String>,
}

struct ServerState {
    sender: mpsc::Sender<MatrixCommand>,
    settings: ScoreboardSettings,
}

impl ServerState {
    fn new(sender: mpsc::Sender<MatrixCommand>, settings: ScoreboardSettings) -> ServerState {
        ServerState { sender, settings }
    }
}

#[get("/")]
fn index(state: State<Mutex<ServerState>>) -> Json<ScoreboardSettingsData> {
    let state = state.lock().unwrap();
    let copy = (*state).settings.get_settings_clone();
    Json(copy)
}
#[post("/configure", format = "json", data = "<new_settings>")]
fn configure(
    new_settings: Json<ScoreboardSettingsData>,
    state: State<Mutex<ServerState>>,
) -> Json<ScoreboardSettingsData> {
    let mut state = state.lock().unwrap();
    (*state).settings.update_settings(new_settings.into_inner());
    (*state)
        .sender
        .send(MatrixCommand::UpdateSettings(
            (*state).settings.get_settings_clone(),
        ))
        .unwrap();
    Json((*state).settings.get_settings_clone())
}

#[post("/setPower", format = "json", data = "<power_request>")]
fn set_power(
    power_request: Json<PowerRequest>,
    state: State<Mutex<ServerState>>,
) -> Json<ScoreboardSettingsData> {
    let mut state = state.lock().unwrap();
    (*state).settings.set_power(&power_request.screen_on);
    (*state)
        .sender
        .send(MatrixCommand::SetPower(power_request.screen_on))
        .unwrap();
    Json((*state).settings.get_settings_clone())
}

#[post("/setSport", format = "json", data = "<sport_request>")]
fn set_sport(
    sport_request: Json<SportRequest>,
    state: State<Mutex<ServerState>>,
) -> Json<ScoreboardSettingsData> {
    let mut state = state.lock().unwrap();
    (*state).settings.set_active_screen(&sport_request.sport);
    (*state)
        .sender
        .send(MatrixCommand::SetActiveScreen(sport_request.sport))
        .unwrap();
    Json((*state).settings.get_settings_clone())
}

#[post("/wifi", format = "json", data = "<_wifi_request>")]
fn wifi(
    _wifi_request: Json<WifiRequest>,
    state: State<Mutex<ServerState>>,
) -> Json<ScoreboardSettingsData> {
    let state = state.lock().unwrap();
    // TODO send matrix command to start restart with wifi

    Json((*state).settings.get_settings_clone())
}

#[get("/logs")]
fn logs(state: State<Mutex<ServerState>>) -> Json<()> {
    let _state = state.lock().unwrap();
    // TODO read log file and send response
    // let log_path = (*state).settings.file_path.pop().join("scoreboard.log");
    Json(())
}

#[post("/showSync")]
fn show_sync(state: State<Mutex<ServerState>>) -> Json<ScoreboardSettingsData> {
    let mut state = state.lock().unwrap();

    // Matrix command turn on screen
    (*state).settings.set_power(&true);
    match (*state).settings.get_settings().setup_state {
        SetupState::Ready => {
            (*state).settings.set_setup_state(&SetupState::Sync);
        }
        SetupState::Sync => {
            (*state).settings.set_setup_state(&SetupState::Ready);
        }
        _ => eprintln!(
            "Cannot set sync mode while in setup state {:?}",
            (*state).settings.get_settings().setup_state
        ),
    }
    Json((*state).settings.get_settings_clone())
}
#[post("/reboot", format = "json", data = "<_reboot_request>")]
fn reboot(
    state: State<Mutex<ServerState>>,
    _reboot_request: Json<RebootRequest>,
) -> Json<ScoreboardSettingsData> {
    let state = state.lock().unwrap();
    // TODO use reboot request
    Json((*state).settings.get_settings_clone())
}

#[post("/sync")]
fn sync(state: State<Mutex<ServerState>>) -> Json<ScoreboardSettingsData> {
    let mut state = state.lock().unwrap();
    (*state).settings.set_power(&true);
    // Matrix command turn on screen
    match (*state).settings.get_settings().setup_state {
        SetupState::Sync => {
            (*state).settings.set_setup_state(&SetupState::Ready);
            // TODO fire matrix command
        }
        _ => eprintln!(
            "Cannot sync while in setup state {:?}",
            (*state).settings.get_settings().setup_state
        ),
    }
    Json((*state).settings.get_settings_clone())
}
#[post("/connect")]
fn connect(state: State<Mutex<ServerState>>) -> Json<ScoreboardSettingsData> {
    let mut state = state.lock().unwrap();
    (*state).settings.set_power(&true);
    // Matrix command turn on screen
    match (*state).settings.get_settings().setup_state {
        SetupState::Hotspot => {
            (*state).settings.set_setup_state(&SetupState::WifiConnect);
            // TODO fire matrix command
        }
        _ => eprintln!(
            "Cannot connect while in setup state {:?}",
            (*state).settings.get_settings().setup_state
        ),
    }
    Json((*state).settings.get_settings_clone())
}

pub fn run_webserver(sender: mpsc::Sender<MatrixCommand>, settings: ScoreboardSettings) {
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(5005)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .manage(Mutex::new(ServerState::new(sender, settings)))
        .mount(
            "/",
            routes![
                index, configure, set_power, set_sport, wifi, logs, show_sync, reboot, sync,
                connect
            ],
        )
        .launch();
}
