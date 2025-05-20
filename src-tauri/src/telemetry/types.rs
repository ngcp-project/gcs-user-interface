// #[derive(Debug)]
// #[taurpc::ipc_type]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[taurpc::ipc_type]
#[derive(Debug, Default)]
#[allow(non_snake_case)]

pub struct VehicleTelemetryData {
    pub ERU: TelemetryData,
    pub MEA: TelemetryData,
    pub MRA: TelemetryData,
}

impl VehicleTelemetryData {
    pub fn update_vehicle_telemetry_state(
        &mut self,
        vehicle_id: String,
        telemetry_data: TelemetryData,
    ) {
        match vehicle_id.as_str() {
            "eru" => self.ERU = telemetry_data,
            "mea" => self.MEA = telemetry_data,
            "mra" => self.MRA = telemetry_data,
            _ => {}
        }
    }
}


#[taurpc::ipc_type]
#[derive(Debug)]
#[derive(Default)]
pub struct TelemetryData {
    pub vehicle_id: String, // Added vehicle_id
    // pub localIP: String
    pub signal_strength: i32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub speed: f32,
    pub altitude: f32,
    pub battery_life: i32, //f32
    pub current_position: Coordinate,
    // pub last_updated: SystemTime, //what is the exact format we have to follow timestamp 8 bytes
    // pub fire_found: bool,
    // pub fire_coordinate: Coordinate,
    pub vehicle_status: String,
    pub request_coordinate: RequestCoordinate,
}#[taurpc::ipc_type]
//Change vehicleStatus : i8 1 byte 0 - 255
#[derive(Debug)]
#[derive(Default)]
pub struct RequestCoordinate {
    pub message_flag: i32,
    pub request_location: Coordinate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_secured: Option<bool>,
}
#[taurpc::ipc_type]
#[derive(Debug)]
#[derive(Default)]
//change this to f64
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}
#[taurpc::ipc_type]
#[derive(Debug)]
#[derive(Default)]
pub struct AppData {
    pub telemetryx: HashMap<String, TelemetryData>,
}
