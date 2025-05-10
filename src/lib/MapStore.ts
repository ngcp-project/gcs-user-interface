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
const TILE_URL = "http://localhost:8080/tile/{z}/{x}/{y}.png";

// =============================================
// Store Interface
// =============================================
interface MapStore {
  map: LeafletMapGeoman | null;
  mapOrigin: LatLng;
  layers: L.FeatureGroup<L.Polygon>;
  localTileURL: string;
  layerTracking: LayerTracking;
  
  // Map Management
  updateMapRef: (ref: LeafletMapGeoman | null) => void;
  toggleDrawMode: () => void;
  rerenderLayers: () => void;
  logMapStore: () => void;
  
  // Layer Management
  updateStagePolygon: (missionId: number, vehicle: VehicleEnum, stageId: number) => void;
  updateZonePolygon: (missionId: number, type: ZoneType, zoneIndex: number) => void;
  removeStageLayer: (missionId: number, vehicle: VehicleEnum, stageId: number) => void;
  getStageLayer: (missionId: number, vehicle: VehicleEnum, stageId: number) => StageLayer | undefined;
  getZoneLayers: (missionId: number, type: ZoneType) => ZoneLayer[];
  setLayerVisibility: (missionId: number, type: ZoneType) => void;
  // TODO: fix it so that theres no as declaration when reading from missionStore
  updateLayerTracking: (state: MissionsStruct) => void;
}

// =============================================
// Store Implementation
// =============================================
const mapStore = createStore<MapStore>((set, get) => ({
  map: null,
  mapOrigin: DEFAULT_MAP_ORIGIN,
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

  setLayerVisibility: (missionId: number, type: ZoneType) => {
    // Get all layers in layerTracking
    const layers = get().layerTracking.missions[missionId];
    const zoneLayers = layers.zones[type];
  
    zoneLayers.forEach((zone) => {
      // check if is a valid zone layer and not empty
      if (!("layer" in zone)) return;
      
      const zoneLayer = zone as ZoneLayer;
      const currentVisibility = zoneLayer.properties.visibility;

      // Toggle visibility of zoneLayer
      zoneLayer.properties.visibility = !currentVisibility;

      // Update polygon style based on visibility
      zoneLayer.layer.setStyle({
        opacity: zoneLayer.properties.visibility ? 1 : 0,
        fillOpacity: zoneLayer.properties.visibility ? 0.2 : 0
      });
    });

    // Update state (optional since we're mutating directly, but good practice)
    set({ layerTracking: get().layerTracking });
  },

  rerenderLayers: () => {
    // Clears all layers and rerenders them
    const map = get().map?.leafletObject;
    if (!map) return;

    get().layers.clearLayers();
    // TODO: Implement layer rerendering logic
  },

  updateLayerTracking: (newState) => {
    const newLayerTracking: LayerTracking = { missions: {} };

    get().layers.clearLayers();

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

      // Update Zone In and Zone Out
      Object.keys(mission.zones).forEach((zoneType) => {
        // zoneType from mission.zones will be keep_in_zones or keep_out_zones
        const zoneMapping: Record<keyof typeof mission.zones, ZoneType> = {
          keep_in_zones: "KeepIn",
          keep_out_zones: "KeepOut"
        }
        const layerTrackedZones = newLayerTracking.missions[mission.mission_id].zones[zoneMapping[zoneType as keyof typeof mission.zones]];
        const missionZone = mission.zones[zoneType as keyof typeof mission.zones];
        
        missionZone.forEach(zone => {
          // If no geofence in zone then push empty object
          if (zone.length < 1) {
            layerTrackedZones.push({});
            return;
          }

          // Map each geocoordinate to leaflet latlng and add it to map as polygon
          const latlngs = zone.map(({lat, long}) => L.latLng([lat, long]));
          const polygon = L.polygon(latlngs).addTo(get().layers);
          
          const zoneLayer: ZoneLayer = {
            layer: polygon,
            properties: {
              color: "#000",
              visibility: true
            } as LayerProperties
          };

          // Set color of zone polygon based on zoneType
          polygon.setStyle({
            color: zoneType == "keep_in_zones" ? "#1a0" : "#a20",
            fillOpacity: zoneLayer.properties.visibility ? 0.2 : 0,
            opacity: zoneLayer.properties.visibility ? 1 : 0
          })

          layerTrackedZones.push(zoneLayer);
        });

        set({ layerTracking: newLayerTracking });
      });
    });
  },

  updateStagePolygon: (missionId: number, vehicle: VehicleEnum, stageId: number) => {
    // const map = get().map?.leafletObject;
    // if (!map) return;

    // const layerTrackedStage = get().layerTracking.missions[missionId].vehicles[vehicle].stages[stageId];

    // // not pushing in zonelayer type, pushing in empty object or L.Polygon layer
    // if (!layerTrackedStage || Object.keys(layerTrackedStage).length === 0) {
    //   // If layerTrackedZone isnt initialized enable Geoman draw mode
    //   map.pm.enableDraw("Polygon");
    //   // .once will ensure that theres only 1 create event listener per function call 
    //   map.once("pm:create", (e) => {
    //     // Store newly created Geoman layer
    //     const layer = e.layer as L.Polygon;

    //     // Get latLngs of create polygon
    //     const latlngs = layer.getLatLngs()[0] as L.LatLng[];
    
    //     // Convert leaflet latlng to our GeoCoordinateStruct[]
    //     const geoCoordinateStructs: GeoCoordinateStruct[] = latlngs.map(latlng => ({
    //       lat: latlng.lat,
    //       long: latlng.lng
    //     }));

    //     // Update the zone in the mission store with new geoCoordinates
    //     missionStore.updateZone(missionId, type, zoneIndex, geoCoordinateStructs);
    //     // Delete newly created layer since we want to create polygons from layerTracking 
    //     layer.remove();
    //   });
      
    // } else {
    //   // Zone has Polygon, so we can edit it
    //   const editedLayer = (layerTrackedZone as ZoneLayer).layer

    //   // TODO: possibly add another ui element to explicitly mark when editing is done
    //   // toggle the edit mode to allow show completing the zone
    //   if (editedLayer.pm.enabled()) {
    //     editedLayer.pm.disable();
    //     return
    //   }
      
    //   // enable edit mode on the selected polygon
    //   editedLayer.pm.enable();

    //   // listen to when the layer edit is complete
    //   editedLayer.once("pm:update", (e) => {
    //     const layer = e.layer as L.Polygon
    //     const latlngs = layer.getLatLngs()[0] as L.LatLng[];
        
    //     const geoCoordinateStructs: GeoCoordinateStruct[] = latlngs.map(latlng => ({
    //       lat: latlng.lat,
    //       long: latlng.lng
    //     }));

    //     missionStore.updateZone(missionId, type, zoneIndex, geoCoordinateStructs);
    //   });

    // }
    
  },
  removeStageLayer: () => {},
  getStageLayer: (missionId: number | null, vehicle: VehicleEnum, stageId: number) => {
    if (missionId === null) return undefined;
    const mission = get().layerTracking.missions[missionId];
    if (!mission) return undefined;

    const vehicleLayers = mission.vehicles[vehicle];
    if (!vehicleLayers) return undefined;

    return vehicleLayers.stages[stageId];
  },
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
