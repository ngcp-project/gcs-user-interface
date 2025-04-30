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
    pub mission_id: u32,
    pub mission_status: MissionStageStatusEnum,
    pub vehicles: VehiclesStruct,
    pub zones: ZonesStruct,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type)]
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
    pub current_stage: u32,
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

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type)]
pub enum PatientStatusEnum {
    Secured,
    Unsecured,
}

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct StageStruct {
    pub stage_name: String,
    pub stage_id: u32,
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

pub type GeofenceType = Vec<GeoCoordinateStruct>;
