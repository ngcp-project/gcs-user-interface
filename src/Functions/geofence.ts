import { LatLngExpression } from "leaflet";
import { core } from "@tauri-apps/api";

interface Coordinates {
  latitude: number;
  longitude: number;
}

let zoneInPolygons: Coordinates[][] = [];
let zoneOutPolygons: Coordinates[][] = [];
let ERU_coords: number[];
let MEA_coords: number[];
let MRA_coords: number[];
let FRA_coords: number[];

export function updateVehicleCords(vehicle: string, coordinate: number[]) {
  switch (vehicle) {
    case "ERU":
      ERU_coords = coordinate;
      break;
    case "MEA":
      MEA_coords = coordinate;
      break;
    case "MRA":
      MRA_coords = coordinate;
      break;
    case "FRA":
      FRA_coords = coordinate;
      break;
  }
}

export function pushZoneInPolygons(coordinate: Coordinates) {
  zoneInPolygons.push([coordinate]);
  // console.log("FROM GEOFENCE.TS: " + zoneInPolygons);
}

export function pushZoneOutPolygons(coordinate: Coordinates) {
  zoneOutPolygons.push([coordinate]);
}

export function clearZoneInPolygons() {
  zoneInPolygons = [];
}

export function clearZoneOutPolygons() {
  zoneOutPolygons = [];
}

export function clearPolygons() {
  zoneInPolygons = [];
  zoneOutPolygons = [];
}

// Want this to return true
export function isInKeepInZone(coordinate: number[]) {
  // console.log("ZONE COORDS: " + zoneInPolygons)
  let isInZone = false;
  // console.log("ZONE IN LENGTH: " + zoneInPolygons.length);
  for (let i = 0; i < zoneInPolygons.length; i++) {
    if (isPointInPolygon(coordinate, zoneInPolygons[i])) {
      isInZone = true;
      break;
    }
  }
  // console.log("IS IN KEEP IN ZONE: " + isInZone);
  return isInZone;
}

// Want this to return false
export function isInKeepOutZone(coordinate: number[]) {
  let isInZone = false;
  // console.log("ZONE OUT LENGTH: " + zoneOutPolygons.length);
  for (let i = 0; i < zoneOutPolygons.length; i++) {
    // co/nsole.log("ZONE OUT COORDS: " + zoneOutPolygons[i]);
    if (isPointInPolygon(coordinate, zoneOutPolygons[i])) {
      isInZone = true;
      break;
    }
  }
  // console.log("IS IN KEEP OUT ZONE: " + isInZone);
  return isInZone;
}

export function KeepOutZones(vehicle: string) {
  const polygons = zoneOutPolygons.map((poly) => ({
    vehicle_id: vehicle.toLowerCase(),
    polygon: Array.isArray(poly) ? poly.map((coord) => [coord.latitude, coord.longitude]) : []
  }));

  console.log("📤 Syncing polygons for:", vehicle);
  console.log("🧱 Payload:", polygons);

  core.invoke("update_keep_out_zone", { data: polygons }).catch((e) =>
    console.error("❌ Failed to sync zones:", e)
  );
}

function isPointInPolygon(point: number[], polygons: LatLngExpression) {
  let isInside = false;
  const polygon = polygons[0];
  let minX = polygon[0][0],
    maxX = polygon[0][0];
  let minY = polygon[0][0],
    maxY = polygon[0][0];
  // console.log(polygon[0]);
  // console.log(minX + " " + maxX + " " + minY + " " + maxY);
  for (let n = 1; n < polygon.length; n++) {
    const q = polygon[n];
    minX = Math.min(q[0], minX);
    maxX = Math.max(q[0], maxX);
    minY = Math.min(q[1], minY);
    maxY = Math.max(q[1], maxY);
  }

  if (point[0] < minX || point[0] > maxX || point[1] < minY || point[1] > maxY) {
    // console.log("1111111111111111111");
    return false;
  }

  let i = 0,
    j = polygon.length - 1;
  // console.log(polygon);
  // console.log("000000");
  for (; i < polygon.length; j = i++) {
    // console.log("1111111111");
    if (
      polygon[i][1] > point[1] != polygon[j][1] > point[1] &&
      point[0] <
        ((polygon[j][0] - polygon[i][0]) * (point[1] - polygon[i][1])) /
          (polygon[j][1] - polygon[i][1]) +
          polygon[i][0]
    ) {
      isInside = !isInside;
      // console.log("123123123123123123");
      // console.log(isInside + " ");
    }
  }
  // console.log("222222");

  return isInside;
}
