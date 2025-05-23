use sqlx::{query, PgPool};

pub async fn insert_telemetry(
    db_conn: PgPool,
    vehicle_id: String,
    signal_strength: i32,
    pitch: f32,
    yaw: f32,
    roll: f32,
    speed: f32,
    altitude: f32,
    battery_life: i32,
    current_position: String,
    status: String,
    request_coordinate: String,
) -> Result<(), sqlx::Error> {
    query("
        INSERT INTO telemetry(vehicle_id, signal_strength, pitch, yaw, roll, speed, altitude, battery_life, current_position, vehicle_status, request_coordinate)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
    ")
    .bind(vehicle_id)
    .bind(signal_strength)
    .bind(pitch)
    .bind(yaw)
    .bind(roll)
    .bind(speed)
    .bind(altitude)
    .bind(battery_life)
    .bind(current_position)
    .bind(status)
    .bind(request_coordinate)
    .execute(&db_conn)
    .await
    .expect("Failed to update vehicle status");

    Ok(())
}