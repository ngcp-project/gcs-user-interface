import { createStore } from "zustand/vanilla";
import { reactive } from "vue";
import { LMap } from "@vue-leaflet/vue-leaflet";
import { LatLngTuple as LatLng, LatLngExpression } from "leaflet";
import * as L from "leaflet";
import {
  GeoCoordinateStruct,
  MissionsStruct,
  VehicleEnum,
  ZoneType,
  StageStruct
} from "@/lib//bindings";
import { missionStore } from "./MissionStore";
import { watch } from "vue";
import {
  LayerProperties,
  ZoneLayer,
  StageLayer,
  VehicleLayers,
  MissionLayers,
  LayerTracking,
  LeafletMap,
  LeafletMapGeoman,
  MapStore
} from "@/lib/MapStore.types";

// =============================================
// Constants
// =============================================
const LAYER_STYLING = {
  KeepIn: { color: "#0a0" },
  KeepOut: { color: "#a00" },
  ERU: { color: "#0000FF" },
  MEA: { color: "#FF00FF" },
  MRA: { color: "#00FFFF" }
} as const;

const DEFAULT_MAP_ORIGIN: LatLng = [33.932573934575075, -117.63059569114814];
const TILE_URL = "http://localhost:8080/tile/{z}/{x}/{y}.png";

