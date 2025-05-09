use crate::telemetry::types::TelemetryData;
use crate::telemetry::types::{AppData, Coordinate};
use futures_util::stream::StreamExt;
use lapin::{
    options::*, types::FieldTable, Channel, Connection, ConnectionProperties, Consumer, Queue,
    Result as LapinResult,
};
use serde_json::json;
use serde_json::Value;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tauri::Window;
use tokio::sync::Mutex;
use tokio_amqp::*;
use std::time::{SystemTime, UNIX_EPOCH};
//creating the structure of the rabbitMQ Consumer
#[derive(Clone)]
pub struct RabbitMQConsumer {
    connection: Arc<Mutex<Connection>>,
    channel: Channel,
    window: Window,
}

// impl RabbitMQConsumer{
//     self.connection =
// }

impl RabbitMQConsumer {
    pub async fn new(addr: &str, window: Window) -> LapinResult<Self> {
        let connection = Connection::connect(
            // let connection = Arc::new(Mutex::new(Connection::connect(
            addr, // certain address
            ConnectionProperties::default().into(),
        )
        .await?;
        let connection = Arc::new(Mutex::new(connection));
        // one single channel for different queues
        let channel = connection.lock().await.create_channel().await?;
        Ok(Self {
            connection,
            channel,
            window,
        })
    }
    //Declare a queue where for the consumer
    pub async fn queue_declare(&self, queue_name: &str) -> LapinResult<Queue> {
        self.channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable: true,
                    auto_delete: false,
                    exclusive: false,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await //waits for the async operation to complete, ? will return any error operation to the calling function
    }

    pub async fn create_consumer(&self, queue_name: &str) -> LapinResult<Consumer> {
        self.channel
            .basic_consume(
                queue_name,
                "telemetry",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
    }
    pub async fn start_consuming(&self, queue_name: &str) -> LapinResult<()> {
        //Declare queue
        // self.queue_declare(queue_name).await?;
        //Create consumer
        let consumer = self.create_consumer(queue_name).await?;
        //Start processing
        self.process_telemetry(queue_name, consumer).await?;

        Ok(())
    }

    //In this part we should emit to the frontend, ads

    pub async fn process_telemetry(&self, mut queue_name: &str, mut consumer: Consumer) -> LapinResult<()> {
        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                let raw_message = String::from_utf8_lossy(&delivery.data);
                println!("Raw received message: {}", raw_message);

                // Step 1: Deserialize into a generic JSON object first
                let parsed_json: Result<Value, _> = serde_json::from_str(&raw_message);
                
                match parsed_json {
                    Ok(mut json_data) => {
                        // Step 2: Transform JSON to match `TelemetryData`
                        let transformed_data = json!({
                            "vehicle_id": queue_name.strip_prefix("telemetry_").unwrap_or("unknown").to_string(),  // Add vehicle ID if missing
                            "pitch": json_data["pitch"].as_f64().unwrap_or(0.0) as f32,
                            "yaw": json_data["yaw"].as_f64().unwrap_or(0.0) as f32,
                            "roll": json_data["roll"].as_f64().unwrap_or(0.0) as f32,
                            "speed": json_data["speed"].as_f64().unwrap_or(0.0) as f32,
                            "altitude": json_data["alt"].as_f64().unwrap_or(0.0) as f32,
                            "battery_life": json_data["battery_life"].as_f64().unwrap_or(0.0) as f32,
                            "current_position": {
                                "latitude": json_data["current_latitude"].as_f64().unwrap_or(0.0),
                                "longitude": json_data["current_longitude"].as_f64().unwrap_or(0.0)
                            },
                            "last_updated": match json_data["lastUpdated"].as_i64() {
                                Some(ts) => UNIX_EPOCH + std::time::Duration::from_secs(ts as u64),
                                None => SystemTime::UNIX_EPOCH,
                            },
                            "patient_status": json_data["patient_status"].as_i64().unwrap_or(0) as i8,
                            "vehicle_status": json_data["vehicle_status"].as_i64().unwrap_or(0) as i8,
                            "request_coordinate": {
                                "message_flag": json_data["message_flag"].as_i64().unwrap_or(0) as i32,
                                "request_location": {
                                    "latitude": json_data["message_lat"].as_f64().unwrap_or(0.0),
                                    "longitude": json_data["message_lon"].as_f64().unwrap_or(0.0)
                                },
                                "patient_secured": Option::<bool>::None

                            }
                        });

                        // Step 3: Deserialize into `TelemetryData`
                        match serde_json::from_value::<TelemetryData>(transformed_data.clone()) {
                            Ok(data) => {
                                let payload = json!({
                                    "vehicle_id": data.vehicle_id,
                                    "telemetry": data
                                });

                                if let Err(e) = self.window.emit("telemetry_update", payload.clone()) {
                                    println!("Failed to emit telemetry update: {}", e);
                                }

                                println!("✅ Successfully received and parsed telemetry: {:?}", payload);
                                delivery.ack(BasicAckOptions::default()).await?;
                            }
                            Err(e) => {
                                println!("❌ Failed to deserialize into TelemetryData: {}. Transformed JSON: {:?}", e, transformed_data);
                                delivery.reject(BasicRejectOptions::default()).await?;
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ Failed to parse raw JSON: {}. Raw message: {}", e, raw_message);
                        delivery.reject(BasicRejectOptions::default()).await?;
                    }
                }
            }
        }
        Ok(())
    }

}

#[tauri::command]
pub async fn init_telemetry_consumer(window: Window, vehicle_id: String) -> Result<(), String> {
    // Validate vehicle ID
    let valid_vehicle_ids = vec!["eru", "fra", "mea", "mra"];
    if !valid_vehicle_ids.contains(&vehicle_id.as_str()) {
        return Err(format!("Invalid vehicle ID: {}", vehicle_id));
    }

    // Create a consumer specific to this vehicle
    let consumer = RabbitMQConsumer::new("amqp://admin:admin@localhost:5672/%2f", window.clone())
        .await
        .map_err(|e| e.to_string())?;

    // Create the queue name for this specific vehicle
    let queue_name = format!("telemetry_{}", vehicle_id);

    // Start consuming for this specific vehicle
    tokio::spawn({
        let consumer = consumer.clone();
        let queue = queue_name.clone();
        async move {
            if let Err(e) = consumer.start_consuming(&queue).await {
                eprintln!("Failed to consume from queue {}: {}", queue, e);

                // Optionally, emit an error event to the frontend
                let _ = consumer.window.emit(
                    "telemetry_error",
                    json!({
                        "vehicle_id": vehicle_id,
                        "error": e.to_string()
                    }),
                );
            }
        }
    });

    // Return success immediately - the consumer will run in the background
    Ok(())
}
