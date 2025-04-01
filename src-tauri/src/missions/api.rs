use super::types::*;
use std::sync::Arc;
use std::{collections::HashMap, option};
use tauri::{AppHandle, Runtime};
use taurpc;
use tokio::sync::Mutex;

// Define the MissionApiImpl struct that contains a mutable MissionsStruct
#[derive(Clone)]
pub struct MissionApiImpl {
    state: Arc<Mutex<MissionsStruct>>,
}

// Default implementation for MissionApiImpl that sets the initial state
// Initializes mission state when calling MissionApiImpl::default()
impl Default for MissionApiImpl {
    fn default() -> Self {
        // Use long default state for now
        // Remove later
        fn create_stage(name: &str, id: u32) -> StageStruct {
            StageStruct {
                stage_name: name.to_string(),
                stage_id: id,
                stage_status: MissionStageStatusEnum::Inactive,
                search_area: vec![
                    GeoCoordinateStruct {
                        lat: 0.0,
                        long: 0.0,
                    },
                    GeoCoordinateStruct {
                        lat: 0.0,
                        long: 1.0,
                    },
                    GeoCoordinateStruct {
                        lat: 1.0,
                        long: 1.0,
                    },
                    GeoCoordinateStruct {
                        lat: 1.0,
                        long: 0.0,
                    },
                ],
            }
        }

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
                        patient_status: Some(PatientStatusEnum::Secured),
                        stages: vec![create_stage("test", 0), create_stage("test1", 1)],
                    },
                    ERU: VehicleStruct {
                        vehicle_name: VehicleEnum::ERU,
                        current_stage: 0,
                        patient_status: Some(PatientStatusEnum::Unsecured),
                        stages: vec![],
                    },
                    MRA: VehicleStruct {
                        vehicle_name: VehicleEnum::MRA,
                        current_stage: 0,
                        patient_status: None,
                        stages: vec![],
                    },
                },
                zones: ZonesStruct {
                    keep_in_zones: vec![
                        GeoCoordinateStruct {
                            lat: 0.0,
                            long: 0.0,
                        },
                        GeoCoordinateStruct {
                            lat: 0.0,
                            long: 1.0,
                        },
                        GeoCoordinateStruct {
                            lat: 1.0,
                            long: 1.0,
                        },
                        GeoCoordinateStruct {
                            lat: 1.0,
                            long: 0.0,
                        },
                    ],
                    keep_out_zones: vec![
                        GeoCoordinateStruct {
                            lat: 0.0,
                            long: 0.0,
                        },
                        GeoCoordinateStruct {
                            lat: 0.0,
                            long: 1.0,
                        },
                        GeoCoordinateStruct {
                            lat: 1.0,
                            long: 1.0,
                        },
                        GeoCoordinateStruct {
                            lat: 1.0,
                            long: 0.0,
                        },
                    ],
                },
            }],
        };

        // Create a new instance of MissionApiImpl with the initial state
        Self::new(initial_state)
    }
}

