use sqlx::postgres::PgConnection;
use sqlx::Connection;
use sqlx::{query, Row};

const DB_URL: &str = "postgres://ngcp:ngcp@localhost:5433/ngcpdb";

pub async fn init_database_dummy_data() {
    let mut db_conn = PgConnection::connect(DB_URL)
        .await
        .expect("Failed to connect to the database");

    let insert_dummy_discover_mission = query(
        "
        INSERT INTO missions(mission_name, keep_in_zones, keep_out_zones, status) 
        VALUES ($1, $2, $3, $4) RETURNING mission_id
    ",
    )
    .bind("Discover Mission")
    .bind(&vec![
        // how the data is gonna look --> array of tuples:
        // [(latitude,longitude),etc.]
        r#"[
            (37.33285,-122.34302),
            (51.54564,-0.49298),
            (-33.78501,151.29494),
            (40.12456,-74.72894),
            (56.94295,3.97837)
        ]"#
        .to_string(),
        r#"[
            (48.33285,-73.34302),
            (-12.54564,103.49298),
            (21.78501,-88.29494),
            (59.12456,12.72894),
            (-4.94295,145.97837)
        ]"#
        .to_string(),
    ])
    .bind(&vec![
        r#"[
            (-41.23756,38.29417),
            (62.23701,-104.23486),
            (-16.98743,113.93240),
            (49.89453,-9.89456),
            (-33.12789,72.24690)
        ]"#
        .to_string(),
        r#"[
            (28.23847, 102.35892),
            (-12.98237, -44.23510),
            (45.23456, 8.65412),
            (-39.76892, 58.71245),
            (23.43258, -82.35821)
        ]"#
        .to_string(),
    ])
    .bind("Active")
    .fetch_one(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into missions");

    let discover_mission_id: i32 = insert_dummy_discover_mission.get::<i32, _>("mission_id");
    println!("Discover Mission ID: {}", discover_mission_id);

    let _insert_dummy_discover_mra = query(
        "
        INSERT INTO vehicles(mission_id, vehicle_name, current_stage_id)
        VALUES ($1, $2, $3) RETURNING vehicle_id
    ",
    )
    .bind(discover_mission_id)
    .bind("MRA")
    .bind(1)
    .fetch_one(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into vehicles");

    let insert_dummy_discover_mra_id: i32 = _insert_dummy_discover_mra.get::<i32, _>("vehicle_id");
    println!("Discover MRA Vehicle ID: {}", insert_dummy_discover_mra_id);

    let _insert_dummy_discover_eru = query(
        "
        INSERT INTO vehicles(mission_id, vehicle_name, current_stage_id)
        VALUES ($1, $2, $3) RETURNING vehicle_id
    ",
    )
    .bind(discover_mission_id)
    .bind("ERU")
    .bind(4)
    .fetch_one(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into vehicles");

    let insert_dummy_discover_eru_id: i32 = _insert_dummy_discover_eru.get::<i32, _>("vehicle_id");
    println!("Discover ERU Vehicle ID: {}", insert_dummy_discover_eru_id);

    let _insert_dummy_discover_mea = query(
        "
        INSERT INTO vehicles(mission_id, vehicle_name, current_stage_id)
        VALUES ($1, $2, $3) RETURNING vehicle_id
    ",
    )
    .bind(discover_mission_id)
    .bind("MEA")
    .bind(6)
    .fetch_one(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into vehicles");

    let insert_dummy_discover_mea_id: i32 = _insert_dummy_discover_mea.get::<i32, _>("vehicle_id");
    println!("Discover MEA Vehicle ID: {}", insert_dummy_discover_mea_id);

    let _insert_dummy_init_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate, status)
        VALUES ($1, $2, $3, $4, $5)
    ",
    )
    .bind(insert_dummy_discover_mra_id)
    .bind(&vec![r#"[
            (43.12876,-156.45231),
            (-12.89354,67.23418),
            (78.43219,-43.98765),
            (-34.56789,142.87654),
            (23.98765,-89.12345)
        ]"#
    .to_string()])
    .bind("Initial Stage")
    .bind(r#"(37.33285,-122.34302)"#.to_string())
    .bind("Active")
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_second_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_discover_mra_id)
    .bind(&vec![r#"[
            (67.34521,-134.89276),
            (-23.45678,88.12453),
            (41.98732,-92.45681),
            (-56.12398,167.34521),
            (12.45678,-157.89012)
        ]"#
    .to_string()])
    .bind("Second Stage")
    .bind(r#"(45.67891,-98.76543)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_third_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_discover_mra_id)
    .bind(&vec![r#"[
            (-34.56789,123.45678),
            (78.90123,-45.67890),
            (12.34567,-167.89012),
            (-67.89012,45.67890),
            (23.45678,-134.56789)
        ]"#
    .to_string()])
    .bind("Third Stage")
    .bind(r#"(-12.34567,145.67890)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_eru_init_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_discover_eru_id)
    .bind(&vec![r#"[
            (54.23451,-123.45612),
            (-15.78901,89.34567),
            (82.12345,-67.89012),
            (-45.67890,156.78901),
            (31.23456,-145.67890)
        ]"#
    .to_string()])
    .bind("init")
    .bind(r#"(42.56789,-134.23456)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_eru_second_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_discover_eru_id)
    .bind(&vec![r#"[
            (76.45678,-178.90123),
            (-34.56789,112.34567),
            (23.45678,-89.01234),
            (-67.89012,145.67890),
            (45.67890,-123.45678)
        ]"#
    .to_string()])
    .bind("Second Stage")
    .bind(r#"(56.78901,-167.89012)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    // Stages for MEA (vehicle_id = 3)
    let _insert_dummy_mea_init_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_discover_mea_id)
    .bind(&vec![r#"[
            (65.43210,-145.67890),
            (-28.90123,134.56789),
            (43.21098,-78.90123),
            (-56.78901,167.89012),
            (34.56789,-112.34567)
        ]"#
    .to_string()])
    .bind("init")
    .bind(r#"(51.23456,-156.78901)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_mea_second_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_discover_mea_id)
    .bind(&vec![r#"[
            (71.23456,-167.89012),
            (-45.67890,123.45678),
            (34.56789,-145.67890),
            (-78.90123,178.90123),
            (23.45678,-134.56789)
        ]"#
    .to_string()])
    .bind("Second Stage")
    .bind(r#"(67.89012,-178.90123)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    // NEW MISSION INSERTION

    let _insert_dummy_retrieve_mission = query(
        "
        INSERT INTO missions(mission_name, keep_in_zones, keep_out_zones, status) 
        VALUES ($1, $2, $3, $4) RETURNING mission_id
    ",
    )
    .bind("Retrieve Mission")
    .bind(&vec![
        r#"[
            (5.23657,-68.74629),
            (33.54321,-101.59834),
            (-28.23471,85.94732),
            (12.59481,77.24362),
            (-53.78192,124.87453)
        ]"#
        .to_string(),
        r#"[
            (49.23849,-87.15234),
            (-13.78657,-102.43578),
            (61.18436,17.94861),
            (21.38940,-13.24867),
            (-45.89267,122.73901)
        ]"#
        .to_string(),
    ])
    .bind(&vec![
        r#"[
            (34.54319,101.63489),
            (-5.89234,56.23418),
            (28.95762,-115.72139),
            (-50.34217,32.94123),
            (13.98312,-79.87655)
        ]"#
        .to_string(),
        r#"[
            (-26.19243,110.73284),
            (62.98123,-43.89357),
            (-35.78420,99.28964),
            (22.84656,-68.12345),
            (48.23950,79.56439)
        ]"#
        .to_string(),
    ])
    .bind("Inactive")
    .fetch_one(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into missions");
    let retrieve_mission_id: i32 = _insert_dummy_retrieve_mission.get::<i32, _>("mission_id");
    println!("Retrieve Mission ID: {}", retrieve_mission_id);

    let _insert_dummy_retrieve_mra = query(
        "
        INSERT INTO vehicles(mission_id, vehicle_name, current_stage_id)
        VALUES ($1, $2, $3) RETURNING vehicle_id
    ",
    )
    .bind(retrieve_mission_id)
    .bind("MRA")
    .bind(8)
    .fetch_one(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into vehicles");

    let insert_dummy_retrieve_mra_id: i32 = _insert_dummy_retrieve_mra.get::<i32, _>("vehicle_id");
    println!("Retrieve MRA Vehicle ID: {}", insert_dummy_retrieve_mra_id);

    let _insert_dummy_retrieve_eru = query(
        "
        INSERT INTO vehicles(mission_id, vehicle_name, current_stage_id)
        VALUES ($1, $2, $3) RETURNING vehicle_id
    ",
    )
    .bind(retrieve_mission_id)
    .bind("ERU")
    .bind(11)
    .fetch_one(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into vehicles");

    let insert_dummy_retrieve_eru_id: i32 = _insert_dummy_retrieve_eru.get::<i32, _>("vehicle_id");
    println!("Retrieve ERU Vehicle ID: {}", insert_dummy_retrieve_eru_id);

    let _insert_dummy_retrieve_mea = query(
        "
        INSERT INTO vehicles(mission_id, vehicle_name, current_stage_id)
        VALUES ($1, $2, $3) RETURNING vehicle_id
    ",
    )
    .bind(retrieve_mission_id)
    .bind("MEA")
    .bind(13)
    .fetch_one(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into vehicles");

    let insert_dummy_retrieve_mea_id: i32 = _insert_dummy_retrieve_mea.get::<i32, _>("vehicle_id");
    println!("Retrieve MEA Vehicle ID: {}", insert_dummy_retrieve_mea_id);

    let _insert_dummy_retrieve_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_retrieve_mra_id)
    .bind(&vec![r#"[
            (12.34567,-78.90123),
            (-45.67890,123.45678),
            (34.56789,-145.67890),
            (-78.90123,178.90123),
            (23.45678,-134.56789)
        ]"#
    .to_string()])
    .bind("Initial Stage")
    .bind(r#"(5.23657,-68.74629)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_retrieve_second_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_retrieve_mra_id)
    .bind(&vec![r#"[
            (67.89012,-123.45678),
            (-34.56789,112.34567),
            (23.45678,-89.01234),
            (-67.89012,145.67890),
            (45.67890,-123.45678)
        ]"#
    .to_string()])
    .bind("Second Stage")
    .bind(r#"(49.23849,-87.15234)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_retrieve_third_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_retrieve_mra_id)
    .bind(&vec![r#"[
            (78.90123,-45.67890),
            (12.34567,-167.89012),
            (-67.89012,45.67890),
            (23.45678,-134.56789),
            (-34.56789,123.45678)
        ]"#
    .to_string()])
    .bind("Third Stage")
    .bind(r#"(-12.34567,145.67890)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_eru_init_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_retrieve_eru_id)
    .bind(&vec![r#"[
            (54.23451,-123.45612),
            (-15.78901,89.34567),
            (82.12345,-67.89012),
            (-45.67890,156.78901),
            (31.23456,-145.67890)
        ]"#
    .to_string()])
    .bind("init")
    .bind(r#"(42.56789,-134.23456)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_eru_second_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_retrieve_eru_id)
    .bind(&vec![r#"[
            (76.45678,-178.90123),
            (-34.56789,112.34567),
            (23.45678,-89.01234),
            (-67.89012,145.67890),
            (45.67890,-123.45678)
        ]"#
    .to_string()])
    .bind("Second Stage")
    .bind(r#"(56.78901,-167.89012)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_mea_init_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_retrieve_mea_id)
    .bind(&vec![r#"[
            (65.43210,-145.67890),
            (-28.90123,134.56789),
            (43.21098,-78.90123),
            (-56.78901,167.89012),
            (34.56789,-112.34567)
        ]"#
    .to_string()])
    .bind("init")
    .bind(r#"(51.23456,-156.78901)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_mea_second_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_retrieve_mea_id)
    .bind(&vec![r#"[
            (71.23456,-167.89012),
            (-45.67890,123.45678),
            (34.56789,-145.67890),
            (-78.90123,178.90123),
            (23.45678,-134.56789)
        ]"#
    .to_string()])
    .bind("Second Stage")
    .bind(r#"(67.89012,-178.90123)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    let _insert_dummy_mea_third_stage = query(
        "
        INSERT INTO stages(vehicle_id, search_area, stage_name, target_coordinate)
        VALUES ($1, $2, $3, $4)
    ",
    )
    .bind(insert_dummy_retrieve_mea_id)
    .bind(&vec![r#"[
            (78.90123,-45.67890),
            (12.34567,-167.89012),
            (-67.89012,45.67890),
            (23.45678,-134.56789),
            (-34.56789,123.45678)
        ]"#
    .to_string()])
    .bind("Third Stage")
    .bind(r#"(-12.34567,145.67890)"#.to_string())
    .execute(&mut db_conn)
    .await
    .expect("Failed to insert dummy data into stages");

    db_conn
        .close()
        .await
        .expect("Failed to close database connection");
}

pub async fn clear_database() {
    let mut db_conn = PgConnection::connect(DB_URL)
        .await
        .expect("Failed to connect to the database");

    let _cleanup_mission = query(
        "
    DROP TABLE IF EXISTS missions CASCADE;
    ",
    )
    .execute(&mut db_conn)
    .await
    .expect("Failed to execute query");

    let _cleanup_vehicle = query(
        "
    DROP TABLE IF EXISTS vehicles CASCADE;
    ",
    )
    .execute(&mut db_conn)
    .await
    .expect("Failed to execute query");

    let _cleanup_stage = query(
        "
    DROP TABLE IF EXISTS stages CASCADE;
    ",
    )
    .execute(&mut db_conn)
    .await
    .expect("Failed to execute query");

    db_conn
        .close()
        .await
        .expect("Failed to close database connection");
}

pub async fn initialize_database() {
    let mut db_conn = PgConnection::connect(DB_URL)
        .await
        .expect("Failed to connect to the database");

    let _create_mission_table = query(
        "
    CREATE TABLE IF NOT EXISTS missions (
        mission_id SERIAL PRIMARY KEY,
        mission_name VARCHAR(255),
        keep_in_zones TEXT[] NOT NULL,
        keep_out_zones TEXT[] NOT NULL,
        status TEXT DEFAULT 'Inactive'
    );
    ",
    )
    .execute(&mut db_conn)
    .await
    .expect("Failed to create table 'missions'");

    let _create_vehicle_table = query(
        "
    CREATE TABLE IF NOT EXISTS vehicles (
        mission_id INTEGER REFERENCES missions ON DELETE CASCADE,
        vehicle_id SERIAL UNIQUE,
        vehicle_name VARCHAR(255) NOT NULL,
        current_stage_id INTEGER NOT NULL,
        is_auto BOOLEAN DEFAULT FALSE,
        patient_status VARCHAR(255) DEFAULT 'Unsecured',
        PRIMARY KEY (mission_id, vehicle_id)
    );
    ",
    )
    .execute(&mut db_conn)
    .await
    .expect("Failed to execute query");

    let _create_stage_table = query(
        "
    CREATE TABLE IF NOT EXISTS stages (
        stage_id SERIAL PRIMARY KEY,
        vehicle_id INTEGER REFERENCES vehicles(vehicle_id) ON DELETE CASCADE,
        search_area TEXT[],      
        stage_name VARCHAR(255) NOT NULL,
        target_coordinate TEXT,
        status TEXT DEFAULT 'Inactive'
    );
    ",
    )
    .execute(&mut db_conn)
    .await
    .expect("Failed to execute query");

    let _create_telemetry_table = query(
        "
    CREATE TABLE IF NOT EXISTS telemetry (
        vehicle_id TEXT,
        signal_strength INTEGER,
        pitch FLOAT,
        yaw FLOAT,
        roll FLOAT,
        speed FLOAT,
        altitude FLOAT,
        battery_life INTEGER,
        current_position TEXT,
        vehicle_status TEXT,
        request_coordinate TEXT    
    );
    ",
    )
    .execute(&mut db_conn)
    .await
    .expect("Failed to execute query");

    db_conn
        .close()
        .await
        .expect("Failed to close database connection");
}