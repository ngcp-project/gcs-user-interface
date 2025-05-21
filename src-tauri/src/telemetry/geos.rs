use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Clone, Debug)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct PolygonDTO {
    pub vehicle_id: String,
    pub polygon: Vec<(f64, f64)>,
}

lazy_static! {
    pub static ref KEEP_OUT_ZONES: RwLock<HashMap<String, Vec<Vec<Coordinate>>>> =
        RwLock::new(HashMap::new());
}

#[tauri::command]
pub fn update_keep_out_zone(data: Vec<PolygonDTO>) {
    let mut zones = KEEP_OUT_ZONES.write().unwrap();
    zones.clear();
    println!("📥 Received {} polygons to update", data.len());

    for dto in data {
        let key = dto.vehicle_id.to_lowercase();
        println!("🔧 Vehicle: {}, Points: {}", key, dto.polygon.len());

        if dto.polygon.len() >= 3 {
            let polygon = dto
                .polygon
                .iter()
                .map(|(lat, lon)| Coordinate {
                    latitude: *lat,
                    longitude: *lon,
                })
                .collect::<Vec<_>>();

            println!(
                "✅ Storing polygon for {}: {:?}",
                key,
                polygon
                    .iter()
                    .map(|c| (c.latitude, c.longitude))
                    .collect::<Vec<_>>()
            );

            zones.entry(key).or_default().push(polygon);
        } else {
            println!("⚠️ Skipped polygon for {}: too few points", key);
        }
    }
}

fn harversine_distance(a: &Coordinate, b: &Coordinate) -> f64 {
    let r = 6371000.0;
    let dlat = (b.latitude - a.latitude).to_radians();
    let dlon = (b.longitude - a.longitude).to_radians();

    let lat1 = a.latitude.to_radians();
    let lat2 = b.latitude.to_radians();

    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r * c
}

pub fn is_near_keep_out_zone(vehicle_id: &str, point: &Coordinate, threshold_m: f64) -> bool {
    let zones = KEEP_OUT_ZONES.read().unwrap();
    println!("🔍 Checking zones for vehicle: {}", vehicle_id);
    println!(
        "📍 Current position: ({}, {})",
        point.latitude, point.longitude
    );
    if let Some(polygons) = zones.get(&vehicle_id.to_lowercase()) {
        for polygon in polygons {
            for coord in polygon {
                let dist = harversine_distance(point, coord);
                if dist <= threshold_m {
                    return true;
                }
            }
        }
    } else {
        println!("⚠️ No zones registered for vehicle {}", vehicle_id);
    }
    return false;
}
