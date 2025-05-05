import { createStore } from "zustand/vanilla";
import { reactive } from "vue";
import { LMap } from "@vue-leaflet/vue-leaflet";
import { LatLngTuple as LatLng } from "leaflet";
import * as L from "leaflet";
import { GeoCoordinateStruct, MissionsStruct, VehicleEnum, ZoneType } from "@/lib//bindings";
import { missionStore } from "./MissionStore";
import { watch } from "vue";

// =============================================
// Types and Interfaces
// =============================================
interface LayerProperties {
  color: string;
  visibility: boolean;
}

interface ZoneLayer {
  layer: L.Polygon;
  properties: LayerProperties;
}

interface StageLayer {
  stageId: number;
  polygon: ZoneLayer | {};
}

interface VehicleLayers {
  stages: Record<number, StageLayer>;
}

interface MissionLayers {
  vehicles: Record<VehicleEnum, VehicleLayers>;
  zones: {
    KeepIn: (ZoneLayer | {})[];
    KeepOut: (ZoneLayer | {})[];
  };
}

interface LayerTracking {
  missions: Record<number, MissionLayers>;
}

/** 
 * Merge in vue-leaflet map type with geoman L.Map
 * since geoman automatically initializes to any leaflet map created
 * Access geoman methods through .leafletObject.pm
**/ 
type LeafletMap = InstanceType<typeof LMap>;
export interface LeafletMapGeoman extends LeafletMap {
  leafletObject: LeafletMap["leafletObject"] & L.Map;
}

// =============================================
// Constants
// =============================================
const LAYER_STYLING = {
  KeepIn: { color: "#00FF00" },
  KeepOut: { color: "#FF0000" },
  ERU: { color: "#0000FF" },
  MEA: { color: "#FF00FF" },
  MRA: { color: "#00FFFF" }
} as const;

const DEFAULT_MAP_ORIGIN: LatLng = [35.33004319829399, -120.75064544958856];
const DEFAULT_ZOOM = 16;
const TILE_URL = "http://localhost:8080/tile/{z}/{x}/{y}.png";

// =============================================
// Store Interface
// =============================================
interface MapStore {
  map: LeafletMapGeoman | null;
  mapOrigin: LatLng;
  zoom: number;
  layers: L.FeatureGroup<L.Polygon>;
  localTileURL: string;
  layerTracking: LayerTracking;
  
  // Map Management
  updateMapRef: (ref: LeafletMapGeoman | null) => void;
  toggleDrawMode: () => void;
  rerenderLayers: () => void;
  logMapStore: () => void;
  
  // Layer Management
  addStageLayer: (missionId: number, vehicle: VehicleEnum, stageId: number, polygon: L.Polygon) => void;
  updateZonePolygon: (missionId: number, type: ZoneType, zoneIndex: number) => void;
  removeStageLayer: (missionId: number, vehicle: VehicleEnum, stageId: number) => void;
  removeZoneLayer: (missionId: number, type: ZoneType, index: number) => void;
  getStageLayer: (missionId: number, vehicle: VehicleEnum, stageId: number) => StageLayer | undefined;
  getZoneLayers: (missionId: number, type: ZoneType) => ZoneLayer[];
  // TODO: fix it so that theres no as declaration when reading from missionStore
  updateLayerTracking: (state: MissionsStruct) => void;
}

