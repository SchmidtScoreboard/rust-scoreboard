use crate::common::{MatrixCommand, ScoreboardSettingsData, ScreenId, WebserverResponse};
use rocket::config::{Config, Environment};
use rocket::response::status;
use rocket::{get, post, routes, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
) -> Result<Json<ScoreboardSettingsData>, status::NotFound<String>> {
    let state = state.lock().unwrap();

    (*state).sender.send(MatrixCommand::GetSettings()).unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::GetSettingsResponse(settings) => Ok(Json(settings)),
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}
#[post("/configure", format = "json", data = "<new_settings>")]
fn configure(
    new_settings: Json<ScoreboardSettingsData>,
    state: State<Mutex<ServerState>>,
) -> Result<Json<ScoreboardSettingsData>, status::NotFound<String>> {
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::UpdateSettings(new_settings.into_inner()))
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::UpdateSettingsResponse(settings) => Ok(Json(settings)),
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}

#[post("/setPower", format = "json", data = "<power_request>")]
fn set_power(
    power_request: Json<PowerRequest>,
    state: State<Mutex<ServerState>>,
) -> Result<Json<ScoreboardSettingsData>, status::NotFound<String>> {
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::SetPower {
            from_webserver: true,
            power: Some(power_request.screen_on),
        })
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::SetPowerResponse(settings) => Ok(Json(settings)),
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}

#[post("/setSport", format = "json", data = "<sport_request>")]
fn set_sport(
    sport_request: Json<SportRequest>,
    state: State<Mutex<ServerState>>,
) -> Result<Json<ScoreboardSettingsData>, status::NotFound<String>> {
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::SetActiveScreen(sport_request.sport))
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::SetPowerResponse(settings) => Ok(Json(settings)),
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}

#[post("/wifi", format = "json", data = "<wifi_request>")]
fn wifi(
    wifi_request: Json<WifiRequest>,
    state: State<Mutex<ServerState>>,
) -> Result<Json<ScoreboardSettingsData>, status::NotFound<String>> {
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
            Some(settings) => Ok(Json(settings)),
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
) -> Result<Json<ScoreboardSettingsData>, status::NotFound<String>> {
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
            Some(settings) => Ok(Json(settings)),
            None => Err(status::NotFound("Failed to show sync".to_string())),
        },
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}
#[post("/reboot", format = "json", data = "<_reboot_request>")]
fn reboot(
    state: State<Mutex<ServerState>>,
    _reboot_request: Json<RebootRequest>,
) -> Result<Json<ScoreboardSettingsData>, status::NotFound<String>> {
    let state = state.lock().unwrap();
    // TODO use reboot request
    (*state).sender.send(MatrixCommand::Reboot()).unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::RebootResponse(settings) => match settings {
            Some(settings) => Ok(Json(settings)),
            None => Err(status::NotFound("Failed to init reboot".to_string())),
        },
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}
#[post("/reset")]
fn reset(
    state: State<Mutex<ServerState>>,
) -> Result<Json<ScoreboardSettingsData>, status::NotFound<String>> {
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
            Some(settings) => Ok(Json(settings)),
            None => Err(status::NotFound("Failed to init reboot".to_string())),
        },
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}

#[post("/sync")]
fn sync(
    state: State<Mutex<ServerState>>,
) -> Result<Json<ScoreboardSettingsData>, status::NotFound<String>> {
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
            Some(settings) => Ok(Json(settings)),
            None => Err(status::NotFound("Failed to move to ready".to_string())),
        },
        _ => Err(status::NotFound("Internal error".to_string())),
    }
}
#[post("/connect")]
fn connect(
    state: State<Mutex<ServerState>>,
) -> Result<Json<ScoreboardSettingsData>, status::NotFound<String>> {
    let state = state.lock().unwrap();
    (*state)
        .sender
        .send(MatrixCommand::GotHotspotConnection())
        .unwrap();
    let response = (*state).receiver.recv().unwrap();
    match response {
        WebserverResponse::GotHotspotConnectionResponse(settings) => match settings {
            Some(settings) => Ok(Json(settings)),
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
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(5005)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .manage(Mutex::new(ServerState::new(sender, receiver, file_path)))
        .mount(
            "/",
            routes![
                index, configure, set_power, set_sport, wifi, logs, show_sync, reboot, reset, sync,
                connect
            ],
        )
        .launch();
}
