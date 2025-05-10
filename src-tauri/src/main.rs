use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

mod telemetry;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            telemetry::init_telemetry_consumer,
            telemetry::update_keep_out_zone
        ])
        .setup(|app| {
            // Create a background task using the Tauri runtime
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                println!("Starting RabbitMQ test publisher");
                match telemetry::publisher::test_publisher().await {
                    Ok(_) => println!("Test completed successfully"),
                    Err(e) => eprintln!("Test failed: {}", e),
                }
            });

            // You can uncomment and fix this section if needed
            // let app_data = telemetry::new();
            // app.manage(Arc::new(Mutex::new(app_data)));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
