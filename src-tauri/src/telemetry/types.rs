// #[derive(Debug)]
// #[taurpc::ipc_type]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Clone, serde::Serialize, serde::Deserialize)]

pub struct TelemetryData {
    pub vehicle_id: String, // Added vehicle_id
    // pub localIP: String
    pub signal_string: i32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub speed: f32,
    pub altitude: f32,
    pub battery_life: i32, //f32
    pub current_position: Coordinate,
    pub last_updated: SystemTime, //what is the exact format we have to follow timestamp 8 bytes
    // pub fire_found: bool,
    // pub fire_coordinate: Coordinate,
    pub vehicle_status: String,
    pub request_coordinate: RequestCoordinate,
}
//Change vehicleStatus : i8 1 byte 0 - 255
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestCoordinate {
    pub message_flag: i32,
    pub request_location: Coordinate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_secured: Option<bool>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
//change this to f64
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

pub struct AppData {
    pub telemetry: HashMap<String, TelemetryData>,
}
