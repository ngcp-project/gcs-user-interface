use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{AppHandle, Wry};
use tauri::Emitter;
use taurpc::{procedures, resolvers};
use serde::{Deserialize, Serialize};
use specta::Type;
use lapin::{
    options::{BasicConsumeOptions, QueueDeclareOptions, BasicAckOptions},
    types::FieldTable,
    Connection, ConnectionProperties,
};
use futures_util::StreamExt;
use serde_json::from_slice;

#[derive(Debug, Deserialize, Serialize, Clone, Type)]
pub struct TelemetryStruct {
    pub vehicle_id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub heading: f64,
    pub battery_level: f64,
    pub timestamp: String,
}

type SharedTelemetry = Arc<Mutex<TelemetryStruct>>;

#[procedures(export_to = "../src/lib/telem_bindings.ts", path = "telem")]
pub trait TeleApi {
    async fn get_default_telemetry() -> TelemetryStruct;
}

#[derive(Clone)]
pub struct TelemApiImpl {
    state: SharedTelemetry,
}

impl Default for TelemApiImpl {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(TelemetryStruct {
                vehicle_id: "default".to_string(),
                latitude: 0.0,
                longitude: 0.0,
                heading: 0.0,
                battery_level: 0.0,
                timestamp: "".to_string(),
            })),
        }
    }
}

#[resolvers]
impl TeleApi for TelemApiImpl {
    async fn get_default_telemetry(self) -> TelemetryStruct {
        let guard = self.state.lock().await;
        (*guard).clone()
    }
}

impl TelemApiImpl {
    pub async fn start_telemetry_listener(&self, app_handle: AppHandle<Wry>) -> Result<(), String> {
        let addr = std::env::var("AMQP_TELEM_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .map_err(|e| e.to_string())?;
        let channel = conn.create_channel()
            .await
            .map_err(|e| e.to_string())?;
        channel
            .queue_declare("vehicle_telemetry", QueueDeclareOptions::default(), FieldTable::default())
            .await
            .map_err(|e| e.to_string())?;
        let mut consumer = channel
            .basic_consume(
                "vehicle_telemetry",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| e.to_string())?;

        tokio::spawn({
            let shared = self.state.clone();
            let app_handle = app_handle.clone();
            async move {
                while let Some(delivery_result) = consumer.next().await {
                    if let Ok(delivery) = delivery_result {
                        if let Ok(telemetry_msg) = from_slice::<TelemetryStruct>(&delivery.data) {
                            println!("Received telemetry: {:?}", telemetry_msg);
                            {
                                let mut state = shared.lock().await;
                                *state = telemetry_msg.clone();
                            }
                            if let Err(e) = app_handle.emit("telemetry_updated", telemetry_msg) {
                                eprintln!("emit failed: {}", e);
                            }
                            if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                                eprintln!("ack failed: {}", e);
                            }
                        }
                    }
                }
                if let Err(e) = conn.close(0, "").await {
                    eprintln!("connection close failed: {}", e);
                }
            }
        });
        Ok(())
    }
}