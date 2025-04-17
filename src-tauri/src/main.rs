// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use taurpc::Router;

mod missions;
use missions::api::{MissionApi, MissionApiImpl};

#[tokio::main]
async fn main() {
    // Initialize apis here
    let missions_api = MissionApiImpl::default();

    let router = Router::new().merge(missions_api.into_handler());

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(router.into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
