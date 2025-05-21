// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use taurpc::Router;
mod missions;
mod telemetry;

use crate::telemetry::rabbitmq::RabbitMQAPI;
use missions::api::{MissionApi, MissionApiImpl};
use telemetry::rabbitmq::RabbitMQAPIImpl;
mod init_db;
use init_db::{clear_database, initialize_database, init_database_dummy_data};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    // Database initialization
    if env::var("CLEAR_DATABASE_EVERYTIME")
        .unwrap_or_default()
        .to_lowercase()
        == "true"
    {
        println!("Clearing database");
        clear_database().await;
    }

    initialize_database().await;

    if env::var("DUMMY_DATA_ENABLED")
        .unwrap_or_default()
        .to_lowercase()
        == "true"
    {
        println!("Seeding dummy data...");
        init_database_dummy_data().await;
    }

    // Initialize APIs outside of Tauri setup
    let rabbitmq_api = RabbitMQAPIImpl::new().await.unwrap();

    let missions_api = MissionApiImpl::new().await;

    // Create router with both handlers
    let router = Router::new()
        .merge(missions_api.into_handler())
        .merge(rabbitmq_api.clone().into_handler());

    let router_handler = router.into_handler();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let rabbitmq = rabbitmq_api.with_app_handle(app_handle);

            if env::var("INITIALIZE_RABBITMQ")
                .unwrap_or_default()
                .to_lowercase()
                == "true"
            {
                // Initialize consumers
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = rabbitmq.init_consumers().await {
                        eprintln!("Failed to initialize telemetry consumers: {}", e);
                    }
                });
            }

            if env::var("TEST_PUBLISHER")
                .unwrap_or_default()
                .to_lowercase()
                == "true"
            {
                // Start test publisher
                tauri::async_runtime::spawn(async {
                    println!("🚀 Starting RabbitMQ test publisher");
                    if let Err(e) = telemetry::publisher::test_publisher().await {
                        eprintln!("❌ Test publisher failed: {}", e);
                    } else {
                        println!("✅ Test publisher finished");
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(move |invoke| router_handler(invoke))
        .run(tauri::generate_context!())
        .expect("Error running Tauri application");
}
