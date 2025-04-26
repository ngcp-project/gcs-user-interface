use std::sync::Arc;
use tokio::sync::Mutex;
use taurpc::{procedures, resolvers};
use serde::{Deserialize, Serialize};
use specta::Type;
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties, BasicProperties,
};
use futures_util::StreamExt;

#[derive(Debug, Deserialize, Serialize, Clone, Type)]
pub struct CommandsStruct {
    pub vehicle_id: String,
    pub command: String,
    pub value: f64,
}

type SharedCommands = Arc<Mutex<CommandsStruct>>;

#[procedures(export_to = "../src/lib/commands_bindings.ts", path = "commands")]
pub trait CommandsApi {
    async fn send_emergency_stop(vehicle_id: String) -> Result<(), String>;
    async fn send_mission_update(vehicle_id: String, mission_id: String) -> Result<(), String>;
    async fn send_zone_update(vehicle_id: String, zone_id: String) -> Result<(), String>;
}

#[derive(Clone)]
pub struct CommandsApiImpl {
    state: SharedCommands,
}

impl Default for CommandsApiImpl {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(CommandsStruct {
                vehicle_id: "default".to_string(),
                command: "".to_string(),
                value: 0.0,
            })),
        }
    }
}

#[resolvers]
impl CommandsApi for CommandsApiImpl {
    async fn send_emergency_stop(self, vehicle_id: String) -> Result<(), String> {
        let mut state = self.state.lock().await;
        state.vehicle_id = vehicle_id;
        state.command = "emergency_stop".to_string();
        state.value = 1.0;
        self.publish_command_to_rabbitmq(&state).await?;
        Ok(())
    }

    async fn send_mission_update(self, vehicle_id: String, mission_id: String) -> Result<(), String> {
        let mut state = self.state.lock().await;
        state.vehicle_id = vehicle_id;
        state.command = "mission_update".to_string();
        state.value = mission_id.parse().unwrap_or(0.0);
        self.publish_command_to_rabbitmq(&state).await?;
        Ok(())
    }

    async fn send_zone_update(self, vehicle_id: String, zone_id: String) -> Result<(), String> {
        let mut state = self.state.lock().await;
        state.vehicle_id = vehicle_id;
        state.command = "zone_update".to_string();
        state.value = zone_id.parse().unwrap_or(0.0);
        self.publish_command_to_rabbitmq(&state).await?;
        Ok(())
    }
}

impl CommandsApiImpl {
    async fn publish_command_to_rabbitmq(&self, command: &CommandsStruct) -> Result<(), String> {
        // 1) Use %2f to select the "/" vhost
        let addr = std::env::var("AMQP_ADDR")
            .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672/%2f".into());
        println!("→ Connecting to RabbitMQ at {}", addr);
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .map_err(|e| format!("Failed to connect to RabbitMQ: {}", e))?;
        println!("→ Connected");

        // 2) Open channel
        let channel = conn
            .create_channel()
            .await
            .map_err(|e| format!("Failed to create channel: {}", e))?;
        println!("Created channel");

        // 3) Declare queue (durable)
        let queue = channel
            .queue_declare(
                "vehicle_commands",
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| format!("Failed to declare queue: {}", e))?;
        println!("Queue 'vehicle_commands' ready count = {}", queue.message_count());

        // 4) Serialize & publish to default exchange
        let payload = serde_json::to_vec(command)
            .map_err(|e| format!("Failed to serialize command: {}", e))?;
        println!("Serialized command: {:?}", command);

        let confirm = channel
            .basic_publish(
                "",
                "vehicle_commands",
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await
            .map_err(|e| format!("Failed to publish: {}", e))?;
        confirm
            .await
            .map_err(|e| format!("Publish confirm failed: {}", e))?;
        println!("Published command via default exchange");

        // 5) One-shot debug: basic_get
        match channel.basic_get("vehicle_commands", Default::default()).await {
            Ok(Some(get)) => {
                let body = String::from_utf8_lossy(&get.data);
                println!("🎉 basic_get got: {}", body);
                get.ack(lapin::options::BasicAckOptions::default())
                    .await
                    .map_err(|e| format!("Ack failed: {}", e))?;
            }
            Ok(None) => println!("⚠️ basic_get saw no messages"),
            Err(e) => println!("❌ basic_get error: {}", e),
        }

        // 6) Close
        conn.close(0, "")
            .await
            .map_err(|e| format!("Failed to close connection: {}", e))?;
        println!("Closed connection");

        Ok(())
    }
}