// =============================================
// Store Implementation
// =============================================
const mapStore = createStore<MapStore>((set, get) => ({
  map: null,
  mapOrigin: DEFAULT_MAP_ORIGIN,
  zoom: DEFAULT_ZOOM,
  localTileURL: TILE_URL,
  layerTracking: { missions: {} },
  layers: L.featureGroup([]),

  // Map Management Methods
  updateMapRef: (refValue: LeafletMapGeoman | null) => {
    // Should be called when map @ready event is initialized
    const mapLeaflet = refValue?.leafletObject;
    if (!mapLeaflet) return;
    
    set({ map: refValue });
    // Assign preInitialized geoJSON to the map
    get().layers.addTo(mapLeaflet);
    get().updateLayerTracking(missionStore.state as MissionsStruct);
  },
  
  toggleDrawMode: () => {
    // Draws a polygon not linked to layers
    get().map?.leafletObject?.pm.enableDraw("Polygon");
  },

  rerenderLayers: () => {
    // Clears all layers and rerenders them
    const map = get().map?.leafletObject;
    if (!map) return;

    get().layers.clearLayers();
    // TODO: Implement layer rerendering logic
  },

  logMapStore: () => {
    console.log(get());
  },

  // Layer Management Methods
  updateZonePolygon: (missionId, type, zoneIndex) => {
    const map = get().map?.leafletObject;
    if (!map) return;

    const layerTrackedZone = get().layerTracking.missions[missionId].zones[type][zoneIndex];

    // not pushing in zonelayer type, pushing in empty object or L.Polygon layer
    if (!layerTrackedZone || Object.keys(layerTrackedZone).length === 0) {
      // If layerTrackedZone isnt initialized enable Geoman draw mode
      map.pm.enableDraw("Polygon");
      // .once will ensure that theres only 1 create event listener per function call 
      map.once("pm:create", (e) => {
        // Store newly created Geoman layer
        const layer = e.layer as L.Polygon;

        // Get latLngs of create polygon
        const latlngs = layer.getLatLngs()[0] as L.LatLng[];
    
        // Convert leaflet latlng to our GeoCoordinateStruct[]
        const geoCoordinateStructs: GeoCoordinateStruct[] = latlngs.map(latlng => ({
          lat: latlng.lat,
          long: latlng.lng
        }));

        // Update the zone in the mission store with new geoCoordinates
        missionStore.updateZone(missionId, type, zoneIndex, geoCoordinateStructs);
        // Delete newly created layer since we want to create polygons from layerTracking 
        layer.remove();
      });
      
    } else {
      // Zone has Polygon, so we can edit it
      const editedLayer = (layerTrackedZone as ZoneLayer).layer

      // TODO: possibly add another ui element to explicitly mark when editing is done
      // toggle the edit mode to allow show completing the zone
      if (editedLayer.pm.enabled()) {
        editedLayer.pm.disable();
        return
      }
      
      // enable edit mode on the selected polygon
      editedLayer.pm.enable();

      // listen to when the layer edit is complete
      editedLayer.once("pm:update", (e) => {
        const layer = e.layer as L.Polygon
        const latlngs = layer.getLatLngs()[0] as L.LatLng[];
        
        const geoCoordinateStructs: GeoCoordinateStruct[] = latlngs.map(latlng => ({
          lat: latlng.lat,
          long: latlng.lng
        }));

        missionStore.updateZone(missionId, type, zoneIndex, geoCoordinateStructs);
      });


    }
  },

  updateLayerTracking: (newState) => {
    const newLayerTracking: LayerTracking = { missions: {} };
    get().rerenderLayers();

    newState.missions.forEach((mission) => {
      newLayerTracking.missions[mission.mission_id] = {
        vehicles: {
          MEA: { stages: {} },
          ERU: { stages: {} },
          MRA: { stages: {} }
        },
        zones: {
          KeepIn: [],
          KeepOut: []
        }
      };


      Object.keys(mission.zones).forEach((zoneType) => {
        const keepInZones = newLayerTracking.missions[mission.mission_id].zones.KeepIn;
        
        mission.zones.keep_in_zones.forEach(zone => {
          if (zone.length < 1) {
            keepInZones.push({});
            return;
          }

          const latlngs = zone.map(({lat, long}) => L.latLng([lat, long]));
          const polygon = L.polygon(latlngs).addTo(get().layers);
          const zoneLayer: ZoneLayer = {
            layer: polygon,
            properties: {
              color: "#000",
              visibility: true
            }
          };
          polygon.setStyle({
            color: "#000",
            fillOpacity: 0.2,
            opacity: 1
          })

          keepInZones.push(zoneLayer);
        });

        set({ layerTracking: newLayerTracking });
      });
    });
  },

  // Unimplemented methods
  addStageLayer: () => {},
  removeStageLayer: () => {},
  removeZoneLayer: () => {},
  getStageLayer: () => undefined,
  getZoneLayers: (missionId: number | null, type: ZoneType) => {
    if (missionId === null) return [];
    const mission = get().layerTracking.missions[missionId];
    if (!mission) return [];
    return (mission.zones[type] as ZoneLayer[])
  }
}));

// =============================================
// Store Export and Watchers
// =============================================
const store: MapStore = reactive(mapStore.getState()) as MapStore;

mapStore.subscribe((state) => {
  Object.assign(store, state);
});

watch(
  () => missionStore.state,
  (newState) => {
    store.updateLayerTracking(newState as MissionsStruct);
  },
  { deep: true }
);

export default store;