// =============================================
// Store Implementation
// =============================================
const mapStore = createStore<MapStore>((set, get) => ({
  map: null,
  mapOrigin: DEFAULT_MAP_ORIGIN,
  markerCoord: L.latLng(DEFAULT_MAP_ORIGIN[0], DEFAULT_MAP_ORIGIN[1]),
  localTileURL: TILE_URL,
  layerTracking: { missions: {} },
  layers: L.featureGroup([]),
  vehicleMarkers: {
    MRA: L.latLng(DEFAULT_MAP_ORIGIN[0], DEFAULT_MAP_ORIGIN[1]),
    MEA: L.latLng(DEFAULT_MAP_ORIGIN[0], DEFAULT_MAP_ORIGIN[1]),
    ERU: L.latLng(DEFAULT_MAP_ORIGIN[0], DEFAULT_MAP_ORIGIN[1])
  },

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
    const map = get().map?.leafletObject;

    if (!map) return;
    
    if (map.pm.globalDrawModeEnabled()) {
      map.pm.disableDraw();
    } else {
      map.pm.enableDraw("Polygon");
    }
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
        const geoCoordinateStructs: GeoCoordinateStruct[] = latlngs.map((latlng) => ({
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
      const editedLayer = (layerTrackedZone as ZoneLayer).layer;

      // TODO: possibly add another ui element to explicitly mark when editing is done
      // toggle the edit mode to allow show completing the zone
      if (editedLayer.pm.enabled()) {
        editedLayer.pm.disable();
        return;
      }

      // enable edit mode on the selected polygon
      editedLayer.pm.enable();

      // listen to when the layer edit is complete
      editedLayer.once("pm:update", (e) => {
        const layer = e.layer as L.Polygon;
        const latlngs = layer.getLatLngs()[0] as L.LatLng[];

        const geoCoordinateStructs: GeoCoordinateStruct[] = latlngs.map((latlng) => ({
          lat: latlng.lat,
          long: latlng.lng
        }));

        missionStore.updateZone(missionId, type, zoneIndex, geoCoordinateStructs);
      });
    }
  },

  setZoneLayerVisibility: (missionId: number, type: ZoneType, zoneIndex: number) => {
    // Get all layers in layerTracking
    const layers = get().layerTracking.missions[missionId];
    const zoneLayers = layers.zones[type];

    // Get the zone layer to update
    const zoneLayer = zoneLayers[zoneIndex];
    if (!("layer" in zoneLayer)) return;

    const currentVisibility = zoneLayer.properties.visibility;

    // Toggle visibility of zoneLayer
    zoneLayer.properties.visibility = !currentVisibility;

    // Update polygon style based on visibility
    zoneLayer.layer.setStyle({
      opacity: zoneLayer.properties.visibility ? 1 : 0,
      fillOpacity: zoneLayer.properties.visibility ? 0.2 : 0
    });

    // Update state (optional since we're mutating directly, but good practice)
    set({ layerTracking: get().layerTracking });
  },

  setStageLayerVisibility: (missionId: number, vehicle: VehicleEnum, stageId: number) => {
    // Get all layers in layerTracking
    const layers = get().layerTracking.missions[missionId];
    const vehicleLayers = layers.vehicles[vehicle];
    const stageLayer = vehicleLayers.stages[stageId];
    if (!stageLayer || !("polygon" in stageLayer)) return;

    const currentVisibility = stageLayer.properties.visibility;

    // Toggle visibility of stageLayer
    stageLayer.properties.visibility = !currentVisibility;

    // Update polygon style based on visibility
    stageLayer.polygon.layer.setStyle({
      opacity: stageLayer.properties.visibility ? 1 : 0,
      fillOpacity: stageLayer.properties.visibility ? 0.2 : 0
    });

    // Update state
    set({ layerTracking: get().layerTracking });
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
        };
        const layerTrackedZones =
          newLayerTracking.missions[mission.mission_id].zones[
            zoneMapping[zoneType as keyof typeof mission.zones]
          ];
        const missionZone = mission.zones[zoneType as keyof typeof mission.zones];

        missionZone.forEach((zone) => {
          // If no geofence in zone then push empty object
          if (zone.length < 1) {
            layerTrackedZones.push({});
            return;
          } else {
            const latLngs: LatLng[] = zone.map((GeoCoord) => [GeoCoord.lat, GeoCoord.long]);

            layerTrackedZones.push({
              layer: L.polygon(latLngs) as L.Polygon,
              properties: {
                color:
                  zoneType === "keep_in_zones"
                    ? LAYER_STYLING.KeepIn.color
                    : LAYER_STYLING.KeepOut.color,
                visibility: true
              }
            });
          }
        });
      });

      // Update Stage Layers
      Object.entries(mission.vehicles).forEach(([vehicleName, vehicleData]) => {
        const vehicle = vehicleName as VehicleEnum;
        const stages = vehicleData.stages as unknown as StageStruct[];

        stages.forEach((stage) => {
          if (!stage.search_area || stage.search_area.length < 1) {
            newLayerTracking.missions[mission.mission_id].vehicles[vehicle].stages[stage.stage_id] =
              {
                stageId: stage.stage_id,
                polygon: {} as ZoneLayer,
                properties: { color: LAYER_STYLING[vehicle].color, visibility: true }
              };
            return;
          }

          const latLngs: LatLng[] = stage.search_area.map((coord: GeoCoordinateStruct) => [
            coord.lat,
            coord.long
          ]);
          const polygon = L.polygon(latLngs) as L.Polygon;

          newLayerTracking.missions[mission.mission_id].vehicles[vehicle].stages[stage.stage_id] = {
            stageId: stage.stage_id,
            polygon: {
              layer: polygon,
              properties: {
                color: LAYER_STYLING[vehicle].color,
                visibility: true
              }
            },
            properties: {
              color: LAYER_STYLING[vehicle].color,
              visibility: true
            }
          };
        });
      });

      set({ layerTracking: newLayerTracking });
      get().rerenderLayers();
    });

    set({ layerTracking: newLayerTracking });
    get().rerenderLayers();
  },

  rerenderLayers: () => {
    const map = get().map?.leafletObject;
    if (!map) return;

    // Clear the existing layers first
    get().layers.clearLayers();

    // console.log("layerTracking", get().layerTracking);

    // Iterate over layerTracking and add polygons to the map
    Object.entries(get().layerTracking.missions).forEach(([missionId, missionData]) => {
      // Iterate over the zones (KeepIn, KeepOut)
      (["KeepIn", "KeepOut"] as ZoneType[]).forEach((type) => {
        missionData.zones[type].forEach((zone) => {
          if (!("layer" in zone) || !(Object.keys(zone.layer).length > 0)) return;

          const zoneLayer = zone as ZoneLayer;
          const polygonLayer = zoneLayer.layer;

          // Add the polygon layer to the map
          polygonLayer.addTo(get().layers);

          // Set the style based on the properties from layerTracking
          polygonLayer.setStyle({
            color: zoneLayer.properties.color,
            fillOpacity: zoneLayer.properties.visibility ? 0.2 : 0,
            opacity: zoneLayer.properties.visibility ? 1 : 0
          });
        });
      });

      // Add stage polygons
      Object.entries(missionData.vehicles).forEach(([vehicleName, vehicleData]) => {
        const vehicle = vehicleName as VehicleEnum;
        Object.values(vehicleData.stages).forEach((stage) => {
          if (!stage || !("polygon" in stage) || !stage.polygon.layer) return;

          const polygonLayer = stage.polygon.layer;
          polygonLayer.addTo(get().layers);

          // Set the style based on the properties from layerTracking
          polygonLayer.setStyle({
            color: stage.properties.color,
            fillOpacity: stage.properties.visibility ? 0.2 : 0,
            opacity: stage.properties.visibility ? 1 : 0
          });
        });
      });
    });
  },

  updateStagePolygon: (missionId: number, vehicle: VehicleEnum, stageId: number) => {
    const map = get().map?.leafletObject;
    if (!map) return;

    // If no stageLayer then make it also an empty object
    const stageLayers = get().layerTracking.missions[missionId]?.vehicles[vehicle]?.stages || {};
    const layerTrackedStage = stageLayers[stageId];

    if (
      !layerTrackedStage ||
      !("polygon" in layerTrackedStage) ||
      Object.keys(layerTrackedStage.polygon).length === 0
    ) {
      map.pm.enableDraw("Polygon");

      map.once("pm:create", (e) => {
        const layer = e.layer as L.Polygon;
        const latlngs = layer.getLatLngs()[0] as L.LatLng[];

        // Set the polygon color based on vehicle type
        layer.setStyle({
          color: LAYER_STYLING[vehicle].color,
          fillOpacity: 0.2
        });

        const geoCoordinateStructs: GeoCoordinateStruct[] = latlngs.map((latlng) => ({
          lat: latlng.lat,
          long: latlng.lng
        }));

        missionStore.updateStageArea(missionId, vehicle, stageId, geoCoordinateStructs);

        // Remove drawn layer (will be re-rendered through updateLayerTracking)
        layer.remove();
      });
    } else {
      const existingLayer = layerTrackedStage.polygon;
      const polygonLayer = existingLayer.layer;

      // Toggle edit mode
      if (polygonLayer.pm.enabled()) {
        polygonLayer.pm.disable();
        return;
      }

      polygonLayer.pm.enable();

      polygonLayer.once("pm:update", (e) => {
        const updatedLayer = e.layer as L.Polygon;
        const latlngs = updatedLayer.getLatLngs()[0] as L.LatLng[];

        // Maintain the polygon color after editing
        updatedLayer.setStyle({
          color: LAYER_STYLING[vehicle].color,
          fillOpacity: 0.2
        });

        const geoCoordinateStructs: GeoCoordinateStruct[] = latlngs.map((latlng) => ({
          lat: latlng.lat,
          long: latlng.lng
        }));

        missionStore.updateStageArea(missionId, vehicle, stageId, geoCoordinateStructs);
      });
    }
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
    return mission.zones[type] as ZoneLayer[];
  },
  updateVehicleMarker: (vehicle: VehicleEnum, lat: number, lng: number) => {
    set((state) => ({
      vehicleMarkers: {
        ...state.vehicleMarkers,
        [vehicle]: L.latLng(lat, lng)
      }
    }));
  },
  updateMarkerCoords: (vehicle: VehicleEnum, coords: LatLngExpression) => {
    if (Array.isArray(coords) && coords.length === 2) {
      set((state) => ({
        vehicleMarkers: {
          ...state.vehicleMarkers,
          [vehicle]: L.latLng(coords[0], coords[1])
        }
      }));
    }
  },
  getVehicleMarkers: () => {
    return get().vehicleMarkers;
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
