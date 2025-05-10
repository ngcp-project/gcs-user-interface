use std::time::{Duration, SystemTime};

use crate::telemetry::types::{Coordinate, RequestCoordinate, TelemetryData};
// use window::Window;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    Result as LapinResult,
};
use serde_json::json;
use tokio::runtime::Runtime;

use crate::telemetry::geos::*;

pub struct RabbitMQPublisher {
    // connection:Connection,
    channel: Channel,
    // window:Window
}

impl RabbitMQPublisher {
    pub async fn new(addr: &str) -> LapinResult<Self> {
        let connection = Connection::connect(addr, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        Ok(Self { channel })
    }
    //
    pub async fn publish_telemetry(
        &self,
        name_of_vehicle: &str,
        telemetry: TelemetryData,
    ) -> LapinResult<()> {
        let queue_name = format!("telemetry_{}", name_of_vehicle);
        self.channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions {
                    durable: true,
                    auto_delete: false,
                    exclusive: false,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        let payload = serde_json::to_vec(&telemetry)
            .map_err(|e| lapin::Error::from(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        self.channel
            .basic_publish(
                "",
                &queue_name,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default()
                    .with_content_type("application/json".into())
                    .with_delivery_mode(2),
            )
            .await?;

        println!("Published telemetry data for vehicle {}", name_of_vehicle);
        Ok(())
    }
}

// pub struct TelemetryData {
//     pub vehicle_id: String, // Added vehicle_id
//     // pub localIP: String,
//     pub pitch: f32,
//     pub yaw: f32,
//     pub roll: f32,
//     pub speed: f32,
//     pub altitude: f32,
//     pub battery_life: i32, //f32
//     pub current_position: Coordinate,
//     pub last_updated: SystemTime, //what is the exact format we have to follow timestamp 8 bytes
//     pub fire_found: bool,
//     pub fire_coordinate: Coordinate,
//     pub vehicle_status: i8,
//     pub request_coordinate: RequestCoordinate,
// }

// //#[derive(Clone, serde::Serialize, serde::Deserialize)]
// pub struct RequestCoordinate{
//     pub message_flag: i32,
//     pub request_location: Coordinate,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub patient_secured: Option<bool>
// }

pub async fn test_publisher() -> Result<(), Box<dyn std::error::Error>> {
    let publisher = RabbitMQPublisher::new("amqp://guest:guest@localhost:5672/%2f").await?;
    let vehicle_ids = vec!["eru", "fra", "mea", "mra"];
    for _ in 0..20 {
        for vehicle_id in &vehicle_ids {
            let data = TelemetryData {
                vehicle_id: vehicle_id.to_string(),
                signal_string: rand::random::<i32>() % 70 + 30,
                pitch: rand::random::<f32>() * 100.0,
                yaw: rand::random::<f32>() * 100.0,
                roll: rand::random::<f32>() * 100.0,
                speed: rand::random::<f32>() * 100.0,
                altitude: rand::random::<f32>() * 100.0,
                battery_life: rand::random::<f32>() * 100.0,
                current_position: Coordinate {
                    latitude: rand::random::<f64>() * 100.0,
                    longitude: rand::random::<f64>() * 100.0,
                },
                last_updated: SystemTime::now(),
                vehicle_status: "something".to_string(),
                request_coordinate: RequestCoordinate {
                    message_flag: rand::random::<i32>(),
                    request_location: Coordinate {
                        latitude: rand::random::<f64>() * 100.0,
                        longitude: rand::random::<f64>() * 100.0,
                    },
                    patient_secured: Some(rand::random()),
                },
            };
            publisher.publish_telemetry(vehicle_id, data).await?;
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }
    println!("test complete");
    Ok(())
}

// async fn publish_mq_commands(vehicle_id: &str, command: &str) -> Result<(), Box<dyn std::error::Error>> {
//     // Connect to RabbitMQ server
//     let uri = "amqp://guest:guest@localhost:5672/%2f";
//     let conn = Connection::connect(uri, ConnectionProperties::default()).await?;

//     // Create a channel
//     let channel = conn.create_channel().await?;

//     // Declare the queue for the specific vehicle
//     let queue_name = format!("telemetry_{}", vehicle_id);
//     channel.queue_declare(&queue_name, QueueDeclareOptions::default(), FieldTable::default()).await?;

//     // Create telemetry data JSON
//     let telemetry_data = json!({
//         "telemetryData": {
//             "pitch": 10.5,
//             "yaw": 20.3,
//             "roll": 5.8,
//             "speed": 45.2,
//             "altitude": 1000.0,
//             "batteryLife": 80.5,
//             "currentPosition": {
//                 "latitude": 37.7749,
//                 "longitude": -122.4194
//             },
//             "lastUpdated": 1741988689454,
//             "requestCoord": {
//                 "messageFlag": 1,
//                 "requestLocation": {
//                     "latitude": 45.8484,
//                     "longitude": 100.4194
//                 },
//                 "patientSecured": null
//             }
//         }
//     });

//     // Combine command and telemetry data
//     let message = json!({
//         "command": command,
//         "telemetryData": telemetry_data["telemetryData"]
//     });

//     // Publish the combined message to the queue
//     channel.basic_publish(
//         "", // Use the default exchange
//         &queue_name,
//         BasicPublishOptions::default(),
//         message.to_string().as_bytes().to_vec(),
//         BasicProperties::default(),
//     ).await?;

//     println!("Published command '{}' with telemetry data to queue '{}'", command, queue_name);

//     Ok(())
// }

// fn main() {
//     let rt = Runtime::new().unwrap();
//     rt.block_on(async {

//         let vehicles = vec!["eru", "fra", "mea", "mra"];
//         // let commands = vec!["command1", "command2", "command3", "command4"];

//         for (i, vehicle) in vehicles.iter().enumerate() {
//             if let Err(e) = publish_mq_commands(vehicle, commands[i]).await {
//                 eprintln!("Error publishing command: {}", e);
//             }
//         }
//     });
// }
