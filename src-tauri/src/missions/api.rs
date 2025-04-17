use super::types::*;
use std::sync::Arc;
use tauri::{AppHandle, Runtime};
use taurpc;
use tokio::sync::Mutex;

/*==============================================================================
 * MissionApiImpl Structure and Default Implementation
 *============================================================================*/

/// Mission API implementation containing mission state
#[derive(Clone)]
pub struct MissionApiImpl {
    state: Arc<Mutex<MissionsStruct>>,
}

impl Default for MissionApiImpl {
    /// Initializes mission state with default values
    /// TODO: Replace with proper database initialization later
    fn default() -> Self {
        let initial_state = MissionsStruct {
            current_mission: 0,
            missions: vec![MissionStruct {
                mission_name: "Mission 1".to_string(),
                mission_id: 0,
                mission_status: MissionStageStatusEnum::Active,
                vehicles: VehiclesStruct {
                    MEA: VehicleStruct {
                        vehicle_name: VehicleEnum::MEA,
                        current_stage: 0,
                        is_auto: Some(false),
                        patient_status: Some(PatientStatusEnum::Secured),
                        stages: vec![
                            Self::create_default_stage("test", 0),
                            Self::create_default_stage("test1", 1),
                        ],
                    },
                    ERU: VehicleStruct {
                        vehicle_name: VehicleEnum::ERU,
                        current_stage: 0,
                        is_auto: Some(false),
                        patient_status: Some(PatientStatusEnum::Unsecured),
                        stages: vec![],
                    },
                    MRA: VehicleStruct {
                        vehicle_name: VehicleEnum::MRA,
                        current_stage: 0,
                        is_auto: None,
                        patient_status: None,
                        stages: vec![],
                    },
                },
                zones: ZonesStruct {
                    keep_in_zones: vec![],
                    keep_out_zones: vec![],
                },
            }],
        };

        Self::new(initial_state)
    }
}

/*==============================================================================
 * MissionApiImpl Methods
 *============================================================================*/

impl MissionApiImpl {
    /// Create new instance with initial state
    pub fn new(initial_state: MissionsStruct) -> Self {
        Self {
            state: Arc::new(Mutex::new(initial_state)),
        }
    }

    /// Create default stage configuration
    pub fn create_default_stage(name: &str, id: u32) -> StageStruct {
        StageStruct {
            stage_name: name.to_string(),
            stage_id: id,
            stage_status: MissionStageStatusEnum::Inactive,
            search_area: vec![],
        }
    }

    /// Create default mission configuration
    pub fn create_default_mission(name: &str, id: u32) -> MissionStruct {
        MissionStruct {
            mission_name: name.to_string(),
            mission_id: id,
            mission_status: MissionStageStatusEnum::Inactive,
            vehicles: VehiclesStruct {
                MEA: VehicleStruct {
                    vehicle_name: VehicleEnum::MEA,
                    current_stage: 0,
                    is_auto: Some(false),
                    patient_status: Some(PatientStatusEnum::Secured),
                    stages: vec![],
                },
                ERU: VehicleStruct {
                    vehicle_name: VehicleEnum::ERU,
                    current_stage: 0,
                    is_auto: Some(false),
                    patient_status: Some(PatientStatusEnum::Unsecured),
                    stages: vec![],
                },
                MRA: VehicleStruct {
                    vehicle_name: VehicleEnum::MRA,
                    current_stage: 0,
                    is_auto: None,
                    patient_status: None,
                    stages: vec![],
                },
            },
            zones: ZonesStruct {
                keep_in_zones: vec![],
                keep_out_zones: vec![],
            },
        }
    }

    /// Emit state changes to frontend
    /// Should be called after any state modification
    fn emit_state_update(
        &self,
        app_handle: &AppHandle<impl Runtime>,
        state: &MissionsStruct,
    ) -> Result<(), String> {
        MissionEventTrigger::new(app_handle.clone())
            .on_updated(state.clone())
            .map_err(|e| e.to_string())
    }
}

/*==============================================================================
 * MissionApi Trait Definition
 *============================================================================*/

