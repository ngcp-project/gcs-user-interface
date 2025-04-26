// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Builder;
use taurpc::Router;
use rabbitmq::{TelemApiImpl, CommandsApiImpl};

// Import the traits so that into_handler() and the RPC methods exist
use rabbitmq::telem::TeleApi;
use rabbitmq::commands::CommandsApi;

mod missions;
mod rabbitmq;

use missions::api::{MissionApiImpl, MissionApi};

#[tokio::main]
async fn main() {
    // Initialize each impl (they all impl Default + Clone)
    let missions_api = MissionApiImpl::default();
    let telem_api = TelemApiImpl::default();
    let commands_api = CommandsApiImpl::default();

    // Clone the impls for the router
    let telem_handler = telem_api.clone();
    let commands_handler = commands_api.clone();

    // Merge all API handlers into a single router
    let router = Router::new()
        .merge(missions_api.into_handler())
        .merge(telem_handler.into_handler())
        .merge(commands_handler.into_handler());

    Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(router.into_handler())
        .setup(move |app| {
            // Spawn your telemetry listener, re-using the same telem_api
            let app_handle = app.handle();
            let handle_clone = app_handle.clone();
            let telem_clone = telem_api.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = telem_clone
                    .start_telemetry_listener(handle_clone)
                    .await
                {
                    eprintln!("Failed to start telemetry listener: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
