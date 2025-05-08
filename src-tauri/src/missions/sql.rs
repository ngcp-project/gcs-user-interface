use sqlx::{query, PgPool, Row};

pub async fn insert_new_mission(
    db_conn: PgPool,
    mission_name: &str,
) -> Result<i32, sqlx::Error> {
    let new_mission = query("
        INSERT INTO missions(mission_name, keep_in_zones, keep_out_zones, status) 
        VALUES ($1, $2, $3, $4::status) RETURNING mission_id
    ")
    .bind(mission_name)
    .bind(&Vec::<String>::new())
    .bind(&Vec::<String>::new())
    .bind("Inactive")
    .fetch_one(&db_conn)
    .await
    .expect("Failed to insert dummy data into missions");

    let new_mission_id: i32 = new_mission.get::<i32, _>("mission_id");

    let insert_mra_vehicle = query("
        INSERT INTO vehicles(mission_id, vehicle_name, current_stage_id)
        VALUES ($1, $2, $3) RETURNING vehicle_id
    ")
    .bind(new_mission_id)
    .bind("MRA")
    .bind(-1)
    .fetch_one(&db_conn)
    .await
    .expect("Failed to insert dummy data into vehicles");
    
    // let mra_vehicle_id: i32 = insert_mra_vehicle.get::<i32, _>("vehicle_id");

    let insert_eru_vehicle = query("
        INSERT INTO vehicles(mission_id, vehicle_name, current_stage_id)
        VALUES ($1, $2, $3) RETURNING vehicle_id
    ")
    .bind(new_mission_id)
    .bind("ERU")
    .bind(-1)
    .fetch_one( &db_conn)
    .await
    .expect("Failed to insert dummy data into vehicles");

    // let eru_vehicle_id: i32 = insert_eru_vehicle.get::<i32, _>("vehicle_id");

    let insert_mea_vehicle = query("
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

    return Ok(new_stage_id);
}