use crate::common;
use crate::matrix;
use rocket::{get, ignite, routes, State};
use std::sync::mpsc;
use std::sync::Mutex;
struct ServerState {
    sender: mpsc::Sender<common::MatrixCommand>,
}

impl ServerState {
    fn new(sender: mpsc::Sender<common::MatrixCommand>) -> ServerState {
        ServerState { sender }
    }
}

#[get("/")]
fn index(state: State<Mutex<ServerState>>) -> &'static str {
    "Hello, world!"
}

pub fn run_webserver(sender: mpsc::Sender<common::MatrixCommand>) {
    rocket::ignite()
        .manage(Mutex::new(ServerState::new(sender)))
        .mount("/", routes![index])
        .launch();
}
