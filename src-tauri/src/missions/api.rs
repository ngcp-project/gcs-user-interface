use super::types::*;
use std::sync::Arc;
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use tauri::{AppHandle, Runtime};
use taurpc;
use tokio::sync::Mutex;
use super::sql::*;
use serde_json::Value;

/*==============================================================================
 * MissionApiImpl Structure and Default Implementation
 *============================================================================*/

/// Mission API implementation containing mission state
#[derive(Clone)]
pub struct MissionApiImpl {
    state: Arc<Mutex<MissionsStruct>>,
    db: PgPool,
}

/*==============================================================================
 * MissionApiImpl Methods
 *============================================================================*/

impl MissionApiImpl {
    /// Create new instance with initial state
    pub async fn new() -> Self {
        let mut initial_state = MissionsStruct {    
            current_mission: 0,
            missions: vec![],
        };

        let database_connection = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://ngcp:ngcp@localhost:5433/ngcpdb")
            .await
            .expect("Failed to connect to the database");

        let all_mission_ids = sqlx::query(
            "SELECT mission_id FROM missions ")
            .fetch_all(&database_connection)
            .await
            .expect("Failed to execute query");

        println!("Number of mission IDs: {}", all_mission_ids.len());
        if all_mission_ids.len() > 0 {
            for mission_id_row in all_mission_ids {
                let mission_id: i32 = mission_id_row.get("mission_id");
                let mission = sqlx::query(
                "
                    SELECT 
                        missions.mission_id,
                        missions.mission_name,
                        missions.status,
                        missions.keep_in_zones,
                        missions.keep_out_zones,
                        vehicles.vehicle_name,
                        vehicles.current_stage_id AS current_stage,
                        vehicles.is_auto,
                        vehicles.patient_status,
                        stages.stage_id,
                        stages.stage_name,
                        stages.search_area,
                        stages.target_coordinate,
                        stages.status AS stage_status
                    FROM missions
                    INNER JOIN vehicles ON missions.mission_id = vehicles.mission_id
                    INNER JOIN stages ON vehicles.vehicle_id = stages.vehicle_id
                    WHERE missions.mission_id = $1
                    "
                )
                .bind(mission_id)
                .fetch_all(&database_connection)
                .await
                .expect("Failed to execute query");

                // Set current mission ID if a mission has a status of "Active"
                if mission[0].try_get::<String, _>("status").unwrap_or_else(|_| "Inactive".to_string()) == "Active" {
                    initial_state.current_mission = mission_id;
                }

                let mea_row = mission.iter()
                    .find(|row| row.get::<String, _>("vehicle_name") == "MEA")
                    .expect("Expected MEA row");

                let eru_row = mission.iter()
                    .find(|row| row.get::<String, _>("vehicle_name") == "ERU")
                    .expect("Expected ERU row");

                let mra_row = mission.iter()
                    .find(|row| row.get::<String, _>("vehicle_name") == "MRA")
                    .expect("Expected MRA row");


                initial_state.missions.push(MissionStruct {
                    mission_name: mission[0].get("mission_name"),
                    mission_id: mission[0].get("mission_id"),
                    mission_status: match mission[0].try_get::<String, _>("status").unwrap_or_else(|_| "Inactive".to_string()).as_str() {
                        "Active" => MissionStageStatusEnum::Active,
                        "Inactive" => MissionStageStatusEnum::Inactive,
                        "Complete" => MissionStageStatusEnum::Complete,
                        "Failed" => MissionStageStatusEnum::Failed,
                        _ => MissionStageStatusEnum::Inactive,
                    },
                    vehicles: VehiclesStruct {
                        MEA: VehicleStruct {
                            vehicle_name: VehicleEnum::MEA,
                            current_stage: mea_row.get("current_stage"),
                            is_auto: mea_row.get("is_auto"),
                            patient_status: mea_row.get("patient_status"),
                            stages: mission.iter()
                                .filter(|row| row.get::<String, _>("vehicle_name") == "MEA")
                                .map(|row| StageStruct {
                                    stage_name: row.get("stage_name"),
                                    stage_id: row.get("stage_id"),
                                    stage_status: match row.try_get::<String, _>("stage_status").unwrap_or_else(|_| "Inactive".to_string()).as_str() {
                                        "Active" => MissionStageStatusEnum::Active,
                                        "Inactive" => MissionStageStatusEnum::Inactive,
                                        "Complete" => MissionStageStatusEnum::Complete,
                                        "Failed" => MissionStageStatusEnum::Failed,
                                        _ => MissionStageStatusEnum::Inactive,
                                    },
                                    search_area: row.try_get::<Vec<String>, _>("search_area")
                                        .unwrap_or_else(|_| Vec::new())
                                        .into_iter()
                                        .filter_map(|s| s.parse::<GeoCoordinateStruct>().ok())
                                        .collect(),
                                })
                                .collect(),
                        },
                        ERU: VehicleStruct {
                            vehicle_name: VehicleEnum::ERU,
                            current_stage: eru_row.get("current_stage"),
                            is_auto: eru_row.get("is_auto"),
                            patient_status: eru_row.get("patient_status"),
                            stages: mission.iter()
                                .filter(|row| row.get::<String, _>("vehicle_name") == "ERU")
                                .map(|row| StageStruct {
                                    stage_name: row.get("stage_name"),
                                    stage_id: row.get("stage_id"),
                                    stage_status: match row.try_get::<String, _>("stage_status").unwrap_or_else(|_| "Inactive".to_string()).as_str() {
                                        "Active" => MissionStageStatusEnum::Active,
                                        "Inactive" => MissionStageStatusEnum::Inactive,
                                        "Complete" => MissionStageStatusEnum::Complete,
                                        "Failed" => MissionStageStatusEnum::Failed,
                                        _ => MissionStageStatusEnum::Inactive,
                                    },
                                    search_area: row.try_get::<Vec<String>, _>("search_area")
                                        .unwrap_or_else(|_| Vec::new())
                                        .into_iter()
                                        .filter_map(|s| s.parse::<GeoCoordinateStruct>().ok())
                                        .collect(),
                                })
                                .collect(),
                        },
                        MRA: VehicleStruct {
                            vehicle_name: VehicleEnum::MRA,
                            current_stage: mra_row.get("current_stage"),
                            is_auto: mra_row.get("is_auto"),
                            patient_status: mra_row.get("patient_status"),
                            stages: mission.iter()
                                .filter(|row| row.get::<String, _>("vehicle_name") == "MRA")
                                .map(|row| StageStruct {
                                    stage_name: row.get("stage_name"),
                                    stage_id: row.get("stage_id"),
                                    stage_status: match row.try_get::<String, _>("stage_status").unwrap_or_else(|_| "Inactive".to_string()).as_str() {
                                        "Active" => MissionStageStatusEnum::Active,
                                        "Inactive" => MissionStageStatusEnum::Inactive,
                                        "Complete" => MissionStageStatusEnum::Complete,
                                        "Failed" => MissionStageStatusEnum::Failed,
                                        _ => MissionStageStatusEnum::Inactive,
                                    },
                                    search_area: row.try_get::<Vec<String>, _>("search_area")
                                        .unwrap_or_else(|_| Vec::new())
                                        .into_iter()
                                        .filter_map(|s| s.parse::<GeoCoordinateStruct>().ok())
                                        .collect(),
                                })
                                .collect(),
                        },
                    },
                    zones: ZonesStruct {
                        keep_in_zones: mission[0]
                            .try_get::<Vec<String>, _>("keep_in_zones")
                            .unwrap_or_else(|_| Vec::new())
                            .into_iter()
                            .map(|zone| {
                                serde_json::from_str::<Vec<GeoCoordinateStruct>>(&zone)
                                    .unwrap_or_else(|_| Vec::new())
                            })
                            .collect(),
                        keep_out_zones: match mission[0].try_get::<Vec<String>, _>("keep_out_zones") {
                            Ok(zones) => zones
                                .into_iter()
                                .map(|zone| {
                                    serde_json::from_str::<Vec<GeoCoordinateStruct>>(&zone)
                                        .unwrap_or_else(|_| Vec::new())
                                })
                                .collect(),
                            Err(_) => Vec::new(),
                        },
                    },
                });
            }
        } 

        // println!("Initial state: {:?}", initial_state);

        Self {
            state: Arc::new(Mutex::new(initial_state)),
            db: database_connection,
        }
    }

