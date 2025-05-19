use crate::telemetry::geos;
use crate::telemetry::geos::*;
use crate::telemetry::types::TelemetryData;
use crate::telemetry::types::{AppData, Coordinate};
use futures_util::stream::StreamExt;
use lapin::{
    options::*, types::FieldTable, Channel, Connection, ConnectionProperties, Consumer, Queue,
    Result as LapinResult,
};
use serde_json::json;
use std::ptr::null;
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow, Window};
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
    channel: Channel,
    window: WebviewWindow,
}

// impl RabbitMQConsumer{
//     self.connection =
// }


// TODO: impl RabbitMQAPIImpl
// TODO: Adjust new() to have no parameters
// TODO: make addr const variable, remove window parameter
impl RabbitMQConsumer {
    pub async fn new(addr: &str, window: WebviewWindow) -> LapinResult<Self> {
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
                        // if(checkKeepOutZone(data.current_position.latitude && data.current_position.longitude)){
                        //     data.vehicle_status = "Dont go furthe,leave the zone".to_String();
                        // }

                        let payload = json!({
                            "vehicle_id": data.vehicle_id,
                            "telemetry": data
                        });

                        if let Err(e) = self.window.emit("telemetry_update", payload.clone()) {
                            println!("Failed to emit telemetry update: {}", e);
                        }
                        // TODO: Instead of window.emit, use contents of emit_state_update()

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
                            )); // or another appropriate variant
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


// TODO: impl RabbitMQAPI for RabbitMQAPIImpl
// TODO: Functions to add: on_updated(new_data: TelemetryType), get_default_data(), get_telemetry() 
// TODO: get_default_data() => Self::new().await.state.lock().await.clone()
    // TODO: get_default_data() initializes default data 
// TODO: get_telemetry() => self.state.lock().await.clone()

// TODO: Move init_telemetry_consumer to be within new()
#[tauri::command]
pub async fn init_telemetry_consumer(
    window: WebviewWindow,
    vehicle_id: String,
) -> Result<(), String> {
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
