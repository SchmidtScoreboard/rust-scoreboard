use crate::common::{
    CommandSource, MatrixCommand, ScoreboardSettingsData, ScreenId, WebserverResponse,
};
use rocket::config::{Config, Environment};
use rocket::response::{status, Content};
use rocket::{get, http::ContentType, post, routes, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use std::sync::Mutex;
use std::sync::{mpsc, Arc};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct GameAction{
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct PowerRequest {
    screen_on: bool,
}
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AutoPowerRequest {
    auto_power: bool,
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
    receiver: mpsc::Receiver<WebserverResponse>,
    file_path: PathBuf,
}

impl ServerState {
    fn new(
        sender: mpsc::Sender<MatrixCommand>,
        receiver: mpsc::Receiver<WebserverResponse>,
        file_path: PathBuf,
    ) -> ServerState {
        ServerState {
            sender,
            receiver,
            file_path,
        }
    }
}

#[get("/")]
fn index(
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();

    let state = state.lock().unwrap();

    (*state).sender.send(MatrixCommand::GetSettings()).unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::GetSettingsResponse(settings) => Ok(Content(content, Json(settings))),
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}
#[get("/version")]
fn version() -> Result<String, status::NotFound<String>> {
    let version = self_update::cargo_crate_version!();
    Ok(version.to_string())
}
#[post("/configure", format = "json", data = "<new_settings>")]
fn configure(
    new_settings: Json<ScoreboardSettingsData>,
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::UpdateSettings(new_settings.into_inner()))
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::UpdateSettingsResponse(settings) => Ok(Content(content, Json(settings))),
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}
#[post("/gameAction", format = "json", data = "<_game_action>")]
fn game_action(
    _game_action: Json<GameAction>,
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::GameAction())
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::GameActionResponse(settings) => Ok(Content(content, Json(settings))),
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}

#[post("/setPower", format = "json", data = "<power_request>")]
fn set_power(
    power_request: Json<PowerRequest>,
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::SetPower {
            source: CommandSource::Webserver(),
            power: Some(power_request.screen_on),
        })
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::SetPowerResponse(settings) => Ok(Content(content, Json(settings))),
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}
#[post("/autoPower", format = "json", data = "<auto_power_request>")]
fn auto_power(
    auto_power_request: Json<AutoPowerRequest>,
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::AutoPower(auto_power_request.auto_power))
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::SetAutoPowerResponse(settings) => Ok(Content(content, Json(settings))),
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}

#[post("/setSport", format = "json", data = "<sport_request>")]
fn set_sport(
    sport_request: Json<SportRequest>,
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::SetActiveScreen(sport_request.sport))
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::SetActiveScreenResponse(settings) => {
            Ok(Content(content, Json(settings)))
        }
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}

#[post("/wifi", format = "json", data = "<wifi_request>")]
fn wifi(
    wifi_request: Json<WifiRequest>,
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::GotWifiDetails {
            ssid: wifi_request.ssid.clone(),
            password: wifi_request.psk.clone(),
        })
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::GotWifiDetailsResponse(settings) => match settings {
            Some(settings) => Ok(Content(content, Json(settings))),
            None => Err(status::NotFound("Failed to setup wifi".to_string())),
        },
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}

#[get("/logs")]
fn logs(state: State<Mutex<ServerState>>) -> Result<String, std::io::Error> {
    let state = state.lock().unwrap();
    let logs_dir = {
        let path = (*state).file_path.clone();
        path.join("logs/")
    };
    let entries = fs::read_dir(logs_dir)?;
    let mut out: String = format!("Begin log file\n");
    for entry in entries {
        debug!("entry {:?}", entry);
        match entry {
            Ok(entry) => {
                let log_output = fs::read_to_string(entry.path())?;
                out.push_str(&format!("\n\nNEW LOG FILE {:?} \n\n", entry.path()));
                out.push_str(&log_output);
            }
            _ => {}
        }
    }
    Ok(out)
}

#[post("/showSync")]
fn show_sync(
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::SyncCommand {
            from_webserver: true,
            show_sync: Some(true),
        })
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::SyncCommandResponse(settings) => match settings {
            Some(settings) => Ok(Content(content, Json(settings))),
            None => Err(status::NotFound("Failed to show sync".to_string())),
        },
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}
#[post("/reboot")]
fn reboot(
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();
    let state = state.lock().unwrap();
    (*state).sender.send(MatrixCommand::Reboot{is_nightly_reboot: false}).unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::RebootResponse(settings) => match settings {
            Some(settings) => Ok(Content(content, Json(settings))),
            None => Err(status::NotFound("Failed to init reboot".to_string())),
        },
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}
#[post("/reset")]
fn reset(
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::Reset {
            from_webserver: true,
        })
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::ResetResponse(settings) => match settings {
            Some(settings) => Ok(Content(content, Json(settings))),
            None => Err(status::NotFound("Failed to init reboot".to_string())),
        },
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}

#[post("/sync")]
fn sync(
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::SyncCommand {
            from_webserver: true,
            show_sync: Some(false),
        })
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::SyncCommandResponse(settings) => match settings {
            Some(settings) => Ok(Content(content, Json(settings))),
            None => Err(status::NotFound("Failed to move to ready".to_string())),
        },
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}
#[post("/connect")]
fn connect(
    state: State<Mutex<ServerState>>,
) -> Result<Content<Json<Arc<ScoreboardSettingsData>>>, status::NotFound<String>> {
    let content = ContentType::parse_flexible("application/json; charset=utf-8").unwrap();
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::GotHotspotConnection())
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::GotHotspotConnectionResponse(settings) => match settings {
            Some(settings) => Ok(Content(content, Json(settings))),
            None => Err(status::NotFound(
                "Failed to handle hotspot connection".to_string(),
            )),
        },
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}

pub fn run_webserver(
    sender: mpsc::Sender<MatrixCommand>,
    receiver: mpsc::Receiver<WebserverResponse>,
    file_path: PathBuf,
) {
    let config = Config::build(Environment::Production)
        .address("0.0.0.0")
        .log_level(rocket::config::LoggingLevel::Critical)
        .port(5005)
        .workers(1)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .manage(Mutex::new(ServerState::new(sender, receiver, file_path)))
        .mount(
            "/",
            routes![
                index, configure, set_power, auto_power, set_sport, wifi, logs, show_sync, reboot,
                reset, sync, connect, version, game_action
            ],
        )
        .launch();
}
