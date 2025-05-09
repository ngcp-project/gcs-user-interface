// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use taurpc::Router;

mod missions;
use missions::api::{MissionApi, MissionApiImpl};

use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager, RunEvent};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;

fn spawn_opencv_sidecar(app_handle: tauri::AppHandle) -> Result<(), String> {
    // Check if a sidecar process already exists
    if let Some(state) = app_handle.try_state::<Arc<Mutex<Option<CommandChild>>>>() {
        let child_process = state.lock().unwrap();
        if child_process.is_some() {
            println!("[tauri] Sidecar is already running. Skipping spawn.");
            return Ok(());
        }
    }

    // Spawn sidecar (sidecar function only expects the filename, not the whole path configured in externalBin)
    let sidecar_command = app_handle
        .shell()
        .sidecar("opencv")
        .map_err(|e| e.to_string())?;
    let (mut rx, child) = sidecar_command.spawn().map_err(|e| e.to_string())?;

    // Store the child process in the app state
    if let Some(state) = app_handle.try_state::<Arc<Mutex<Option<CommandChild>>>>() {
        *state.lock().unwrap() = Some(child);
    } else {
        return Err("Failed to access app state".to_string());
    }

    // Spawn an async task to handle sidecar communication
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    println!("Sidecar stdout: {}", line);

                    // Emit the line to the frontend
                    app_handle
                        .emit("sidecar-stdout", line.to_string())
                        .expect("Failed to emit sidecar stdout event");
                }

                CommandEvent::Stderr(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    eprintln!("Sidecar stderr: {}", line);
                    // Emit the error line to the frontend
                    app_handle
                        .emit("sidecar-stderr", line.to_string())
                        .expect("Failed to emit sidecar stderr event");
                }
                _ => {}
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn start_sidecar(app_handle: tauri::AppHandle) -> Result<String, String> {
    println!("[tauri] Received command to start sidecar.");
    spawn_opencv_sidecar(app_handle)?;
    Ok("Sidecar spawned and monitoring started.".to_string())
}

#[tauri::command]
fn shutdown_sidecar(app_handle: tauri::AppHandle) -> Result<String, String> {
    println!("[tauri] Received command to shutdown sidecar.");
    // Access the sidecar process state
    if let Some(state) = app_handle.try_state::<Arc<Mutex<Option<CommandChild>>>>() {
        let mut child_process = state
            .lock()
            .map_err(|_| "[tauri] Failed to acquire lock on sidecar process.")?;

        if let Some(mut process) = child_process.take() {
            let command = "sidecar shutdown\n"; // Add newline to signal the end of the command

            // Attempt to write the command to the sidecar's stdin
            if let Err(err) = process.write(command.as_bytes()) {
                println!("[tauri] Failed to write to sidecar stdin: {}", err);

                // Restore the process reference if shutdown fails
                *child_process = Some(process);
                return Err(format!("Failed to write to sidecar stdin: {}", err));
            }

            println!("[tauri] Sent 'sidecar shutdown' command to sidecar.");
            Ok("'sidecar shutdown' command sent.".to_string())
        } else {
            println!("[tauri] No active sidecar process to shutdown.");
            Err("No active sidecar process to shutdown.".to_string())
        }
    } else {
        Err("Sidecar process state not found.".to_string())
    }
}

#[tokio::main]
async fn main() {
    // Initialize apis here
    let missions_api = MissionApiImpl::default();

    let router = Router::new().merge(missions_api.into_handler());

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Store the initial sidecar process in the app state
            app.manage(Arc::new(Mutex::new(None::<CommandChild>)));
            // Clone the app handle for use elsewhere
            let app_handle = app.handle();
            // Spawn the Python sidecar on startup
            println!("[tauri] Creating sidecar...");
            spawn_opencv_sidecar(app_handle.clone()).ok();
            println!("[tauri] Sidecar spawned and monitoring started.");
            Ok(())
        })
        // Register the shutdown_server command
        .invoke_handler(tauri::generate_handler![start_sidecar, shutdown_sidecar,])
        .invoke_handler(router.into_handler())
        .build(tauri::generate_context!())
        .expect("Error while running tauri application")
        .run(|app_handle, event| match event {
            // Ensure the Python sidecar is killed when the app is closed
            RunEvent::ExitRequested { .. } => {
                if let Some(child_process) =
                    app_handle.try_state::<Arc<Mutex<Option<CommandChild>>>>()
                {
                    if let Ok(mut child) = child_process.lock() {
                        if let Some(process) = child.as_mut() {
                            // Send msg via stdin to sidecar where it self terminates
                            let command = "sidecar shutdown\n";
                            let buf: &[u8] = command.as_bytes();
                            let _ = process.write(buf);

                            // Force kill the process after a short delay to ensure cleanup
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            if let Some(process) = child.take() {
                                let _ = process.kill();
                            }

                            // TODO: This is kind of messy, find a better way to clear, preferably cross platform
                            // Additional cleanup for Windows
                            #[cfg(target_os = "windows")]
                            {
                                use std::process::Command;
                                let _ = Command::new("taskkill")
                                    .args(["/F", "/IM", "opencv.exe"])
                                    .output();
                            }

                            println!("[tauri] Sidecar closed.");
                        }
                    }
                }
            }
            _ => {}
        });
}
