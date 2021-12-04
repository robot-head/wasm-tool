#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use poem::{listener::TcpListener, Server};
use routes::get_routes;
mod handlers;
mod routes;
async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8081");
    Server::new(listener).run(get_routes()).await.unwrap();
}

fn main() {
    tauri::async_runtime::spawn(start_server());
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
