import { LatLngTuple as LatLng } from "leaflet";

// Define polygon type as array of LatLng obj
type Polygon = LatLng[];

let zoneInPolygons: Polygon[] = [];
let zoneOutPolygons: Polygon[] = [];
let ERU_coords: LatLng;
let MEA_coords: LatLng;
let MRA_coords: LatLng;
let FRA_coords: LatLng;

export function updateVehicleCords(vehicle: string, coordinate: LatLng) {
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

export function pushZoneInPolygons(coordinates: LatLng[]) {
  zoneInPolygons.push(coordinates);
}

export function pushZoneOutPolygons(coordinates: LatLng[]) {
  zoneOutPolygons.push(coordinates);
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
export function isInKeepInZone(coordinate: LatLng) {
  let isInZone = false;
  for (let i = 0; i < zoneInPolygons.length; i++) {
    if (isPointInPolygon(coordinate, zoneInPolygons[i])) {
      isInZone = true;
      break;
    }
  }
  return isInZone;
}

// Want this to return false
export function isInKeepOutZone(coordinate: LatLng) {
  let isInZone = false;
  for (let i = 0; i < zoneOutPolygons.length; i++) {
    if (isPointInPolygon(coordinate, zoneOutPolygons[i])) {
      isInZone = true;
      break;
    }
  }
  return isInZone;
}

function isPointInPolygon(point: LatLng, polygon: Polygon) {
  if (!polygon.length) {
    return false;
  }

  let isInside = false;
  const [firstLat, firstLng] = polygon[0];
  
  let minLat = firstLat,
      maxLat = firstLat;
  let minLng = firstLng,
      maxLng = firstLng;

  for (let n = 1; n < polygon.length; n++) {
    const [lat, lng] = polygon[n];
    minLat = Math.min(lat, minLat);
    maxLat = Math.max(lat, maxLat);
    minLng = Math.min(lng, minLng);
    maxLng = Math.max(lng, maxLng);
  }

  const [pointLat, pointLng] = point;

  if (pointLat < minLat || pointLat > maxLat || pointLng < minLng || pointLng > maxLng) {
    return false;
  }

  let i = 0,
    j = polygon.length - 1;

  for (; i < polygon.length; j = i++) {
    const [iLat, iLng] = polygon[i];
    const [jLat, jLng] = polygon[j];
    
    if  (
      iLng > pointLng != jLng > pointLng &&
      pointLat <
        ((jLat - iLat) * (pointLng - iLng)) /
          (jLng - iLng) +
          iLat
    )  {
      isInside = !isInside;
    }
  }

  return isInside;
}
