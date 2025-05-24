use crate::telemetry::geos;
use crate::telemetry::geos::*;

use crate::telemetry::types::{TelemetryData, VehicleTelemetryData};
use futures_util::stream::StreamExt;
use lapin::{
    options::*, types::FieldTable, Channel, Connection, ConnectionProperties, Consumer, Queue,
    Result as LapinResult,
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use taurpc;
use tokio::sync::Mutex;
use tokio::time::{interval, sleep};
use tokio_amqp::*;

use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use super::sql::*;


#[derive(Clone, Debug)]
pub struct VehicleHeartbeat {
    pub last_seen: Instant,
    pub is_connected: bool,
    pub consecutive_failures: u32,
}

impl VehicleHeartbeat {
    pub fn new() -> Self {
        Self {
            last_seen: Instant::now(),
            is_connected: true,
            consecutive_failures: 0,
        }
    }

    pub fn update(&mut self) {
        self.last_seen = Instant::now();
        self.is_connected = true;
        self.consecutive_failures = 0;
    }

    pub fn is_timeout(&self, timeout_duration: Duration) -> bool {
        self.last_seen.elapsed() > timeout_duration
    }

    pub fn mark_disconnected(&mut self) {
        self.is_connected = false;
        self.consecutive_failures += 1;
    }
}

#[derive(Clone)]
pub struct RabbitMQAPIImpl {
    connection: Arc<Mutex<Connection>>,
    state: Arc<Mutex<VehicleTelemetryData>>,
    channel: Channel,
    db: PgPool,
    app_handle: Option<AppHandle>,
    // Heartbeat tracking
    vehicle_heartbeats: Arc<Mutex<HashMap<String, VehicleHeartbeat>>>,
    heartbeat_timeout: Duration,
    heartbeat_check_interval: Duration,
}

// Constants
const RABBITMQ_ADDR: &str = "amqp://admin:admin@localhost:5672/%2f";
const DATABASE_URL: &str = "postgres://ngcp:ngcp@localhost:5433/ngcpdb";
const VALID_VEHICLE_IDS: [&str; 4] = ["eru", "fra", "mea", "mra"];
const DEFAULT_HEARTBEAT_TIMEOUT_SECS: u64 = 10; // 30 seconds timeout
const DEFAULT_HEARTBEAT_CHECK_INTERVAL_SECS: u64 = 1; // Check every 10 seconds

impl RabbitMQAPIImpl {
    pub async fn new() -> LapinResult<Self> {
        let connection =
            Connection::connect(RABBITMQ_ADDR, ConnectionProperties::default().with_tokio())
                .await?;

        let connection = Arc::new(Mutex::new(connection));
        let channel = connection.lock().await.create_channel().await?;

        // Initialize heartbeat tracking for all valid vehicles
        let mut vehicle_heartbeats = HashMap::new();
        for vehicle_id in VALID_VEHICLE_IDS.iter() {
            vehicle_heartbeats.insert(vehicle_id.to_string(), VehicleHeartbeat::new());
        }

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
            vehicle_heartbeats: Arc::new(Mutex::new(vehicle_heartbeats)),
            heartbeat_timeout: Duration::from_secs(DEFAULT_HEARTBEAT_TIMEOUT_SECS),
            heartbeat_check_interval: Duration::from_secs(DEFAULT_HEARTBEAT_CHECK_INTERVAL_SECS),
        };

        Ok(consumer)
    }

    // Method to set the app handle after initialization
    pub fn with_app_handle(mut self, app_handle: AppHandle) -> Self {
        self.app_handle = Some(app_handle);
        self
    }

    // Method to configure heartbeat settings
    pub fn with_heartbeat_config(mut self, timeout_secs: u64, check_interval_secs: u64) -> Self {
        self.heartbeat_timeout = Duration::from_secs(timeout_secs);
        self.heartbeat_check_interval = Duration::from_secs(check_interval_secs);
        self
    }

    // Initialize all consumers and start heartbeat monitoring
    pub async fn init_consumers(&self) -> LapinResult<()> {
        // Start heartbeat monitor
        self.start_heartbeat_monitor().await;

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

    // Start the heartbeat monitoring task
    async fn start_heartbeat_monitor(&self) {
        let heartbeats = self.vehicle_heartbeats.clone();
        let state = self.state.clone();
        let app_handle = self.app_handle.clone();
        let timeout = self.heartbeat_timeout;
        let check_interval = self.heartbeat_check_interval;

        tokio::spawn(async move {
            let mut interval_timer = interval(check_interval);

            loop {
                interval_timer.tick().await;

                let mut heartbeats_guard = heartbeats.lock().await;
                let mut state_guard = state.lock().await;
                let mut status_changed = false;

                for (vehicle_id, heartbeat) in heartbeats_guard.iter_mut() {
                    if heartbeat.is_timeout(timeout) && heartbeat.is_connected {
                        println!("Vehicle {} heartbeat timeout detected", vehicle_id);
                        heartbeat.mark_disconnected();

                        // Update vehicle status in telemetry data based on vehicle_id
                        match vehicle_id.as_str() {
                            "eru" => {
                                state_guard.ERU.vehicle_status = "Disconnected".to_string();
                                status_changed = true;
                            }
                            "mea" => {
                                state_guard.MEA.vehicle_status = "Disconnected".to_string();
                                status_changed = true;
                            }
                            "mra" => {
                                state_guard.MRA.vehicle_status = "Disconnected".to_string();
                                status_changed = true;
                            }
                            _ => {
                                println!("Unknown vehicle_id: {}", vehicle_id);
                            }
                        }

                        if status_changed {
                            println!(
                                "Vehicle {} marked as disconnected after {} seconds of no data",
                                vehicle_id,
                                timeout.as_secs()
                            );
                        }
                    }
                }

                // If any status changed, emit update
                if status_changed {
                    if let Some(app_handle) = &app_handle {
                        let vehicle_telemetry = state_guard.clone();
                        drop(state_guard); // Release the lock before emitting
                        drop(heartbeats_guard); // Release the lock before emitting

                        // Try to emit via TelemetryEventTrigger first
                        match TelemetryEventTrigger::new(app_handle.clone())
                            .on_updated(vehicle_telemetry.clone())
                        {
                            Ok(_) => {
                                println!("Successfully emitted heartbeat status update via event trigger");
                            }
                            Err(e) => {
                                println!(
                                    "Failed to emit heartbeat status update via event trigger: {}",
                                    e
                                );

                                // Fallback to regular app_handle emit
                                let payload = json!({
                                    "type": "heartbeat_update",
                                    "telemetry": vehicle_telemetry
                                });
                                if let Err(e) = app_handle.emit("telemetry_update", &payload) {
                                    println!("Failed to emit heartbeat status update: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    // Update heartbeat for a vehicle
    async fn update_vehicle_heartbeat(&self, vehicle_id: &str) {
        let mut heartbeats = self.vehicle_heartbeats.lock().await;
        if let Some(heartbeat) = heartbeats.get_mut(vehicle_id) {
            let was_disconnected = !heartbeat.is_connected;
            heartbeat.update();

            if was_disconnected {
                println!(
                    "Vehicle {} reconnected after being disconnected",
                    vehicle_id
                );

                // Update vehicle status back to normal if it was disconnected
                let mut state_guard = self.state.lock().await;
                match vehicle_id {
                    "eru" => {
                        if state_guard.ERU.vehicle_status == "Disconnected" {
                            state_guard.ERU.vehicle_status = "Connected".to_string();
                        }
                    }
                    "mea" => {
                        if state_guard.MEA.vehicle_status == "Disconnected" {
                            state_guard.MEA.vehicle_status = "Connected".to_string();
                        }
                    }
                    "mra" => {
                        if state_guard.MRA.vehicle_status == "Disconnected" {
                            state_guard.MRA.vehicle_status = "Connected".to_string();
                        }
                    }
                    _ => {
                        println!("Unknown vehicle_id for reconnection: {}", vehicle_id);
                    }
                }
            }
        }
    }

    // Get heartbeat status for all vehicles
    pub async fn get_heartbeat_status(&self) -> HashMap<String, VehicleHeartbeat> {
        self.vehicle_heartbeats.lock().await.clone()
    }

    // Check if a specific vehicle is connected
    pub async fn is_vehicle_connected(&self, vehicle_id: &str) -> bool {
        let heartbeats = self.vehicle_heartbeats.lock().await;
        heartbeats
            .get(vehicle_id)
            .map(|h| h.is_connected && !h.is_timeout(self.heartbeat_timeout))
            .unwrap_or(false)
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

                        // Update heartbeat for this vehicle
                        self.update_vehicle_heartbeat(&data.vehicle_id).await;

                        // Existing signal strength check
                        if data.signal_strength < -70 {
                            data.vehicle_status = "Bad Connection".to_string();
                        }

                        // Existing geo-fencing check
                        let point = geos::Coordinate {
                            latitude: data.current_position.latitude,
                            longitude: data.current_position.longitude,
                        };

                        if is_near_keep_out_zone(&data.vehicle_id, &point, 1000.0) {
                            data.vehicle_status = "Approaching restricted area".to_string();
                        }

                        // If vehicle was marked as disconnected but we're receiving data,
                        // and no other critical status is set, mark as connected
                        if data.vehicle_status.is_empty() || data.vehicle_status == "Disconnected" {
                            if self.is_vehicle_connected(&data.vehicle_id).await {
                                data.vehicle_status = "Connected".to_string();
                            }
                        }

                        let vehicle_id = data.vehicle_id.clone();
                        self.state
                            .lock()
                            .await
                            .update_vehicle_telemetry_state(vehicle_id.clone(), data.clone());

                        // Create payload for the event
                        let payload = json!({
                            "vehicle_id": vehicle_id,
                            "telemetry": data.clone(),
                            "timestamp": std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
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
                                        "Successfully emitted telemetry update via event trigger for vehicle: {}",
                                        vehicle_id
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

                        println!("Received telemetry data from {}: {:?}", vehicle_id, payload);
                        println!("Vehicle {} status: {:?}", vehicle_id, data.vehicle_status);
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
                                "error": "Failed to establish a connection after 3 invalid messages",
                                "consecutive_failures": failure_count
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

    // Heartbeat Management
    // async fn get_heartbeat_status() -> HashMap<String, VehicleHeartbeat>;
    // async fn is_vehicle_connected(vehicle_id: String) -> bool;
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

    // async fn get_heartbeat_status(self) -> HashMap<String, VehicleHeartbeat> {
    //     self.get_heartbeat_status().await
    // }

    // async fn is_vehicle_connected(self, vehicle_id: String) -> bool {
    //     self.is_vehicle_connected(&vehicle_id).await
    // }
}
