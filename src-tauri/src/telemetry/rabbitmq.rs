use crate::telemetry::types::TelemetryData;
use crate::telemetry::types::{AppData, Coordinate};
use futures_util::stream::StreamExt;
use lapin::{
    options::*, types::FieldTable, Channel, Connection, ConnectionProperties, Consumer, Queue,
    Result as LapinResult,
};
use serde_json::json;
use std::sync::Arc;
use tauri::Window;
use tokio::sync::Mutex;
use tokio_amqp::*;

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
            ConnectionProperties::default().with_tokio(),
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
                "telemetry_consumer",
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
        self.process_telemetry(consumer).await?;

        Ok(())
    }

    //In this part we should emit to the frontend, ads

    pub async fn process_telemetry(&self, mut consumer: Consumer) -> LapinResult<()> {
        use futures_util::StreamExt;
        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                if let Ok(data) = serde_json::from_slice::<TelemetryData>(&delivery.data) {
                    let payload = json!({
                        // map according to the structure key | val
                        "vehicle_id": data.vehicle_id,
                        "telemetry": data
                    });
                    //we have different structure: consumer and publisher
                    if let Err(e) = self.window.emit("telemetry_update", payload.clone()) {
                        println!("Failed to emit telemetry update: {}", e);
                        // You might want to return the error or handle it appropriately
                    }
                    println!("Received telemetry data: {:?}", payload);
                    delivery.ack(BasicAckOptions::default()).await?;
                } else {
                    delivery.reject(BasicRejectOptions::default()).await?;
                    println!("Failed to parse Telemetry data")
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
    let consumer = RabbitMQConsumer::new("amqp://guest:guest@localhost:5672/%2f", window.clone())
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
