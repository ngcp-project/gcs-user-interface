// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use taurpc::Router;

mod missions;
use missions::api::{MissionApi, MissionApiImpl};

use tauri::{Emitter, Manager};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

fn spawn_opencv_sidecar(app: &tauri::AppHandle, window: tauri::WebviewWindow) {
    // sidecar() function expects just the filename, NOT the whole path configured in the externalBin array.
    let sidecar_command = app.shell().sidecar("opencv").unwrap();
    let (mut rx, mut child) = sidecar_command.spawn().expect("Failed to spawn sidecar");

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line_bytes) = event {
                let line = String::from_utf8_lossy(&line_bytes);
                window
                    .emit("message", Some(format!("'{}'", line)))
                    .expect("failed to emit event");
                // write to stdin
                child.write("message from Rust\n".as_bytes()).unwrap();
            }
        }
    });
}

#[tokio::main]
async fn main() {
    // Initialize apis here
    let missions_api = MissionApiImpl::default();

    let router = Router::new().merge(missions_api.into_handler());

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let window = app.get_webview_window("main").expect("No main window found");

            spawn_opencv_sidecar(app_handle, window);

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(router.into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
