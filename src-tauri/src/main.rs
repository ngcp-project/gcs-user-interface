use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

mod telemetry;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![telemetry::init_telemetry_consumer])
        .setup(|app| {
            // Start RabbitMQ Consumer
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                println!("Starting RabbitMQ Consumer...");
                let vehicle_id = "eru".to_string();  // Adjust this as needed
                match telemetry::init_telemetry_consumer(app_handle.get_window("main").unwrap(), vehicle_id).await {
                    Ok(_) => println!("Consumer initialized successfully"),
                    Err(e) => eprintln!("Consumer initialization failed: {}", e),
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
