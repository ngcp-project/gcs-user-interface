use crate::telemetry::geos;
use crate::telemetry::geos::*;
use crate::telemetry::types::TelemetryData;
use crate::telemetry::types::{AppData, Coordinate};
use futures_util::stream::StreamExt;
use lapin::{
    options::*, types::FieldTable, Channel, Connection, ConnectionProperties, Consumer, Error as LapinError, Queue,
    Result as LapinResult,
};
use serde_json::json;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Mutex;
use tokio_amqp::*;

// Renamed to RabbitMQAPIImpl as suggested in TODO
#[derive(Clone)]
pub struct RabbitMQAPIImpl {
    connection: Arc<Mutex<Connection>>,
    state: Arc<Mutex<TelemetryData>>,
    channel: Channel,
    app_handle: Option<AppHandle>,
}

// Constants
const RABBITMQ_ADDR: &str = "amqp://admin:admin@localhost:5672/%2f";
const VALID_VEHICLE_IDS: [&str; 4] = ["eru", "fra", "mea", "mra"];

impl RabbitMQAPIImpl {
    pub async fn new() -> LapinResult<Self> {
        let connection = Connection::connect(
            RABBITMQ_ADDR,
            ConnectionProperties::default().with_tokio(),
        )
        .await?;
        
        let connection = Arc::new(Mutex::new(connection));
        let channel = connection.lock().await.create_channel().await?;
        
        let consumer = Self {
            connection,
            channel,
            state: Arc::new(Mutex::new(TelemetryData::default())),
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
            
            tokio::spawn({
                let consumer = self.clone();
                let queue = queue_name.clone();
                let vehicle_id = vehicle_id.to_string();
                async move {
                    if let Err(e) = consumer.start_consuming(&queue).await {
                        eprintln!("Failed to consume from queue {}: {}", queue, e);
                        
                        // Emit error if app_handle is available
                        if let Some(app_handle) = &consumer.app_handle {
                            let _ = app_handle.emit(
                                "telemetry_error",
                                json!({
                                    "vehicle_id": vehicle_id,
                                    "error": e.to_string()
                                }),
                            );
                        }
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
        self.channel
            .basic_consume(
                queue_name,
                "telemetry_consumer",
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

                        if data.signal_string < -70 {
                            data.vehicle_status = "Bad Connection".to_string();
                        }
                        
                        let point = geos::Coordinate {
                            latitude: data.current_position.latitude,
                            longitude: data.current_position.longitude,
                        };
                        
                        if is_near_keep_out_zone(&data.vehicle_id, &point, 1000.0) {
                            data.vehicle_status = "Approaching restricted area".to_string();
                        }

                        // Update the internal state
                        {
                            let mut state = self.state.lock().await;
                            *state = data.clone();
                        }

                        // Create payload for the event
                        let payload = json!({
                            "vehicle_id": data.vehicle_id,
                            "telemetry": data.clone()
                        });

                        // Emit the telemetry update using TelemetryEventTrigger
                        if let Some(app_handle) = &self.app_handle {
                            match TelemetryEventTrigger::new(app_handle.clone())
                                .on_updated(data.clone())
                                
                            {
                                Ok(_) => {
                                    println!("Successfully emitted telemetry update via event trigger");
                                }
                                Err(e) => {
                                    println!("Failed to emit telemetry update via event trigger: {}", e);
                                    
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
    path = "mission"
)]
pub trait RabbitMQAPI {
    #[taurpc(event)]
    async fn on_updated(new_data: TelemetryData);
    
    // State Management
    async fn get_default_data() -> TelemetryData;
    async fn get_telemetry() -> TelemetryData;
}

// Implementation of the TauRPC trait for our API
impl RabbitMQAPI for RabbitMQAPIImpl {
    
    type get_default_dataFut = std::pin::Pin<Box<dyn std::future::Future<Output = TelemetryData> + Send>>;
    type get_telemetryFut = std::pin::Pin<Box<dyn std::future::Future<Output = TelemetryData> + Send>>;

   fn get_default_data(self) -> Self::get_default_dataFut {
        Box::pin(async move {
            Self::new().await.unwrap().state.lock().await.clone()
        })
    }


    
    fn get_telemetry(self) -> Self::get_telemetryFut {
        Box::pin(async move {
            self.state.lock().await.clone()
        })
    }
}

// Function to setup and initialize the RabbitMQ API implementation
// pub async fn setup_rabbitmq_api(app_handle: AppHandle) -> Result<RabbitMQAPIImpl, String> {
//     let api_impl = RabbitMQAPIImpl::new()
//         .await
//         .map_err(|e| format!("Failed to create RabbitMQ API: {}", e))?;
    
//     let api_with_handle = api_impl.with_app_handle(app_handle);
    
//     api_with_handle.init_consumers()
//         .await
//         .map_err(|e| format!("Failed to initialize consumers: {}", e))?;
        
//     Ok(api_with_handle)
// }