#[taurpc::procedures(
    event_trigger = MissionEventTrigger,
    export_to = "../src/lib/bindings.ts",
    path = "mission"
)]
pub trait MissionApi {
    // ----------------------------------
    // Event Handlers
    // ----------------------------------
    #[taurpc(event)]
    async fn on_updated(new_data: MissionsStruct);

    // ----------------------------------
    // State Management
    // ----------------------------------
    async fn get_default_data() -> MissionsStruct;
    async fn get_all_missions() -> MissionsStruct;

    // ----------------------------------
    // Mission Operations
    // ----------------------------------
    async fn rename_mission(
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        mission_name: String,
    ) -> Result<(), String>;
    async fn get_mission_data(mission_id: u32) -> MissionStruct;
    async fn create_mission(
        app_handle: AppHandle<impl Runtime>,
        mission_name: String,
    ) -> Result<(), String>;
    async fn delete_mission(
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
    ) -> Result<(), String>;

    // ----------------------------------
    // Vehicle Operations
    // ----------------------------------
    async fn set_auto_mode(
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        vehicle_name: VehicleEnum,
        is_auto: bool,
    ) -> Result<(), String>;

    // ----------------------------------
    // Stage Operations
    // ----------------------------------
    async fn add_stage(
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        vehicle_name: VehicleEnum,
        stage_name: String,
    ) -> Result<(), String>;
    async fn delete_stage(
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        vehicle_name: VehicleEnum,
        stage_id: u32,
    ) -> Result<(), String>;
    async fn rename_stage(
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        vehicle_name: VehicleEnum,
        stage_id: u32,
        stage_name: String,
    ) -> Result<(), String>;
    async fn transition_stage(
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        vehicle_name: VehicleEnum,
    ) -> Result<(), String>;

    // ----------------------------------
    // Zone Operations
    // ----------------------------------
    async fn add_zone(
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        zone_type: ZoneType,
    ) -> Result<(), String>;
    async fn delete_zone(
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        zone_type: ZoneType,
        zone_index: u32,
    ) -> Result<(), String>;
}

/*==============================================================================
 * MissionApi Trait Implementation
 *============================================================================*/

#[taurpc::resolvers]
impl MissionApi for MissionApiImpl {
    // ----------------------------------
    // State Management Implementations
    // ----------------------------------
    async fn get_default_data(self) -> MissionsStruct {
        Self::default().state.lock().await.clone()
    }

    async fn get_all_missions(self) -> MissionsStruct {
        self.state.lock().await.clone()
    }

    // ----------------------------------
    // Mission Operations Implementations
    // ----------------------------------
    async fn get_mission_data(self, mission_id: u32) -> MissionStruct {
        let state = self.state.lock().await;
        state
            .missions
            .iter()
            .find(|m| m.mission_id == mission_id)
            .map(|m| m.clone())
            .unwrap_or_else(|| panic!("Mission not found"))
    }

