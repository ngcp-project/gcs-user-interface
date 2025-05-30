use super::sql::*;
use super::types::*;
use std::sync::Arc;
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use tauri::{AppHandle, Runtime};
use taurpc;
use tokio::sync::Mutex;
use serde_json::Value;
use crate::commands::CommandsApi;
use crate::commands::commands::{CommandsApiImpl, GeoCoordinate};

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

        let all_mission_ids = sqlx::query("SELECT mission_id FROM missions ")
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
                    LEFT JOIN vehicles ON missions.mission_id = vehicles.mission_id
                    LEFT JOIN stages ON vehicles.vehicle_id = stages.vehicle_id
                    WHERE missions.mission_id = $1
                    ",
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
                    mission_status: match mission[0]
                        .try_get::<String, _>("status")
                        .unwrap_or_else(|_| "Inactive".to_string())
                        .as_str()
                    {
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
                            patient_status: 
                                match mea_row.get::<String, _>("patient_status").as_str() {
                                    "Unsecured" => Some(PatientStatusEnum::Unsecured),
                                    "Secured" => Some(PatientStatusEnum::Secured),
                                    _ => Some(PatientStatusEnum::Unsecured),
                                }, 
                            stages: 
                            if mea_row.get::<i32, _>("current_stage") != -1 {
                                mission.iter()
                                    .filter(|row| row.get::<String, _>("vehicle_name") == "MEA")
                                    .map(|row| StageStruct {
                                        stage_name: row.get("stage_name"),
                                        stage_id: row.get("stage_id"),
                                        stage_status: match row
                                            .try_get::<String, _>("stage_status")
                                            .unwrap_or_else(|_| "Inactive".to_string())
                                            .as_str()
                                        {
                                            "Active" => MissionStageStatusEnum::Active,
                                            "Inactive" => MissionStageStatusEnum::Inactive,
                                            "Complete" => MissionStageStatusEnum::Complete,
                                            "Failed" => MissionStageStatusEnum::Failed,
                                            _ => MissionStageStatusEnum::Inactive,
                                        },
                                        search_area:
                                        match row.try_get::<Vec<String>, _>("search_area").unwrap_or_else(|_| Vec::new()) {
                                            search_areas => search_areas
                                                .into_iter()
                                                .filter_map(|area: String| {
                                                    serde_json::from_str::<Vec<GeoCoordinateStruct>>(convert_zone_to_json(&area).as_str()).ok()
                                                })
                                                .flatten()
                                                .collect::<Vec<GeoCoordinateStruct>>()
                                            }
                                    })
                                    .collect()
                            } else {
                                vec![]
                            }
                        },
                        ERU: VehicleStruct {
                            vehicle_name: VehicleEnum::ERU,
                            current_stage: eru_row.get("current_stage"),
                            is_auto: eru_row.get("is_auto"),
                            patient_status: 
                                match eru_row.get::<String, _>("patient_status").as_str() {
                                    "Unsecured" => Some(PatientStatusEnum::Unsecured),
                                    "Secured" => Some(PatientStatusEnum::Secured),
                                    _ => Some(PatientStatusEnum::Unsecured),
                                },
                            stages: 
                            if eru_row.get::<i32, _>("current_stage") != -1 {
                                mission.iter()
                                    .filter(|row| row.get::<String, _>("vehicle_name") == "ERU")
                                    .map(|row| StageStruct {
                                        stage_name: row.get("stage_name"),
                                        stage_id: row.get("stage_id"),
                                        stage_status: match row
                                            .try_get::<String, _>("stage_status")
                                            .unwrap_or_else(|_| "Inactive".to_string())
                                            .as_str()
                                        {
                                            "Active" => MissionStageStatusEnum::Active,
                                            "Inactive" => MissionStageStatusEnum::Inactive,
                                            "Complete" => MissionStageStatusEnum::Complete,
                                            "Failed" => MissionStageStatusEnum::Failed,
                                            _ => MissionStageStatusEnum::Inactive,
                                        },
                                        search_area: 
                                            match row.try_get::<Vec<String>, _>("search_area").unwrap_or_else(|_| Vec::new()) {
                                            search_areas => search_areas
                                                .into_iter()
                                                .filter_map(|area: String| {
                                                    serde_json::from_str::<Vec<GeoCoordinateStruct>>(convert_zone_to_json(&area).as_str()).ok()
                                                })
                                                .flatten()
                                                .collect::<Vec<GeoCoordinateStruct>>()
                                            }
                                    })
                                    .collect()
                            } else {
                                vec![]
                            }
                        },
                        MRA: VehicleStruct {
                            vehicle_name: VehicleEnum::MRA,
                            current_stage: mra_row.get("current_stage"),
                            is_auto: mra_row.get("is_auto"),
                            patient_status:
                                match mra_row.get::<String, _>("patient_status").as_str() {
                                    "Unsecured" => Some(PatientStatusEnum::Unsecured),
                                    "Secured" => Some(PatientStatusEnum::Secured),
                                    _ => Some(PatientStatusEnum::Unsecured),
                                },
                            stages: 
                            if mra_row.get::<i32, _>("current_stage") != -1 {
                                mission.iter()
                                    .filter(|row| row.get::<String, _>("vehicle_name") == "MRA")
                                    .map(|row| StageStruct {
                                        stage_name: row.get("stage_name"),
                                        stage_id: row.get("stage_id"),
                                        stage_status: match row
                                            .try_get::<String, _>("stage_status")
                                            .unwrap_or_else(|_| "Inactive".to_string())
                                            .as_str()
                                        {
                                            "Active" => MissionStageStatusEnum::Active,
                                            "Inactive" => MissionStageStatusEnum::Inactive,
                                            "Complete" => MissionStageStatusEnum::Complete,
                                            "Failed" => MissionStageStatusEnum::Failed,
                                            _ => MissionStageStatusEnum::Inactive,
                                        },
                                        search_area:
                                            match row.try_get::<Vec<String>, _>("search_area").unwrap_or_else(|_| Vec::new()) {
                                            search_areas => search_areas
                                                .into_iter()
                                                .filter_map(|area: String| {
                                                    serde_json::from_str::<Vec<GeoCoordinateStruct>>(convert_zone_to_json(&area).as_str()).ok()
                                                })
                                                .flatten()
                                                .collect::<Vec<GeoCoordinateStruct>>()
                                            },
                                    })
                                    .collect()
                            } else {
                                vec![]
                            }
                        },
                    },
                    zones: ZonesStruct {
                        keep_in_zones: mission[0]
                            .try_get::<Vec<String>, _>("keep_in_zones")
                            .unwrap_or_else(|_| Vec::new())
                            .into_iter()
                            .map(|zone| {
                                serde_json::from_str::<Vec<GeoCoordinateStruct>>(convert_zone_to_json(&zone).as_str())
                                    .unwrap_or_else(|_| Vec::new())
                            })
                            .collect(),
                        keep_out_zones:
                            mission[0]
                                .try_get::<Vec<String>, _>("keep_out_zones")
                                .unwrap_or_else(|_| Vec::new())
                                .into_iter()
                                .map(|zone| {
                                    serde_json::from_str::<Vec<GeoCoordinateStruct>>(convert_zone_to_json(&zone).as_str())
                                        .unwrap_or_else(|_| Vec::new())
                                })
                                .collect(),
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
        let stage_id = insert_new_stage(self.db.clone(), id, name)
            .await
            .expect("Failed to insert new stage into database");

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

        update_mission_name(self.db.clone(), mission.mission_id, &mission_name)
            .await
            .expect("Failed to update mission name");
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
        delete_mission(self.db.clone(), state.missions[mission_index].mission_id)
            .await
            .expect("Failed to delete mission from database");

        state.missions.remove(mission_index);
        self.emit_state_update(&app_handle, &state)
    }

    async fn start_mission(
        self,
        app_handle: AppHandle<impl Runtime>,
        mission_id: i32,
    ) -> Result<(), String> {
        let mut state = self.state.lock().await;
        let commands_api = CommandsApiImpl::default();

        // First, handle the previous mission if it exists
        if let Some(prev_mission_index) = state.missions.iter().position(|m| m.mission_id == state.current_mission) {
            state.missions[prev_mission_index].mission_status = MissionStageStatusEnum::Complete;
            update_mission_status(self.db.clone(), state.missions[prev_mission_index].mission_id, "Complete").await.expect("Failed to update mission status");
        }

        // Find and update the new mission
        let start_mission_index = state.missions.iter().position(|m| m.mission_id == mission_id)
            .ok_or("Mission not found")?;
        
        // Update mission status first
        state.missions[start_mission_index].mission_status = MissionStageStatusEnum::Active;
        state.current_mission = mission_id;
        update_mission_status(self.db.clone(), mission_id, "Active").await.expect("Failed to update mission status");

        // Emit state update to ensure frontend reflects the change
        self.emit_state_update(&app_handle, &state)?;

        // Now handle the zone updates
        let mission = &state.missions[start_mission_index];
        
        // Send keep-in zones (commandID: 2) only if there are valid zones
        for zone in &mission.zones.keep_in_zones {
            if zone.len() >= 3 {  // Only send if we have at least 3 coordinates
                let coords: Vec<GeoCoordinate> = zone.iter()
                    .take(6) // Limit to 6 points
                    .map(|coord| GeoCoordinate {
                        lat: coord.lat,
                        long: coord.long,
                    })
                    .collect();
                
                // Send to ALL vehicles at once
                commands_api.clone().send_zone_update("ALL".to_string(), "2".to_string(), coords).await?;
            }
        }

        // Send keep-out zones (commandID: 3) only if there are valid zones
        for zone in &mission.zones.keep_out_zones {
            if zone.len() >= 3 {  // Only send if we have at least 3 coordinates
                let coords: Vec<GeoCoordinate> = zone.iter()
                    .take(6) // Limit to 6 points
                    .map(|coord| GeoCoordinate {
                        lat: coord.lat,
                        long: coord.long,
                    })
                    .collect();
                
                // Send to ALL vehicles at once
                commands_api.clone().send_zone_update("ALL".to_string(), "3".to_string(), coords).await?;
            }
        }

        // Update vehicle stages and send search areas
        let vehicles = &mut state.missions[start_mission_index].vehicles;
        
        // Set the first stage of each vehicle to active if they have stages
        if !vehicles.MEA.stages.is_empty() {
            vehicles.MEA.stages[0].stage_status = MissionStageStatusEnum::Active;
            update_stage_status(
                self.db.clone(),
                vehicles.MEA.stages[0].stage_id,
                "Active",
            ).await.expect("Failed to update stage status");

            // Send search area for MEA only if it has valid coordinates
            let search_area = &vehicles.MEA.stages[0].search_area;
            if search_area.len() >= 3 {  // Only send if we have at least 3 coordinates
                let coords: Vec<GeoCoordinate> = search_area.iter()
                    .take(6)
                    .map(|coord| GeoCoordinate {
                        lat: coord.lat,
                        long: coord.long,
                    })
                    .collect();
                
                commands_api.clone().send_zone_update("MEA".to_string(), "4".to_string(), coords).await?;
            }
        }
        
        if !vehicles.ERU.stages.is_empty() {
            vehicles.ERU.stages[0].stage_status = MissionStageStatusEnum::Active;
            update_stage_status(
                self.db.clone(),
                vehicles.ERU.stages[0].stage_id,
                "Active",
            ).await.expect("Failed to update stage status");

            // Send search area for ERU only if it has valid coordinates
            let search_area = &vehicles.ERU.stages[0].search_area;
            if search_area.len() >= 3 {  // Only send if we have at least 3 coordinates
                let coords: Vec<GeoCoordinate> = search_area.iter()
                    .take(6)
                    .map(|coord| GeoCoordinate {
                        lat: coord.lat,
                        long: coord.long,
                    })
                    .collect();
                
                commands_api.clone().send_zone_update("ERU".to_string(), "4".to_string(), coords).await?;
            }
        }
        
        if !vehicles.MRA.stages.is_empty() {
            vehicles.MRA.stages[0].stage_status = MissionStageStatusEnum::Active;
            update_stage_status(
                self.db.clone(),
                vehicles.MRA.stages[0].stage_id,
                "Active",
            ).await.expect("Failed to update stage status");

            // Send search area for MRA only if it has valid coordinates
            let search_area = &vehicles.MRA.stages[0].search_area;
            if search_area.len() >= 3 {  // Only send if we have at least 3 coordinates
                let coords: Vec<GeoCoordinate> = search_area.iter()
                    .take(6)
                    .map(|coord| GeoCoordinate {
                        lat: coord.lat,
                        long: coord.long,
                    })
                    .collect();
                
                commands_api.clone().send_zone_update("MRA".to_string(), "4".to_string(), coords).await?;
            }
        }
        
        // Final state update after all changes
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
        )
        .await
        .expect("Failed to update auto mode in database");

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
        )
        .await
        .expect("Failed to find vehicle mission");

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
        
        let vehicle_id = select_vehicle_from_mission(
            self.db.clone(),
            mission.mission_id,
            vehicle.vehicle_name.to_string(),
        ).await.expect("Failed to find vehicle mission");

        let current_stage_id = update_stage_area(
            self.db.clone(),
            stage.stage_id,
            search_area_array,
            vehicle_id,
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
        delete_stage(self.db.clone(), stage_id)
            .await
            .expect("Failed to delete stage from database");

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

        update_stage_name(self.db.clone(), stage.stage_id, &stage_name)
            .await
            .expect("Failed to update stage name");

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
        let commands_api = CommandsApiImpl::default();
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
        )
        .await
        .expect("Failed to transition stage");

        println!(
            "After Transition Stage: {:?}",
            transitioned_stage.unwrap_or(vehicle.current_stage)
        );

        if let Some(stage) = vehicle.stages.iter_mut().find(|s| s.stage_id == transitioned_stage.unwrap_or(vehicle.current_stage)) {
            vehicle.current_stage = transitioned_stage.unwrap_or(vehicle.current_stage);
            stage.stage_status = MissionStageStatusEnum::Active;

            // Send search area for the new active stage if it has valid coordinates
            if stage.search_area.len() >= 3 {  // Only send if we have at least 3 coordinates
                let coords: Vec<GeoCoordinate> = stage.search_area.iter()
                    .take(6) // Limit to 6 points
                    .map(|coord| GeoCoordinate {
                        lat: coord.lat,
                        long: coord.long,
                    })
                    .collect();
                
                // Send search area (commandID: 4) to the specific vehicle
                commands_api.clone().send_zone_update(
                    vehicle.vehicle_name.to_string(),
                    "4".to_string(),
                    coords
                ).await?;
            }
        } else {
            println!("No next stage available");
        }

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
        println!(
            "Deleting zone of type: {:?} at index: {}",
            zone_type, zone_index
        );
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
    
    // async fn on_updated(new_data: MissionsStruct) {
    //     todo!()
    // }
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
fn convert_zone_to_json(zone_str: &str) -> String {
    // Remove brackets and whitespace
    let content = zone_str
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .trim();

    // Parse each coordinate pair
    let coords: Vec<String> = content
        .split(',')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunk| {
            let lat = chunk[0]
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .parse::<f64>()
                .unwrap_or(0.0);
            let long = chunk[1]
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .parse::<f64>()
                .unwrap_or(0.0);
            format!(r#"{{"lat":{:.5},"long":{:.5}}}"#, lat, long)
        })
        .collect();

    format!("[{}]", coords.join(","))
}
