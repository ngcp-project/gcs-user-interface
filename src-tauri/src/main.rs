#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties, BasicProperties,
};
use serde::Serialize;
use serde_json::json;
use std::env;
use tauri::{AppHandle, Manager};
use tokio::runtime::Runtime;

#[derive(Clone, Serialize)]
struct CommandMessage {
    vehicle_id: String,
    command_type: String,
    command_data: serde_json::Value,
}

#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    Ok(format!(
        "Hello, {}! You've been greeted from Rust!",
        name
    ))
}

#[tauri::command]
async fn emergency_stop(app_handle: AppHandle, vehicle: String) -> Result<(), String> {
    // Build the command message.
    let command = CommandMessage {
        vehicle_id: vehicle.clone(),
        command_type: "EMERGENCY_STOP".into(),
        command_data: json!({ "emergency": true }),
    };

    // Publish the command message to RabbitMQ.
    if let Err(e) = publish_command(&command).await {
        return Err(format!("Failed to publish command: {}", e));
    }

    // Emit an event so the frontend knows the command was sent.
    app_handle
        .emit_all("command_sent", command)
        .map_err(|e| e.to_string())?;

    println!("Emergency stop command published for vehicle {}", vehicle);
    Ok(())
}

async fn publish_command(cmd: &CommandMessage) -> Result<(), Box<dyn std::error::Error>> {
    // Use an environment variable or default address.
    let addr = env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    // Remove .with_tokio() since it's not available:
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    // Declare the queue (if it doesn't exist).
    channel
        .queue_declare(
            "vehicle_commands",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    // Serialize the command.
    let payload = serde_json::to_vec(cmd)?;
    channel
        .basic_publish(
            "",
            "vehicle_commands",
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await?
        .await?; // Wait for publish confirmation.

    conn.close(0, "").await?;
    Ok(())
}

fn main() {
    // Create a Tokio runtime for our async commands.
    let _rt = Runtime::new().unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, emergency_stop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
