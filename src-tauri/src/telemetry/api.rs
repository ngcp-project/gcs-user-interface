// use crate::telemetry::types::{AppData, TelemetryData, Coordinate};
// use tauri::{State, Window};
// use tokio::sync::Mutex;
// use tokio::time::{interval, Duration};
// use rand::{Rng, SeedableRng, rngs::StdRng};
// use std::sync::Arc;

// #[tauri::command]
// pub async fn get_telemetry(
//     vehicle_id: String,
//     app_data: State<'_, Arc<Mutex<AppData>>>,
//     window: Window
// ) -> Result<TelemetryData, String> {
//     let app_data_clone = app_data.inner().clone();
//     let vehicle_id_clone = vehicle_id.clone();
//     let window_clone = window.clone();

//     tokio::spawn(async move {
//         let mut rng = StdRng::from_entropy();
//         let mut interval = interval(Duration::from_secs(1));

//         loop {
//             interval.tick().await;
//             let mut data = app_data_clone.lock().await;

//             if let Some(telemetry) = data.telemetry.get_mut(&vehicle_id_clone) {
//                 // Randomize telemetry values
//                 telemetry.speed += rng.gen_range(0.5..2.0);
//                 telemetry.altitude += rng.gen_range(-1.0..1.0);
//                 telemetry.pitch += rng.gen_range(-0.5..0.5);
//                 telemetry.yaw += rng.gen_range(-1.0..1.0);
//                 telemetry.roll += rng.gen_range(-0.5..0.5);
//                 telemetry.lastUpdated = rng.gen_range(0.0..100.0);
//                 telemetry.batteryLife += rng.gen_range(-2..1);
//                 telemetry.currentPosition.latitude += rng.gen_range(-1.0..3.0);
//                 telemetry.currentPosition.longitude += rng.gen_range(-1.0..3.5);
                

//                 let payload = serde_json::json!(telemetry);

//                 if let Err(e) = window_clone.emit("telemetry_update", payload) {
//                     eprintln!("Failed to emit telemetry update: {}", e);
//                 }
//             }
//         }
//     });
//     let data = app_data.lock().await;
//     if let Some(telemetry) = data.telemetry.get(&vehicle_id) {
//         let payload = serde_json::json!(telemetry);
//         window.emit("telemetry_update", payload).map_err(|e| e.to_string())?;
//         Ok((telemetry.clone()))
//     } else {
//         Err("Vehicle not found".to_string())
//     }
// }

// pub fn initialize_telemetry_data() -> AppData {
//     let mut telemetry = std::collections::HashMap::new();

//     telemetry.insert(
//         "eru".to_string(),
//         TelemetryData {
//             vehicle_id: "eru".to_string(),
//             localIP: "192.168.1.1".to_string(),
//             pitch: 0.0,
//             yaw: 0.0,
//             roll: 0.0,
//             speed: 0.0,
//             altitude: 0.0,
//             batteryLife: 100,
//             currentPosition: Coordinate {
//                 latitude: 0.0,
//                 longitude: 0.0,
//             },
//             lastUpdated: 0.0,
//             fireFound: false,
//             fireCoordinate: Coordinate {
//                 latitude: 0.0,
//                 longitude: 0.0,
//             },
//             vehicleStatus: "Standby".to_string(),
//         },
//     );

//     telemetry.insert(
//         "mea".to_string(),
//         TelemetryData {
//             vehicle_id: "mea".to_string(),
//             localIP: "192.168.1.2".to_string(),
//             pitch: 0.0,
//             yaw: 0.0,
//             roll: 0.0,
//             speed: 0.0,
//             altitude: 0.0,
//             batteryLife: 100,
//             currentPosition: Coordinate {
//                 latitude: 0.0,
//                 longitude: 0.0,
//             },
//             lastUpdated: 0.0,
//             fireFound: false,
//             fireCoordinate: Coordinate {
//                 latitude: 0.0,
//                 longitude: 0.0,
//             },
//             vehicleStatus: "Standby".to_string(),
//         },
//     );

//     telemetry.insert(
//         "mra".to_string(),
//         TelemetryData {
//             vehicle_id: "mra".to_string(),
//             localIP: "192.168.1.3".to_string(),
//             pitch: 0.0,
//             yaw: 0.0,
//             roll: 0.0,
//             speed: 0.0,
//             altitude: 0.0,
//             batteryLife: 100,
//             currentPosition: Coordinate {
//                 latitude: 0.0,
//                 longitude: 0.0,
//             },
//             lastUpdated: 0.0,
//             fireFound: false,
//             fireCoordinate: Coordinate {
//                 latitude: 0.0,
//                 longitude: 0.0,
//             },
//             vehicleStatus: "Standby".to_string(),
//         },
//     );

//     telemetry.insert(
//         "fra".to_string(),
//         TelemetryData {
//             vehicle_id: "fra".to_string(),
//             localIP: "192.168.1.4".to_string(),
//             pitch: 0.0,
//             yaw: 0.0,
//             roll: 0.0,
//             speed: 0.0,
//             altitude: 0.0,
//             batteryLife: 100,
//             currentPosition: Coordinate {
//                 latitude: 0.0,
//                 longitude: 0.0,
//             },
//             lastUpdated: 0.0,
//             fireFound: false,
//             fireCoordinate: Coordinate {
//                 latitude: 0.0,
//                 longitude: 0.0,
//             },
//             vehicleStatus: "Standby".to_string(),
//         },
//     );

//     AppData { telemetry }
// }