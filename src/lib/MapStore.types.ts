import { LMap } from "@vue-leaflet/vue-leaflet";
import { LatLngTuple as LatLng, LatLngExpression } from "leaflet";
import * as L from "leaflet";
import { MissionsStruct, VehicleEnum, ZoneType } from "@/lib//bindings";

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
// Store Interface
// =============================================
export interface MapStore {
  map: LeafletMapGeoman | null;
  mapOrigin: LatLng;
  markerCoord: LatLngExpression;
  layers: L.FeatureGroup<L.Polygon>;
  localTileURL: string;
  layerTracking: LayerTracking;
  vehicleMarkers: Record<VehicleEnum, LatLngExpression>;

  // Map Management
  updateMapRef: (ref: LeafletMapGeoman | null) => void;
  toggleDrawMode: () => void;
  rerenderLayers: () => void;
  logMapStore: () => void;
  updateVehicleMarker: (vehicle: VehicleEnum, lat: number, lng: number) => void;
  updateMarkerCoords: (vehicle: VehicleEnum, coords: LatLngExpression) => void;
  getVehicleMarkers: () => Record<VehicleEnum, LatLngExpression>;

  // Layer Management
  updateStagePolygon: (missionId: number, vehicle: VehicleEnum, stageId: number) => void;
  updateZonePolygon: (missionId: number, type: ZoneType, zoneIndex: number) => void;
  removeStageLayer: (missionId: number, vehicle: VehicleEnum, stageId: number) => void;
  getStageLayer: (
    missionId: number,
    vehicle: VehicleEnum,
    stageId: number
  ) => StageLayer | undefined;
  getZoneLayers: (missionId: number, type: ZoneType) => ZoneLayer[];
  setZoneLayerVisibility: (missionId: number, type: ZoneType, zoneIndex: number) => void;
  setStageLayerVisibility: (missionId: number, vehicle: VehicleEnum, stageId: number) => void;
  // TODO: fix it so that theres no as declaration when reading from missionStore
  updateLayerTracking: (state: MissionsStruct) => void;
}