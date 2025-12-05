import { LMap } from "@vue-leaflet/vue-leaflet";
import { LatLngTuple as LatLng, LatLngExpression } from "leaflet";
import * as L from "leaflet";
import { VehicleEnum } from "@/lib//bindings";

// =============================================
// Types and Interfaces
// =============================================
export interface LayerProperties {
  color: string;
  visibility: boolean;
}

export interface ZoneLayer {
  layer: L.Polygon;
  properties: LayerProperties;
}

export interface StageLayer {
  stageId: number;
  polygon: ZoneLayer;
  properties: LayerProperties;
}

export interface VehicleLayers {
  stages: Record<number, StageLayer>;
}

export interface MissionLayers {
  vehicles: Record<VehicleEnum, VehicleLayers>;
  zones: {
    KeepIn: (ZoneLayer | {})[];
    KeepOut: (ZoneLayer | {})[];
  };
}

export interface LayerTracking {
  missions: Record<number, MissionLayers>;
}

/**
 * Merge in vue-leaflet map type with geoman L.Map
 * since geoman automatically initializes to any leaflet map created
 * Access geoman methods through .leafletObject.pm
 **/
export type LeafletMap = InstanceType<typeof LMap>;
export interface LeafletMapGeoman extends LeafletMap {
  leafletObject: LeafletMap["leafletObject"] & L.Map;
}

// =============================================
// State Interface
// =============================================
export interface MapState {  
  map: LeafletMapGeoman | null;
  mapOrigin: LatLng;
  markerCoord: LatLngExpression;
  layers: L.FeatureGroup<L.Polygon>;
  localTileURL: string;
  layerTracking: LayerTracking;
  vehicleMarkers: Record<VehicleEnum, LatLngExpression>;
}

