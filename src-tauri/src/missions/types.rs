use std::{num::ParseFloatError, str::FromStr};

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct MissionsStruct {
    pub current_mission: i32, // -1 for no mission
    pub missions: Vec<MissionStruct>,
}

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct MissionStruct {
    pub mission_name: String,
    pub mission_id: i32,
    pub mission_status: MissionStageStatusEnum,
    pub vehicles: VehiclesStruct,
    pub zones: ZonesStruct,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type, sqlx::Type)]
#[sqlx(type_name = "status")]
pub enum MissionStageStatusEnum {
    Active,
    Inactive,
    Complete,
    Failed,
}

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct VehicleStruct {
    pub vehicle_name: VehicleEnum,
    pub current_stage: i32,
    pub is_auto: Option<bool>,
    pub patient_status: Option<PatientStatusEnum>,
    pub stages: Vec<StageStruct>,
}

#[taurpc::ipc_type]
#[derive(Debug)]
#[allow(non_snake_case)]
// create a VehiclesStruct for each vehicle
// since each mission requires all 3 vehicles to exist
// so hardcode in the vehicle enums as keys
pub struct VehiclesStruct {
    pub MEA: VehicleStruct,
    pub ERU: VehicleStruct,
    pub MRA: VehicleStruct,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type)]
pub enum VehicleEnum {
    MEA,
    ERU,
    MRA,
}

impl VehicleEnum {
    pub fn to_string(&self) -> String {
        match self {
            VehicleEnum::MEA => "MEA".to_string(),
            VehicleEnum::ERU => "ERU".to_string(),
            VehicleEnum::MRA => "MRA".to_string(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type, sqlx::Type)]
#[sqlx(type_name = "patient_status_enum")]
pub enum PatientStatusEnum {
    Secured,
    Unsecured,
}

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct StageStruct {
    pub stage_name: String,
    pub stage_id: i32,
    pub stage_status: MissionStageStatusEnum,
    pub search_area: GeofenceType,
}

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct ZonesStruct {
    pub keep_in_zones: Vec<GeofenceType>,
    pub keep_out_zones: Vec<GeofenceType>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type)]
pub enum ZoneType {
    KeepIn,
    KeepOut,
}

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct GeoCoordinateStruct {
    pub lat: f64,
    pub long: f64,
}
impl FromStr for GeoCoordinateStruct {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Invalid coordinate format".parse::<f64>().unwrap_err());
        }
        let latitude = parts[0].trim().parse::<f64>()?;
        let longitude = parts[1].trim().parse::<f64>()?;
        Ok(GeoCoordinateStruct { 
            lat: latitude,
            long: longitude,
         })
    }
}


pub type GeofenceType = Vec<GeoCoordinateStruct>;
