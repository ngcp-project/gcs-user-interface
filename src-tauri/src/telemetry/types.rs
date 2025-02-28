// #[derive(Debug)]
// #[taurpc::ipc_type]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;



#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TelemetryData {    
    pub vehicle_id: String, // Added vehicle_id
    pub localIP: String,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub speed: f32,
    pub altitude: f32,
    pub batteryLife: i32,
    pub currentPosition: Coordinate,
    pub lastUpdated: f32,
    pub fireFound: bool,
    pub fireCoordinate: Coordinate,
    pub vehicleStatus: String ,

}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Coordinate{
    pub latitude: f32,
    pub longitude: f32,
}

pub struct AppData{
    pub telemetry: HashMap<String, TelemetryData>,
}
