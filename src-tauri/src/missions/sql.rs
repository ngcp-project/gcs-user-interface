use sqlx::{query, PgPool, Row};

pub async fn insert_new_mission(
    db_conn: PgPool,
    mission_name: &str,
) -> Result<i32, sqlx::Error> {
    let new_mission = query("
        INSERT INTO missions(mission_name, keep_in_zones, keep_out_zones) 
        VALUES ($1, $2, $3) RETURNING mission_id
    ")
    .bind(mission_name)
    .bind(&Vec::<String>::new())
    .bind(&Vec::<String>::new())
    .fetch_one(&db_conn)
    .await
    .expect("Failed to insert dummy data into missions");

    let new_mission_id: i32 = new_mission.get::<i32, _>("mission_id");

    let _insert_mra_vehicle = query("
        INSERT INTO vehicles(mission_id, vehicle_name, current_stage_id)
        VALUES ($1, $2, $3) RETURNING vehicle_id
    ")
    .bind(new_mission_id)
    .bind("MRA")
    .bind(-1)
    .fetch_one(&db_conn)
    .await
    .expect("Failed to insert dummy data into vehicles");
    

    let _insert_eru_vehicle = query("
        INSERT INTO vehicles(mission_id, vehicle_name, current_stage_id)
        VALUES ($1, $2, $3) RETURNING vehicle_id
    ")
    .bind(new_mission_id)
    .bind("ERU")
    .bind(-1)
    .fetch_one( &db_conn)
    .await
    .expect("Failed to insert dummy data into vehicles");


    let _insert_mea_vehicle = query("
        INSERT INTO vehicles(mission_id, vehicle_name, current_stage_id)
        VALUES ($1, $2, $3) RETURNING vehicle_id
    ")
    .bind(new_mission_id)
    .bind("MEA")
    .bind(-1)
    .fetch_one( &db_conn)
    .await
    .expect("Failed to insert dummy data into vehicles");

    return Ok(new_mission_id);
    
}


pub async fn update_mission_name(
    db_conn: PgPool,
    mission_id: i32,
    new_mission_name: &str,
) -> Result<(), sqlx::Error> {
    query("
        UPDATE missions SET mission_name = $1 WHERE mission_id = $2
    ")
    .bind(new_mission_name)
    .bind(mission_id)
    .execute(&db_conn)
    .await
    .expect("Failed to update mission name");

    Ok(())
}

pub async fn delete_mission(
    db_conn: PgPool,
    mission_id: i32,
) -> Result<(), sqlx::Error> {
    query("
        DELETE FROM missions WHERE mission_id = $1
    ")
    .bind(mission_id)
    .execute(&db_conn)
    .await
    .expect("Failed to delete mission");

    Ok(())
}


pub async fn select_vehicle_from_mission(
    db_conn: PgPool,
    mission_id: i32,
    vehicle_name: String,
) -> Result<i32, sqlx::Error> {
    let vehicle = query("
        SELECT vehicle_id FROM vehicles WHERE mission_id = $1 AND vehicle_name = $2
    ")
    .bind(mission_id)
    .bind(vehicle_name)
    .fetch_one(&db_conn)
    .await
    .expect("Failed to find vehicle in mission");

    let vehicle_id: i32 = vehicle.get::<i32, _>("vehicle_id");

    return Ok(vehicle_id);
}


pub async fn insert_new_stage(
    db_conn: PgPool,
    vehicle_id: i32,
    stage_name: &str,
) -> Result<i32, sqlx::Error> {
    let new_stage = query("
        INSERT INTO stages(vehicle_id, stage_name) 
        VALUES ($1, $2) RETURNING stage_id
    ")
    .bind(vehicle_id)
    .bind(stage_name)
    .fetch_one(&db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let new_stage_id: i32 = new_stage.get::<i32, _>("stage_id");

    // set current stage id if previously didn't exitst (-1)
    let current_stage = query("
        SELECT current_stage_id FROM vehicles WHERE vehicle_id = $1
    ")
    .bind(vehicle_id)
    .fetch_one(&db_conn)
    .await
    .expect("Failed to find vehicle in mission");

    if current_stage.get::<i32, _>("current_stage_id") == -1 {
        query("
            UPDATE vehicles SET current_stage_id = $1 WHERE vehicle_id = $2
        ")
        .bind(new_stage_id)
        .bind(vehicle_id)
        .execute(&db_conn)
        .await
        .expect("Failed to update vehicle current stage id");

        println!("Updated vehicle current stage id to {}", new_stage_id);
    }

    return Ok(new_stage_id);
}


pub async fn delete_stage(
    db_conn: PgPool,
    stage_id: i32,
) -> Result<(), sqlx::Error> {
    query("
        DELETE FROM stages WHERE stage_id = $1
    ")
    .bind(stage_id)
    .execute(&db_conn)
    .await
    .expect("Failed to delete stage");

    Ok(())
}


pub async fn update_stage_name(
    db_conn: PgPool,
    stage_id: i32,
    new_stage_name: &str,
) -> Result<(), sqlx::Error> {
    query("
        UPDATE stages SET stage_name = $1 WHERE stage_id = $2
    ")
    .bind(new_stage_name)
    .bind(stage_id)
    .execute(&db_conn)
    .await
    .expect("Failed to update stage name");

    Ok(())
}


pub async fn update_auto_mode_vehicle(
    db_conn: PgPool,
    mission_id: i32,
    vehicle_name: String,
    is_auto: bool,
) -> Result<(), sqlx::Error> {
    query("
        UPDATE vehicles SET auto_mode = $1 WHERE vehicle_name = $2 AND mission_id = $3
    ")
    .bind(is_auto)
    .bind(vehicle_name)
    .bind(mission_id)
    .execute(&db_conn)
    .await
    .expect("Failed to update vehicle auto mode");

    Ok(())
}


pub async fn transition_stage(
    db_conn: PgPool,
    mission_id: i32,
    vehicle_name: String,
    current_stage_id: i32,
) -> Result<Option<i32>, sqlx::Error> {
    let rows = query("
        SELECT stage_id
        FROM stages
        WHERE vehicle_id = (
            SELECT vehicle_id
            FROM vehicles
            WHERE mission_id = $1 AND vehicle_name = $2
        )
        ORDER BY stage_id
    ")
    .bind(mission_id)
    .bind(vehicle_name.clone())
    .fetch_all(&db_conn)
    .await
    .expect("Failed to select stage_id");

    let stage_ids: Vec<i32> = rows.iter().map(|row| row.get("stage_id")).collect(); 

    // get next index
    if let Some(pos) = stage_ids.iter().position(|&id| id == current_stage_id) {    
        if let Some(&next_stage_id) = stage_ids.get(pos + 1) {
            query("
                UPDATE vehicles
                SET current_stage_id = $1
                WHERE mission_id = $2 AND vehicle_name = $3
            ")
            .bind(next_stage_id)
            .bind(mission_id)
            .bind(vehicle_name)
            .execute(&db_conn)
            .await
            .expect("Failed to transition next stage");

            return Ok(Some(next_stage_id));
        }
    }

    Ok(None)
}