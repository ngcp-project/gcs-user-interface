use super::types::*;
use std::collections::HashMap;
use std::sync::Arc;
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
        let stage = StageStruct {
            stage_name: "Takeoff".to_string(),
            stage_status: MissionStageStatusEnum::Complete,
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
        };

        let initial_state = MissionsStruct {
            current_mission: 0,
            missions: HashMap::from([(
                0,
                MissionStruct {
                    mission_name: "Mission 1".to_string(),
                    mission_status: MissionStageStatusEnum::Active,
                    vehicles: VehiclesStruct {
                        MEA: VehicleStruct {
                            vehicle_name: VehicleEnum::MEA,
                            current_stage: 0,
                            patient_status: Some(PatientStatusEnum::Secured),
                            stages: HashMap::from([(0, stage.clone()), (1, stage.clone())]),
                        },
                        ERU: VehicleStruct {
                            vehicle_name: VehicleEnum::ERU,
                            current_stage: 0,
                            patient_status: Some(PatientStatusEnum::Unsecured),
                            stages: HashMap::from([(0, stage.clone()), (1, stage.clone())]),
                        },
                        MRA: VehicleStruct {
                            vehicle_name: VehicleEnum::MRA,
                            current_stage: 0,
                            patient_status: None,
                            stages: HashMap::from([(0, stage.clone()), (1, stage.clone())]),
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
                },
            )]),
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

    // Helper method to modify state and emit update
    //     async fn update_state<F>(
    //         &self,
    //         app_handle: AppHandle<impl Runtime>,
    //         updater: F,
    //     ) -> Result<(), String>
    //     where
    //         F: FnOnce(&mut MissionsStruct),
    //     {
    //         let mut state = self.state.lock().await;

    //         // Updater is parameter that takes a generic function that can modify the state
    //         // state must be a mutable reference to the state
    //         updater(&mut state);
    //         // Emit the state update for frontend to listen to
    //         self.emit_state_update(&app_handle, &state)
    //     }
}

#[taurpc::procedures(
  event_trigger = MissionEventTrigger, // Define the event trigger for the mission api (used in emit_state_update)
  export_to = "../src/lib/bindings.ts", // Export the API to the bindings file
  path = "mission" // Namespace for the mission api
)]

// Define the MissionApi trait with the required methods
pub trait MissionApi {
    async fn get_default_data() -> MissionsStruct;
    async fn get_all_missions() -> MissionsStruct;

    // async fn get_mission_data(mission_id: u32) -> MissionStruct;
    // async fn get_vehicle_data(mission_id: u32, vehicle_name: VehicleEnum) -> VehicleStruct;

    // async fn get_stage_data(
    //     mission_id: u32,
    //     vehicle_name: VehicleEnum,
    //     stage_id: u32,
    // ) -> StageStruct;
    // async fn get_zones_data(mission_id: u32) -> ZonesStruct;

    async fn submit_mission(
        app_handle: AppHandle<impl Runtime>,
        mission_data: MissionStruct,
    ) -> Result<(), String>;

    #[taurpc(event)]
    async fn on_updated(new_data: MissionsStruct);
}

// Implement the MissionApi trait methods
#[taurpc::resolvers]
impl MissionApi for MissionApiImpl {
    // async fn get_mission_data(self, mission_id: u32) -> MissionStruct {
    //     self.state.lock().await.missions[mission_id as usize].clone()
    // }

    // async fn get_vehicle_data(self, mission_id: u32, vehicle_name: VehicleEnum) -> VehicleStruct {
    //     let vehicles = self.state.lock().await.missions[mission_id as usize]
    //         .vehicles
    //         .clone();

    //     match vehicle_name {
    //         VehicleEnum::MEA => vehicles.MEA,
    //         VehicleEnum::ERU => vehicles.ERU,
    //         VehicleEnum::MRA => vehicles.MRA,
    //     }
    // }

    // async fn get_stage_data(
    //     self,
    //     mission_id: u32,
    //     vehicle_name: VehicleEnum,
    //     stage_id: u32,
    // ) -> StageStruct {
    //     let vehicle = self.get_vehicle_data(mission_id, vehicle_name).await;
    //     vehicle.stages[stage_id as usize].clone()
    // }

    // async fn get_zones_data(self, mission_id: u32) -> ZonesStruct {
    //     self.state.lock().await.missions[mission_id as usize]
    //         .zones
    //         .clone()
    // }

    async fn submit_mission(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_data: MissionStruct,
    ) -> Result<(), String> {
        let state = self.state.lock().await;
        println!("Submitting Mission: {:?}", state);
        // Append to missions array
        // state.missions.push(mission_data);
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
