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
use std::ptr::null;
use std::sync::Arc;
use tauri::{Emitter, Manager, WebviewWindow, Window};
use tokio::sync::Mutex;
use tokio_amqp::*;
// use crate::telemetry::geo::*; // Removed as the `geo` module does not exist
//creating the structure of the rabbitMQ Consumer
#[derive(Clone)]
// TODO: Transform this struct into one that's compatible with TaRPC
// TODO: Struct name = RabbitMQAPIImpl
pub struct RabbitMQConsumer {
    connection: Arc<Mutex<Connection>>,
    // TODO: state : Arc<Mutex<TelemetryType>>,
    statex:  Arc<Mutex<TelemetryData>>,
    channel: Channel,
    window: WebviewWindow,
}


// TODO: impl RabbitMQAPIImpl
// TODO: Adjust new() to have no parameters
// TODO: make addr const variable, remove window parameter
impl RabbitMQConsumer {
        pub async fn new(window: WebviewWindow) -> LapinResult<Self> {
                // Hardcoded RabbitMQ connection address
                const RABBITMQ_ADDR: &str = "amqp://admin:admin@localhost:5672/%2f";
                
                // Hardcoded valid vehicle IDs
                let valid_vehicle_ids = vec!["eru", "fra", "mea", "mra"];
        
        let connection = Connection::connect(
            RABBITMQ_ADDR,
            ConnectionProperties::default().with_tokio(),
        )
        .await?;
        
        let connection = Arc::new(Mutex::new(connection));
        // one single channel for different queues
        let channel = connection.lock().await.create_channel().await?;
        
        let consumer = Self {
            connection,
            channel,
            statex: Arc::new(Mutex::new(TelemetryData::default())),
            window: window.clone(),
        };
        
        // Initialize consumers for all valid vehicle IDs
        for vehicle_id in valid_vehicle_ids {
            // Create the queue name for this specific vehicle
            let queue_name = format!("telemetry_{}", vehicle_id);
            
            // Start consuming for this specific vehicle
            tokio::spawn({
                let consumer = consumer.clone();
                let queue = queue_name.clone();
                let vehicle_id = vehicle_id.to_string();
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
        }
        
        Ok(consumer)
    }

    //#[tauri::command]

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
        // TODO: No need for create_consumer here because it's only being called once
        //Start processing
        self.process_telemetry(consumer).await?;

        Ok(())
    }

    //In this part we should emit to the frontend, ads
    // TODO: contents of emit_state_update() should be here
  pub async fn process_telemetry(&self, mut consumer: Consumer) -> LapinResult<()> {
    use futures_util::StreamExt;

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
                        let mut state = self.statex.lock().await;
                        *state = data.clone();
                    }

                    // Create payload for the event
                    let payload = json!({
                        "vehicle_id": data.vehicle_id,
                        "telemetry": data.clone()
                    });

                    // Emit the telemetry update using TelemetryEventTrigger
                    if let Some(app_handle) = self.window.app_handle().as_ref() {
                        match TelemetryEventTrigger::new(app_handle.clone())
                            .on_updated(data.clone())
                            .await
                        {
                            Ok(_) => {
                                println!("Successfully emitted telemetry update via event trigger");
                            }
                            Err(e) => {
                                println!("Failed to emit telemetry update via event trigger: {}", e);
                                
                                // Fallback to regular window emit if event trigger fails
                                if let Err(e) = self.window.emit("telemetry_update", &payload) {
                                    println!("Failed to emit telemetry update: {}", e);
                                }
                            }
                        }
                    } else {
                        // Fallback if app_handle isn't available
                        if let Err(e) = self.window.emit("telemetry_update", &payload) {
                            println!("Failed to emit telemetry update: {}", e);
                        }
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

                        self.window.emit("telemetry_error", error_payload).ok();

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

    // pub async fn checkKeepOutZone(mut latitude : f32 , mut longitud : f32) -> Boolean {
    //     return true;

    // }
}

// pub async fn lost_vehicle_connection(mut string_signal:f32) -> boolean{
//         if string_signal < -40.0  {
//             string_signal = "Disconnect";
//         }
// }

// pub async
//create function that check if the signal string is inside the range.
// if not just wait n second, pass that we send an alert/ trigger the
// function failedconnectioninit()?? }
//



// TODO: Trait binding for taurpc (taurpc::procedures)
// TODO: pub trait RabbitMQAPI {
#[taurpc::procedures(
    event_trigger = TelemetryEventTrigger,
    export_to = "../src/lib/bindings.ts",
    path = "mission"
)]

// TODO: impl RabbitMQAPI for RabbitMQAPIImpl
// TODO: Functions to add: on_updated(new_data: TelemetryType), get_default_data(), get_telemetry() 
// TODO: get_default_data() => Self::new().await.state.lock().await.clone()
    // TODO: get_default_data() initializes default data 
// TODO: get_telemetry() => self.state.lock().await.clone()
pub trait TelemetryApi{
    #[taurpc(event)]
    async fn on_updated(new_data: TelemetryData);
    // ----------------------------------
    // State Management
    // ----------------------------------
    async fn get_default_data() -> TelemetryData;
    async fn get_telemetry() -> TelemetryData;
}

impl TelemetryApi for RabbitMQConsumer {
    // type get_default_dataFut = std::pin::Pin<Box<dyn std::future::Future<Output = TelemetryData> + Send>>;
    // type get_telemetryFut = std::pin::Pin<Box<dyn std::future::Future<Output = TelemetryData> + Send>>;

    async fn get_default_data(self) -> TelemetryData {
        Self::new(self.window.clone()).await.unwrap().statex.lock().await.clone()
    }

    async fn get_telemetry(self) -> TelemetryData {
        self.statex.lock().await.clone()
    }


    //     async fn get_default_data(self) -> MissionsStruct {
    //     Self::new().await.state.lock().await.clone()
    // }

    // async fn get_all_missions(self) -> MissionsStruct {
    //     self.state.lock().await.clone()
    // }
}

























