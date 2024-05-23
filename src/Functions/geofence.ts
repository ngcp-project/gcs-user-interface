import {LatLngExpression} from "leaflet";

interface Coordinates {
    latitude: number;
    longitude: number;
}

let zoneInPolygons = [] as LatLngExpression[];
let zoneOutPolygons = [] as LatLngExpression[];

export function pushZoneInPolygons(coordinate: Coordinates) {
    zoneInPolygons.push([coordinate]);
    console.log("FROM GEOFENCE.TS: " + zoneInPolygons);
}

export function pushZoneOutPolygons(newZoneOutPolygons: LatLngExpression[]) {
    zoneOutPolygons = newZoneOutPolygons;
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
export function isInKeepInZone(coordinate: Coordinates) {
    let isInZone = false;
    for (let i = 0; i < zoneInPolygons.length; i++) {
        if (isPointInPolygon(coordinate, zoneInPolygons[i])) {
            isInZone = true;
            break;
        }
    }
    console.log("IS IN KEEP IN ZONE: " + isInZone);
    return isInZone;
}

// Want this to return false
export function isInKeepOutZone(coordinate: Coordinates) {
    let isInZone = false;
    for (let i = 0; i < zoneOutPolygons.length; i++) {
        if (isPointInPolygon(coordinate, zoneOutPolygons[i])) {
            isInZone = true;
            break;
        }
    }
    console.log("IS IN KEEP OUT ZONE: " + isInZone);
    return isInZone;

}

function isPointInPolygon(point: Coordinates, polygon: LatLngExpression) {
    var isInside = false;
    var minX = polygon[0].x, maxX = polygon[0].x;
    var minY = polygon[0].y, maxY = polygon[0].y;
    for (var n = 1; n < polygon.length; n++) {
        var q = polygon[n];
        minX = Math.min(q.x, minX);
        maxX = Math.max(q.x, maxX);
        minY = Math.min(q.y, minY);
        maxY = Math.max(q.y, maxY);
    }

    if (point.latitude < minX || point.latitude > maxX || point.longitude < minY || point.longitude > maxY) {
        return false;
    }

    
    for (var i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
        if ( (polygon[i].y > point.longitude) != (polygon[j].y > point.longitude) &&
        point.latitude < (polygon[j].x - polygon[i].x) * (point.longitude - polygon[i].y) / (polygon[j].y - polygon[i].y) + polygon[i].x ) {
            isInside = !isInside;
        }
    }

    return isInside;
}

