#[taurpc::ipc_type]
#[derive(Debug)]
pub struct MissionsStruct {
    pub current_mission: u32,
    pub missions: Vec<MissionStruct>,
}

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct MissionStruct {
    pub mission_name: String,
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
    pub patient_status: Option<PatientStatusEnum>,
    pub stages: Vec<StageStruct>,
}

#[taurpc::ipc_type]
#[derive(Debug)]
#[allow(non_snake_case)]
// taurpc doesnt support hashmaps
// so we split the keys into their own struct
// and ingore snake case to match VehicleEnum
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
    pub stage_status: MissionStageStatusEnum,
    pub search_area: GeofenceType,
}

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct ZonesStruct {
    pub keep_in_zones: GeofenceType,
    pub keep_out_zones: GeofenceType,
}

#[taurpc::ipc_type]
#[derive(Debug)]
pub struct GeoCoordinateStruct {
    pub lat: f64,
    pub long: f64,
}

pub type GeofenceType = Vec<GeoCoordinateStruct>;
