use crate::common;
use crate::scoreboard_settings;
use rocket::config::{Config, Environment};
use rocket::{get, routes, State};
use rocket_contrib::json::Json;

use std::fs;

use std::sync::mpsc;
use std::sync::Mutex;
struct ServerState {
    sender: mpsc::Sender<common::MatrixCommand>,
    settings: scoreboard_settings::ScoreboardSettings,
}

impl ServerState {
    fn new(
        sender: mpsc::Sender<common::MatrixCommand>,
        settings: scoreboard_settings::ScoreboardSettings,
    ) -> ServerState {
        ServerState { sender, settings }
    }
}

#[get("/")]
fn index(state: State<Mutex<ServerState>>) -> Json<scoreboard_settings::ScoreboardSettingsData> {
    let state = state.lock().unwrap();
    let copy = (*state).settings.data.clone();
    Json(copy)
}
// #[post("/configure", format = "json", data = "<user>")]
// fn configure(state: State<Mutex<ServerState>>) -> &'static str {
//     "Hello, world!"
// }

pub fn run_webserver(
    sender: mpsc::Sender<common::MatrixCommand>,
    settings: scoreboard_settings::ScoreboardSettings,
) {
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(5005)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .manage(Mutex::new(ServerState::new(sender, settings)))
        .mount("/", routes![index])
        .launch();
}