    async fn rename_mission(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        mission_name: String,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let mission = state
            .missions
            .iter_mut()
            .find(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;
        mission.mission_name = mission_name;
        self.emit_state_update(&app_handle, &state)
    }

    async fn create_mission(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_name: String,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let new_mission = Self::create_default_mission(&mission_name, rand::random::<u32>());
        state.missions.push(new_mission);
        self.emit_state_update(&app_handle, &state)
    }

    async fn delete_mission(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let mission_index = state
            .missions
            .iter()
            .position(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;

        if !matches!(
            state.missions[mission_index].mission_status,
            MissionStageStatusEnum::Inactive
        ) {
            return Err("Cannot delete active/past missions".into());
        }

        state.missions.remove(mission_index);
        self.emit_state_update(&app_handle, &state)
    }

    // ----------------------------------
    // Vehicle Operations Implementations
    // ----------------------------------
    async fn set_auto_mode(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        vehicle_name: VehicleEnum,
        is_auto: bool,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let mission = state
            .missions
            .iter_mut()
            .find(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;

        let vehicle = match vehicle_name {
            VehicleEnum::MEA => &mut mission.vehicles.MEA,
            VehicleEnum::ERU => &mut mission.vehicles.ERU,
            VehicleEnum::MRA => return Err("MRA auto mode unsupported".into()),
        };

        vehicle.is_auto = Some(is_auto);
        self.emit_state_update(&app_handle, &state)
    }

    // ----------------------------------
    // Stage Operations Implementations
    // ----------------------------------
    async fn add_stage(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        vehicle_name: VehicleEnum,
        stage_name: String,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let mission = state
            .missions
            .iter_mut()
            .find(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;

        let vehicle = match vehicle_name {
            VehicleEnum::MEA => &mut mission.vehicles.MEA,
            VehicleEnum::ERU => &mut mission.vehicles.ERU,
            VehicleEnum::MRA => &mut mission.vehicles.MRA,
        };

        vehicle.stages.push(Self::create_default_stage(
            &stage_name,
            rand::random::<u32>(),
        ));
        self.emit_state_update(&app_handle, &state)
    }

    async fn delete_stage(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        vehicle_name: VehicleEnum,
        stage_id: u32,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let mission = state
            .missions
            .iter_mut()
            .find(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;

        let vehicle = match vehicle_name {
            VehicleEnum::MEA => &mut mission.vehicles.MEA,
            VehicleEnum::ERU => &mut mission.vehicles.ERU,
            VehicleEnum::MRA => &mut mission.vehicles.MRA,
        };

        let stage_index = vehicle
            .stages
            .iter()
            .position(|s| s.stage_id == stage_id)
            .ok_or("Stage not found")?;

        if vehicle.current_stage >= stage_index as u32 {
            return Err("Cannot delete current/completed stage".into());
        }

        vehicle.stages.remove(stage_index);
        self.emit_state_update(&app_handle, &state)
    }
    async fn rename_stage(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        vehicle_name: VehicleEnum,
        stage_id: u32,
        stage_name: String,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let mission = state
            .missions
            .iter_mut()
            .find(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;
        let vehicle = match vehicle_name {
            VehicleEnum::MEA => &mut mission.vehicles.MEA,
            VehicleEnum::ERU => &mut mission.vehicles.ERU,
            VehicleEnum::MRA => &mut mission.vehicles.MRA,
        };
        let stage = vehicle
            .stages
            .iter_mut()
            .find(|s| s.stage_id == stage_id)
            .ok_or("Stage not found")?;
        stage.stage_name = stage_name;
        self.emit_state_update(&app_handle, &state)
    }
    async fn transition_stage(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        vehicle_name: VehicleEnum,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let mission = state
            .missions
            .iter_mut()
            .find(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;

        let vehicle = match vehicle_name {
            VehicleEnum::MEA => &mut mission.vehicles.MEA,
            VehicleEnum::ERU => &mut mission.vehicles.ERU,
            VehicleEnum::MRA => &mut mission.vehicles.MRA,
        };

        // Mark current stage as complete
        vehicle.stages[vehicle.current_stage as usize].stage_status =
            MissionStageStatusEnum::Complete;

        // Transition to next stage if available
        if (vehicle.current_stage as usize) < vehicle.stages.len() - 1 {
            vehicle.current_stage += 1;
            vehicle.stages[vehicle.current_stage as usize].stage_status =
                MissionStageStatusEnum::Active;
        }

        self.emit_state_update(&app_handle, &state)
    }

    // ----------------------------------
    // Zone Operations Implementations
    // ----------------------------------
    async fn add_zone(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        zone_type: ZoneType,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let mission = state
            .missions
            .iter_mut()
            .find(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;

        match zone_type {
            ZoneType::KeepIn => mission.zones.keep_in_zones.push(GeofenceType::default()),
            ZoneType::KeepOut => mission.zones.keep_out_zones.push(GeofenceType::default()),
        }

        self.emit_state_update(&app_handle, &state)
    }

    async fn delete_zone(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        zone_type: ZoneType,
        zone_index: u32,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let mission = state
            .missions
            .iter_mut()
            .find(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;

        match zone_type {
            ZoneType::KeepIn => {
                if zone_index >= mission.zones.keep_in_zones.len() as u32 {
                    return Err("KeepIn index out of range".into());
                }
                mission.zones.keep_in_zones.remove(zone_index as usize);
            }
            ZoneType::KeepOut => {
                if zone_index >= mission.zones.keep_out_zones.len() as u32 {
                    return Err("KeepOut index out of range".into());
                }
                mission.zones.keep_out_zones.remove(zone_index as usize);
            }
        }

        self.emit_state_update(&app_handle, &state)
    }
}
