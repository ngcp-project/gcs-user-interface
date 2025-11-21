
use std::collections::HashMap;

#[taurpc::ipc_type]
#[derive(Debug)]
#[allow(non_snake_case)]

pub struct VehicleTelemetryData {
    pub ERU: TelemetryData,
    pub MEA: TelemetryData,
    pub MRA: TelemetryData,
}

impl Default for VehicleTelemetryData {
    fn default() -> Self {
        let default_coords = Coordinate {
            latitude: 33.932573934575075,
            longitude: -117.63059569114814,
        };
        
        Self {
            ERU: TelemetryData {
                vehicle_id: "eru".to_string(),
                signal_strength: 0,
                pitch: 0.0,
                yaw: 0.0,
                roll: 0.0,
                speed: 0.0,
                altitude: 0.0,
                battery_life: 0,
                current_position: default_coords.clone(),
                vehicle_status: "".to_string(),
                request_coordinate: RequestCoordinate {
                    message_flag: 0,
                    request_location: default_coords.clone(),
                    patient_secured: None,
                },
            },
            MEA: TelemetryData {
                vehicle_id: "mea".to_string(),
                signal_strength: 0,
                pitch: 0.0,
                yaw: 0.0,
                roll: 0.0,
                speed: 0.0,
                altitude: 0.0,
                battery_life: 0,
                current_position: default_coords.clone(),
                vehicle_status: "".to_string(),
                request_coordinate: RequestCoordinate {
                    message_flag: 0,
                    request_location: default_coords.clone(),
                    patient_secured: None,
                },
            },
            MRA: TelemetryData {
                vehicle_id: "mra".to_string(),
                signal_strength: 0,
                pitch: 0.0,
                yaw: 0.0,
                roll: 0.0,
                speed: 0.0,
                altitude: 0.0,
                battery_life: 0,
                current_position: default_coords.clone(),
                vehicle_status: "".to_string(),
                request_coordinate: RequestCoordinate {
                    message_flag: 0,
                    request_location: default_coords.clone(),
                    patient_secured: None,
                },
            },
        }
    }
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
#[derive(Debug, Default)]
pub struct TelemetryData {
    pub vehicle_id: String, // Added vehicle_id
    pub signal_strength: i32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub speed: f32,
    pub altitude: f32,
    pub battery_life: i32, //f32
    pub current_position: Coordinate,
    pub vehicle_status: String,
    pub request_coordinate: RequestCoordinate,
}
#[taurpc::ipc_type]
//Change vehicleStatus : i8 1 byte 0 - 255
#[derive(Debug, Default)]
pub struct RequestCoordinate {
    pub message_flag: i32,
    pub request_location: Coordinate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_secured: Option<bool>,
}
#[taurpc::ipc_type]
#[derive(Debug, Default)]
//change this to f64
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}
#[taurpc::ipc_type]
#[derive(Debug, Default)]
pub struct AppData {
    pub telemetryx: HashMap<String, TelemetryData>,
}