impl MissionApiImpl {
    // Constructor for MissionApiImpl
    pub fn new(initial_state: MissionsStruct) -> Self {
        // Must wrap the state in an Arc<Mutex<>>
        Self {
            state: Arc::new(Mutex::new(initial_state)),
        }
    }
    pub fn create_default_stage(name: &str, id: u32) -> StageStruct {
        StageStruct {
            stage_name: name.to_string(),
            stage_id: id,
            stage_status: MissionStageStatusEnum::Inactive,
            search_area: vec![],
        }
    }
    pub fn create_default_mission(name: &str, id: u32) -> MissionStruct {
        MissionStruct {
            mission_name: name.to_string(),
            mission_id: id,
            mission_status: MissionStageStatusEnum::Inactive,
            vehicles: VehiclesStruct {
                MEA: VehicleStruct {
                    vehicle_name: VehicleEnum::MEA,
                    current_stage: 0,
                    patient_status: Some(PatientStatusEnum::Secured),
                    stages: vec![],
                },
                ERU: VehicleStruct {
                    vehicle_name: VehicleEnum::ERU,
                    current_stage: 0,
                    patient_status: Some(PatientStatusEnum::Unsecured),
                    stages: vec![],
                },
                MRA: VehicleStruct {
                    vehicle_name: VehicleEnum::MRA,
                    current_stage: 0,
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
    // Helper method to emit state changes
    // Use whenever state needs to update
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

#[taurpc::procedures(
  event_trigger = MissionEventTrigger, // Define the event trigger for the mission api (used in emit_state_update)
  export_to = "../src/lib/bindings.ts", // Export the API to the bindings file
  path = "mission" // Namespace for the mission api
)]

// Define the MissionApi trait with the required methods
pub trait MissionApi {
    #[taurpc(event)]
    async fn on_updated(new_data: MissionsStruct);
    async fn get_default_data() -> MissionsStruct;

    // State initialization
    async fn get_all_missions() -> MissionsStruct;

    // Mission Data
    async fn set_mission_data(
        app_handle: AppHandle<impl Runtime>,
        mission_data: MissionStruct,
    ) -> Result<(), String>;
    async fn get_mission_data(mission_id: u32) -> MissionStruct;
    async fn create_mission(
        app_handle: AppHandle<impl Runtime>,
        mission_name: String,
        mission_id: u32,
    ) -> Result<(), String>;

    // Stage Data
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
    async fn transition_stage(
        app_handle: AppHandle<impl Runtime>,
        mission_id: u32,
        vehicle_name: VehicleEnum,
    ) -> Result<(), String>;
}

// Implement the MissionApi trait methods
#[taurpc::resolvers]
impl MissionApi for MissionApiImpl {
    // State initialization
    async fn get_mission_data(self, mission_id: u32) -> MissionStruct {
        // search for mission_id field in missions array
        let state = self.state.lock().await;

        for mission in state.missions.iter() {
            if mission.mission_id == mission_id {
                return mission.clone();
            }
        }
        panic!("Mission not found");
    }

    // Mission Data
    async fn set_mission_data(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_data: MissionStruct,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let new_mission = mission_data.clone();
        state.missions[new_mission.mission_id as usize] = mission_data;
        println!("Mission data set: {:?}", state);
        self.emit_state_update(&app_handle, &state)
    }

    async fn create_mission(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_name: String,
        mission_id: u32,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;

        // TODO: fix to use database defined id
        let new_mission_data = Self::create_default_mission(&mission_name, mission_id);

        state.missions.push(new_mission_data);
        println!("Mission length: {:?}", state.missions.len());
        self.emit_state_update(&app_handle, &state)
    }

    // Stage Data
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
            .ok_or("Mission not found".to_string())?;

        let vehicle = match vehicle_name {
            VehicleEnum::MEA => &mut mission.vehicles.MEA,
            VehicleEnum::ERU => &mut mission.vehicles.ERU,
            VehicleEnum::MRA => &mut mission.vehicles.MRA,
        };

        vehicle
            .stages
            .push(Self::create_default_stage(&stage_name, 4));
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
            .ok_or("Mission not found".to_string())?;
        let vehicle = match vehicle_name {
            VehicleEnum::MEA => &mut mission.vehicles.MEA,
            VehicleEnum::ERU => &mut mission.vehicles.ERU,
            VehicleEnum::MRA => &mut mission.vehicles.MRA,
        };

        if vehicle.stages.len() < 1 {
            return Err("Vehicle has no stages".to_string());
        }

        let stage_index = vehicle.stages.iter().position(|s| s.stage_id == stage_id);

        if stage_index == None {
            return Err("Stage not found".to_string());
        }

        if vehicle.current_stage >= (stage_index.unwrap() as u32) {
            return Err("Cannot delete already completed or current stage".to_string());
        }

        vehicle.stages.remove(stage_index.unwrap());
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
            .ok_or("Mission not found".to_string())?;
        // Pass by reference to avoid partial move
        let vehicle = match vehicle_name {
            VehicleEnum::MEA => &mut mission.vehicles.MEA,
            VehicleEnum::ERU => &mut mission.vehicles.ERU,
            VehicleEnum::MRA => &mut mission.vehicles.MRA,
        };

        if (vehicle.current_stage as usize) >= vehicle.stages.len() - 1 {
            return Err("Vehicle is already at last stage".to_string());
        }

        vehicle.current_stage += 1;
        println!("Vehicle data set: {:?}", vehicle);
        self.emit_state_update(&app_handle, &state)
    }

    // Return the default state of the mission
    // used by frontend to first initialize the mission
    async fn get_default_data(self) -> MissionsStruct {
        Self::default().state.lock().await.clone()
    }

    // Get the current state of the mission
    async fn get_all_missions(self) -> MissionsStruct {
        self.state.lock().await.clone()
    }
}
