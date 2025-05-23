use crate::telemetry::geos;
use crate::telemetry::geos::*;

use crate::telemetry::types::{TelemetryData, VehicleTelemetryData};
use futures_util::stream::StreamExt;
use lapin::{
    options::*, types::FieldTable, Channel, Connection, ConnectionProperties, Consumer,
     Queue, Result as LapinResult,
};
use serde_json::json;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use taurpc;
use tokio::sync::Mutex;
use tokio_amqp::*;

use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use super::sql::*;


#[derive(Clone)]
pub struct RabbitMQAPIImpl {
    connection: Arc<Mutex<Connection>>,
    state: Arc<Mutex<VehicleTelemetryData>>,
    channel: Channel,
    db: PgPool,
    app_handle: Option<AppHandle>,
}

// Constants
const RABBITMQ_ADDR: &str = "amqp://admin:admin@localhost:5672/%2f";
const DATABASE_URL: &str = "postgres://ngcp:ngcp@localhost:5433/ngcpdb";
const VALID_VEHICLE_IDS: [&str; 4] = ["eru", "fra", "mea", "mra"];

impl RabbitMQAPIImpl {
    pub async fn new() -> LapinResult<Self> {
        let connection =
            Connection::connect(RABBITMQ_ADDR, ConnectionProperties::default().with_tokio())
                .await?;

        let connection = Arc::new(Mutex::new(connection));
        let channel = connection.lock().await.create_channel().await?;

        let database_connection = PgPoolOptions::new()
            .max_connections(5)
            .connect(DATABASE_URL)
            .await
            .expect("Failed to connect to the database");
        let db = database_connection;

        let consumer = Self {
            connection,
            channel,
            db,
            state: Arc::new(Mutex::new(VehicleTelemetryData::default())),
            app_handle: None,
        };

        Ok(consumer)
    }

    // Method to set the app handle after initialization
    pub fn with_app_handle(mut self, app_handle: AppHandle) -> Self {
        self.app_handle = Some(app_handle);
        self
    }

    // Initialize all consumers
    pub async fn init_consumers(&self) -> LapinResult<()> {
        for vehicle_id in VALID_VEHICLE_IDS.iter() {
            let queue_name = format!("telemetry_{}", vehicle_id);
            println!("Initializing consumer for queue: {}", queue_name);

            // Declare queue first
            self.queue_declare(&queue_name).await?;

            tokio::spawn({
                let consumer = self.clone();
                let queue = queue_name.clone();
                async move {
                    if let Err(e) = consumer.start_consuming(&queue).await {
                        eprintln!("Failed to consume from queue {}: {}", queue, e);
                    }
                }
            });
        }

        Ok(())
    }