    /// Create default stage configuration
    pub async fn create_default_stage(self, name: &str, id: i32) -> StageStruct {
        let stage_id = insert_new_stage(
            self.db.clone(),
            id,
            name,
        ).await.expect("Failed to insert new stage into database");

        StageStruct {
            stage_name: name.to_string(),
            stage_id: stage_id,
            stage_status: MissionStageStatusEnum::Inactive,
            search_area: vec![],
        }
    }

    /// Create default mission configuration
    pub async fn create_default_mission(self, name: &str) -> MissionStruct {
        let new_mission_id = insert_new_mission(self.db, name).await.unwrap_or(0);

        MissionStruct {
            mission_name: name.to_string(),
            mission_id: new_mission_id,
            mission_status: MissionStageStatusEnum::Inactive,
            vehicles: VehiclesStruct {
                MEA: VehicleStruct {
                    vehicle_name: VehicleEnum::MEA,
                    current_stage: -1,
                    is_auto: Some(false),
                    patient_status: Some(PatientStatusEnum::Unsecured),
                    stages: vec![],
                },
                ERU: VehicleStruct {
                    vehicle_name: VehicleEnum::ERU,
                    current_stage: -1,
                    is_auto: Some(false),
                    patient_status: Some(PatientStatusEnum::Unsecured),
                    stages: vec![],
                },
                MRA: VehicleStruct {
                    vehicle_name: VehicleEnum::MRA,
                    current_stage: -1,
                    is_auto: None,
                    patient_status: Some(PatientStatusEnum::Unsecured),
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
        mission_id: i32,
        mission_name: String,
    ) -> Result<(), String>;
    async fn get_mission_data(mission_id: i32) -> MissionStruct;
    async fn create_mission(
        app_handle: AppHandle<impl Runtime>,
        mission_name: String,
    ) -> Result<(), String>;
    async fn delete_mission(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
    ) -> Result<(), String>;
    async fn start_mission(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
    ) -> Result<(), String>;

    // ----------------------------------
    // Vehicle Operations
    // ----------------------------------
    async fn set_auto_mode(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
        is_auto: bool,
    ) -> Result<(), String>;

    // ----------------------------------
    // Stage Operations
    // ----------------------------------
    async fn add_stage(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
        stage_name: String,
    ) -> Result<(), String>;
    async fn update_stage(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
        stage_id: i32,
        new_stage_name: Option<String>,
        new_status: Option<MissionStageStatusEnum>,
    ) -> Result<(), String>;
    async fn delete_stage(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
        stage_id: i32,
    ) -> Result<(), String>;
    async fn rename_stage(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
        stage_id: i32,
        stage_name: String,
    ) -> Result<(), String>;
    async fn transition_stage(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
    ) -> Result<(), String>;

    async fn update_stage_area(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
        stage_id: i32,
        area: GeofenceType,
    ) -> Result<(), String>;

    // ----------------------------------
    // Zone Operations
    // ----------------------------------
    async fn add_zone(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        zone_type: ZoneType,
    ) -> Result<(), String>;
    async fn update_zone(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        zone_type: ZoneType,
        zone_index: i32,
        zone_coords: GeofenceType,
    ) -> Result<(), String>;
    async fn delete_zone(
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        zone_type: ZoneType,
        zone_index: i32,
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
        Self::new().await.state.lock().await.clone()
    }

    async fn get_all_missions(self) -> MissionsStruct {
        self.state.lock().await.clone()
    }

    // ----------------------------------
    // Mission Operations Implementations
    // ----------------------------------
    async fn get_mission_data(self, mission_id: i32) -> MissionStruct {
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
        mission_id: i32,
        mission_name: String,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let mission = state
            .missions
            .iter_mut()
            .find(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;

        update_mission_name(
            self.db.clone(),
            mission.mission_id,
            &mission_name,
        ).await.expect("Failed to update mission name");
        mission.mission_name = mission_name;
        self.emit_state_update(&app_handle, &state)
    }

    async fn create_mission(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_name: String,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let new_mission = Self::create_default_mission(self.clone(), &mission_name).await;
        state.missions.push(new_mission);
        self.emit_state_update(&app_handle, &state)
    }

    async fn delete_mission(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
    ) -> Result<(), String> {
        println!("Deleting mission with ID: {}", mission_id);
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
        delete_mission(
            self.db.clone(),
            state.missions[mission_index].mission_id,
        ).await.expect("Failed to delete mission from database");

        state.missions.remove(mission_index);
        self.emit_state_update(&app_handle, &state)
    }

    async fn start_mission(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;

        if let Some(prev_mission_index) = state.missions.iter().position(|m| m.mission_id == state.current_mission) {
            state.missions[prev_mission_index].mission_status = MissionStageStatusEnum::Complete;
            update_mission_status(self.db.clone(), state.missions[prev_mission_index].mission_id, "Complete").await.expect("Failed to update mission status");
        }

        let start_mission_index = state.missions.iter().position(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;
                
        state.missions[start_mission_index].mission_status = MissionStageStatusEnum::Active;
        state.current_mission = mission_id;
        update_mission_status(self.db.clone(), mission_id, "Active").await.expect("Failed to update mission status");

        
        self.emit_state_update(&app_handle, &state)
    }

    // ----------------------------------
    // Vehicle Operations Implementations
    // ----------------------------------
    async fn set_auto_mode(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
        is_auto: bool,
    ) -> Result<(), String> {
        println!("Setting auto mode for vehicle: {:?}", vehicle_name);
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

        update_auto_mode_vehicle(
            self.db.clone(),
            mission.mission_id,
            vehicle.vehicle_name.to_string(),
            is_auto,
        ).await.expect("Failed to update auto mode in database");

        vehicle.is_auto = Some(is_auto);
        self.emit_state_update(&app_handle, &state)
    }

    // ----------------------------------
    // Stage Operations Implementations
    // ----------------------------------
    async fn add_stage(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
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
        let vehicle_id = select_vehicle_from_mission(
            self.db.clone(),
            mission.mission_id,
            vehicle.vehicle_name.to_string(),
        ).await.expect("Failed to find vehicle mission");

        let default_stage = Self::create_default_stage(
            self.clone(),
            &stage_name,
            vehicle_id
        ).await;
        println!("Default stage created: {:?}", &default_stage);
        let stage_id = default_stage.stage_id;
        vehicle.stages.push(default_stage);

        if vehicle.current_stage == -1 {
            vehicle.current_stage = stage_id;
        }

        self.emit_state_update(&app_handle, &state)
    }

    async fn update_stage(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
        stage_id: i32,
        new_stage_name: Option<String>,
        new_status: Option<MissionStageStatusEnum>,
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

        if let Some(name) = new_stage_name {
            stage.stage_name = name;
        }
        if let Some(status) = new_status {
            stage.stage_status = status;
        }

        update_stage_name(
            self.db.clone(),
            stage.stage_id,
            &stage.stage_name,
        ).await.expect("Failed to update stage name");
        
        update_stage_status(
            self.db.clone(),
            stage.stage_id,
            &match stage.stage_status {
                MissionStageStatusEnum::Active => "Active",
                MissionStageStatusEnum::Inactive => "Inactive",
                MissionStageStatusEnum::Complete => "Complete",
                MissionStageStatusEnum::Failed => "Failed",
            },
        ).await.expect("Failed to update stage status");

        self.emit_state_update(&app_handle, &state)
    }

    async fn update_stage_area(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
        stage_id: i32,
        area: GeofenceType,
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

        stage.search_area = area;

        let search_area_string = format!(
            "[\n    {}\n]",
            stage.search_area
                .iter()
                .map(|coord| format!("({}, {})", coord.lat, coord.long))
                .collect::<Vec<String>>()
                .join(",\n    ")
        );
        
        let search_area_array: Vec<String> = vec![search_area_string.clone()];
        
        update_stage_area(
            self.db.clone(),
            stage.stage_id,
            search_area_array,
        ).await.expect("Failed to update stage area");

        self.emit_state_update(&app_handle, &state)
    }

    async fn delete_stage(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
        stage_id: i32,
    ) -> Result<(), String> {
        println!("Deleting stage with ID: {}", stage_id);
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

        let stage = &vehicle.stages[stage_index];
        if matches!(stage.stage_status, MissionStageStatusEnum::Active | MissionStageStatusEnum::Complete) {
            return Err("Cannot delete current/completed stage".into());
        }
        delete_stage(
            self.db.clone(),
            stage_id,
        ).await.expect("Failed to delete stage from database");

        vehicle.stages.remove(stage_index);
        self.emit_state_update(&app_handle, &state)
    }

    async fn rename_stage(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,
        stage_id: i32,
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

        update_stage_name(
            self.db.clone(),
            stage.stage_id,
            &stage_name,
        ).await.expect("Failed to update stage name");

        stage.stage_name = stage_name;
        self.emit_state_update(&app_handle, &state)
    }
    
    async fn transition_stage(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        vehicle_name: VehicleEnum,

    ) -> Result<(), String> {
        println!("Transitioning stage for vehicle: {:?}", vehicle_name);
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

        // println!("\n\nStart vehicle rust state: {:?}\n\n\n", vehicle);
        println!("Current Stage: {:?}", vehicle.current_stage);

        // Mark current stage as complete
        if let Some(stage) = vehicle.stages.iter_mut().find(|s| s.stage_id == vehicle.current_stage) {
            stage.stage_status = MissionStageStatusEnum::Complete;
        } else {
            println!("Stage with ID not found");
        }

        // Transition to next stage if available
        let transitioned_stage = transition_stage(
            self.db.clone(),
            mission.mission_id,
            vehicle.vehicle_name.to_string(),
            vehicle.current_stage,
        ).await.expect("Failed to transition stage");

        println!("After Transition Stage: {:?}", transitioned_stage.unwrap_or(vehicle.current_stage));

        if let Some(stage) = vehicle.stages.iter_mut().find(|s| s.stage_id == transitioned_stage.unwrap_or(vehicle.current_stage)) {
            vehicle.current_stage = transitioned_stage.unwrap_or(vehicle.current_stage);
            // println!("Rust state current Stage after transition: {:?}", vehicle.current_stage);
            stage.stage_status = MissionStageStatusEnum::Active;
        } else {
            println!("No next stage available");
        }

        // println!("\n\n\nEnd vehicle rust state: {:?}", vehicle);

        self.emit_state_update(&app_handle, &state)
    }

    // ----------------------------------
    // Zone Operations Implementations
    // ----------------------------------
    async fn add_zone(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        zone_type: ZoneType,
    ) -> Result<(), String> {
        println!("Adding zone of type: {:?}", zone_type);
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

        // note: no need for SQL here since its just an empty zone be changed in the rust state

        self.emit_state_update(&app_handle, &state)
    }

    async fn update_zone(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        zone_type: ZoneType,
        zone_index: i32,
        zone_coords: GeofenceType,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let mission = state
            .missions
            .iter_mut()
            .find(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;

        // TODO: Error handling for out of bounds
        match zone_type {
            ZoneType::KeepIn => {
                // if zone_index >= mission.zones.keep_in_zones.len() as u32 {
                //     return Err("KeepIn index out of range".into());
                // }
                mission.zones.keep_in_zones[zone_index as usize] = zone_coords;
            }
            ZoneType::KeepOut => {
                // if zone_index >= mission.zones.keep_out_zones.len() as u32 {
                //     return Err("KeepOut index out of range".into());
                // }
                mission.zones.keep_out_zones[zone_index as usize] = zone_coords;
            }
        }

        let keep_in_zones = mission.zones.keep_in_zones.iter()
            .map(|zone| {
                let json = serde_json::to_string(zone).unwrap();
                convert_zone_format(&json)
            })
            .collect::<Vec<String>>();

        let keep_out_zones = mission.zones.keep_out_zones.iter()
            .map(|zone| {
                let json = serde_json::to_string(zone).unwrap();
                convert_zone_format(&json)
            })
            .collect::<Vec<String>>();


        // update zones
        update_zones(
            self.db.clone(),
            mission.mission_id,
            keep_in_zones.clone(),
            keep_out_zones.clone(),
        ).await.expect("Failed to add zones");

        self.emit_state_update(&app_handle, &state)
    }

    async fn delete_zone(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
        zone_type: ZoneType,
        zone_index: i32,
    ) -> Result<(), String> {
        println!("Deleting zone of type: {:?} at index: {}", zone_type, zone_index);
        let mut state = self.state.lock().await;
        let mission = state
            .missions
            .iter_mut()
            .find(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;

        // TODO: Error handling for out of bounds
        match zone_type {
            ZoneType::KeepIn => {
                if zone_index >= mission.zones.keep_in_zones.len() as i32 {
                    return Err("KeepIn index out of range".into());
                }
                mission.zones.keep_in_zones.remove(zone_index as usize);
            }
            ZoneType::KeepOut => {
                if zone_index >= mission.zones.keep_out_zones.len() as i32 {
                    return Err("KeepOut index out of range".into());
                }
                mission.zones.keep_out_zones.remove(zone_index as usize);
            }
        }

        let keep_in_zones = mission.zones.keep_in_zones.iter()
            .map(|zone| {
                let json = serde_json::to_string(zone).unwrap();
                convert_zone_format(&json)
            })
            .collect::<Vec<String>>();

        let keep_out_zones = mission.zones.keep_out_zones.iter()
            .map(|zone| {
                let json = serde_json::to_string(zone).unwrap();
                convert_zone_format(&json)
            })
            .collect::<Vec<String>>();


        // update zones
        update_zones(
            self.db.clone(),
            mission.mission_id,
            keep_in_zones.clone(),
            keep_out_zones.clone(),
        ).await.expect("Failed to delete zones");

        self.emit_state_update(&app_handle, &state)
    }
}

// helper function for converting JSON string to zone format
fn convert_zone_format(json_str: &str) -> String {
    let parsed: Value = serde_json::from_str(json_str).unwrap();

    if let Some(arr) = parsed.as_array() {
        let tuples: Vec<String> = arr.iter().map(|point| {
            let lat = point["lat"].as_f64().unwrap();
            let long = point["long"].as_f64().unwrap();
            format!("({:.5},{:.5})", lat, long)
        }).collect();

        format!("[\n    {}\n]", tuples.join(",\n    "))
    } else {
        String::new()
    }
}
