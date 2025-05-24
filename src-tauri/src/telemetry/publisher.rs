use std::time::{Duration};
use crate::telemetry::{sql::insert_telemetry, types::{Coordinate, RequestCoordinate, TelemetryData}};
// use window::Window;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    Result as LapinResult,
};
use rand::Rng;
use sqlx::{postgres::PgPoolOptions, PgPool, Row};

pub struct RabbitMQPublisher {
    channel: Channel,
}

impl RabbitMQPublisher {
    pub async fn new(addr: &str) -> LapinResult<Self> {
        let connection = Connection::connect(addr, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        Ok(Self { channel })
    }

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



pub async fn test_publisher() -> Result<(), Box<dyn std::error::Error>> {
    let publisher = RabbitMQPublisher::new("amqp://admin:admin@localhost:5672/%2f").await?;
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://ngcp:ngcp@localhost:5433/ngcpdb")
        .await
        .expect("Failed to connect to the database");
    let vehicle_ids = vec!["eru", "fra", "mea", "mra"];
    let vehicle_statuses = vec!["IN USE", "STANDBY", "EMERGENCY STOP"];
    for _ in 0..20 {
        for vehicle_id in &vehicle_ids {
            let data = TelemetryData {
                vehicle_id: vehicle_id.to_string(),
                signal_strength: rand::random::<i32>() % 70 + 30,
                pitch: rand::random::<f32>() * 100.0,
                yaw: rand::random::<f32>() * 100.0,
                roll: rand::random::<f32>() * 100.0,
                speed: rand::random::<f32>() * 100.0,
                altitude: rand::random::<f32>() * 100.0,
                battery_life: rand::random::<i32>() % 40 + 20,
                current_position: Coordinate {
                    latitude: 33.932573934575075 + rand::random::<f64>() * 0.01,
                    longitude: -117.63059569114814 + rand::random::<f64>() * 0.01,
                },
                // last_updated: SystemTime::now(),
                // vehicle_status: "something".to_string(),
                vehicle_status: vehicle_statuses[rand::rng().random_range(0..vehicle_statuses.len())].to_string(),
                request_coordinate: RequestCoordinate {
                    message_flag: rand::random::<i32>(),
                    request_location: Coordinate {
                        latitude: rand::random::<f64>() * 100.0,
                        longitude: rand::random::<f64>() * 100.0,
                    },
                    patient_secured: Some(rand::random()),
                },
            };

            let current_position_str = serde_json::to_string(&data.current_position).unwrap();
            let request_coordinate_str = serde_json::to_string(&data.request_coordinate).unwrap();
        
            insert_telemetry(
                db.clone(),
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
            ).await?;
            
            publisher.publish_telemetry(vehicle_id, data).await?;
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }
    println!("test complete");
    Ok(())
}