    // Declare a queue for the consumer
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
            .await
    }

    // Create a consumer for a specific queue
    pub async fn create_consumer(&self, queue_name: &str) -> LapinResult<Consumer> {
        // Generate unique consumer tag using queue name and timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let consumer_tag = format!("consumer_{}_{}", queue_name, timestamp);

        println!("Creating consumer with tag: {}", consumer_tag);

        self.channel
            .basic_consume(
                queue_name,
                &consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
    }

    // Start consuming from a specific queue
    pub async fn start_consuming(&self, queue_name: &str) -> LapinResult<()> {
        let consumer = self.create_consumer(queue_name).await?;
        self.process_telemetry(consumer).await?;
        Ok(())
    }

    // Process telemetry data from the consumer
    pub async fn process_telemetry(&self, mut consumer: Consumer) -> LapinResult<()> {
        let mut failure_count = 0;

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                match serde_json::from_slice::<TelemetryData>(&delivery.data) {
                    Ok(mut data) => {
                        failure_count = 0; // reset on success

                        if data.signal_strength < -70 {
                            data.vehicle_status = "Bad Connection".to_string();
                        }

                        let point = geos::Coordinate {
                            latitude: data.current_position.latitude,
                            longitude: data.current_position.longitude,
                        };

                        if is_near_keep_out_zone(&data.vehicle_id, &point, 1000.0) {
                            data.vehicle_status = "Approaching restricted area".to_string();
                        }

                        let vehicle_id = data.vehicle_id.clone();
                        self.state
                            .lock()
                            .await
                            .update_vehicle_telemetry_state(vehicle_id.clone(), data.clone());

                        // Create payload for the event
                        let payload = json!({
                            "vehicle_id": vehicle_id,
                            "telemetry": data.clone()
                        });

                        // Emit the telemetry update using TelemetryEventTrigger
                        if let Some(app_handle) = &self.app_handle {
                            let vehicle_telemetry: VehicleTelemetryData =
                                self.state.lock().await.clone();
                            match TelemetryEventTrigger::new(app_handle.clone())
                                .on_updated(vehicle_telemetry)
                            {
                                Ok(_) => {
                                    println!(
                                        "Successfully emitted telemetry update via event trigger"
                                    );
                                }
                                Err(e) => {
                                    println!(
                                        "Failed to emit telemetry update via event trigger: {}",
                                        e
                                    );

                                    // Fallback to regular app_handle emit
                                    if let Err(e) = app_handle.emit("telemetry_update", &payload) {
                                        println!("Failed to emit telemetry update: {}", e);
                                    }
                                }
                            }
                        } else {
                            println!("Warning: No app_handle available to emit telemetry updates");
                        }

                        println!("Received telemetry data: {:?}", payload);
                        println!("Signal status: {:?}", data.vehicle_status);
                        delivery.ack(BasicAckOptions::default()).await?;

                        // Insert telemetry data into the database
                        let current_position_str = serde_json::to_string(&data.current_position).unwrap();
                        let request_coordinate_str = serde_json::to_string(&data.request_coordinate).unwrap();
                    
                        if let Err(e) = insert_telemetry(
                            self.db.clone(),
                            data.vehicle_id.clone(),
                            data.signal_strength,
                            data.pitch,
                            data.yaw,
                            data.roll,
                            data.speed,
                            data.altitude,
                            data.battery_life,
                            current_position_str,
                            data.vehicle_status.clone(),
                            request_coordinate_str,
                        ).await {
                            eprintln!("Failed to insert telemetry data: {}", e);
                        }
                    }
                    Err(e) => {
                        failure_count += 1;
                        println!(
                            "Failed to parse Telemetry data (attempt {}): {}",
                            failure_count, e
                        );
                        println!("Raw payload: {:?}", String::from_utf8_lossy(&delivery.data));
                        delivery.reject(BasicRejectOptions::default()).await?;

                        if failure_count >= 3 {
                            let error_payload = json!({
                                "error": "Failed to establish a connection after 3 invalid messages"
                            });

                            if let Some(app_handle) = &self.app_handle {
                                app_handle.emit("telemetry_error", error_payload).ok();
                            }

                            return Err(lapin::Error::InvalidChannelState(
                                lapin::ChannelState::Closed,
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

// TauRPC trait definition
#[taurpc::procedures(
    event_trigger = TelemetryEventTrigger,
    export_to = "../src/lib/bindings.ts",
    path = "telemetry"
)]
pub trait RabbitMQAPI {
    #[taurpc(event)]
    async fn on_updated(new_data: VehicleTelemetryData);

    // State Management
    async fn get_default_data() -> VehicleTelemetryData;
    async fn get_telemetry() -> VehicleTelemetryData;
}

// Implementation of the TauRPC trait for our API
#[taurpc::resolvers]
impl RabbitMQAPI for RabbitMQAPIImpl {
    async fn get_default_data(self) -> VehicleTelemetryData {
        Self::new().await.unwrap().state.lock().await.clone()
    }

    async fn get_telemetry(self) -> VehicleTelemetryData {
        self.state.lock().await.clone()
    }
}
