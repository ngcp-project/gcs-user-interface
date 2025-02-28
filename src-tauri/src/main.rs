use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;

mod telemetry;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![telemetry::get_telemetry])
        .setup(|app| {
            let app_data = telemetry::initialize_telemetry_data();
            app.manage(Arc::new(Mutex::new(app_data)));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}