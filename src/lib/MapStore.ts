import { createStore } from "zustand/vanilla";
import { reactive } from "vue";
import { LMap } from "@vue-leaflet/vue-leaflet";
import { LatLngTuple as LatLng } from "leaflet";
import * as L from "leaflet";
import { GeoCoordinateStruct, VehicleEnum, ZoneType } from "@/lib//bindings";
import { missionStore } from "./MissionStore";
import { watch } from "vue";

// New interfaces for layer tracking
interface StageLayer {
  stageId: number;
  polygon: ZoneLayer | {}; // TODO: change name to stageLayer
}

interface VehicleLayers {
  stages: Record<number, StageLayer>;
}
interface layerProperties {
  missionId: number;
  vehicle: VehicleEnum;
  type: ZoneType | "Search";
  visibility: boolean;
}
interface ZoneLayer {
  layer: L.Polygon;
  properties: layerProperties;
}
interface MissionLayers {
  vehicles: Record<VehicleEnum, VehicleLayers>;
  zones: {
    // Match to ZoneType enum
    KeepIn: (ZoneLayer | {} )[];
    KeepOut: (ZoneLayer | {} )[];
  };
}

interface LayerTracking {
  missions: Record<number, MissionLayers>;
}

// Merge Vue Leaflet Map with Geoman PM Map types
type LeafletMap = InstanceType<typeof LMap>;
export interface LeafletMapGeoman extends LeafletMap {
  leafletObject: LeafletMap["leafletObject"] & L.Map;
}


// TODO: Find actual colors that dont hurt operators eyes
const layerStyling = {
  KeepIn: {
    color: "#00FF00"
  },
  KeepOut: {
    color: "#FF0000"
  },
  ERU: {
    color: "#0000FF"
  },
  MEA: {
    color: "#FF00FF"
  },
  MRA: {
    color: "#00FFFF"
  }
};

interface MapStore {
  map: LeafletMapGeoman | null;
  mapOrigin: LatLng;
  zoom: number;
  layers: L.GeoJSON<L.Polygon>; // manages GeoJSON layers
  localTileURL: string;
  layerTracking: LayerTracking; // Add layer tracking
  updateMapRef: (ref: LeafletMapGeoman | null) => void;
  toggleDrawMode: () => void;
  createNewGeoJSON: () => void;
  rerenderLayers: () => void;
  logMapStore: () => void;
  // Add new methods for layer management
  addStageLayer: (missionId: number, vehicle: VehicleEnum, stageId: number, polygon: L.Polygon) => void;
  updateZonePolygon: (missionId: number, type: ZoneType, zoneIndex: number) => void;
  removeStageLayer: (missionId: number, vehicle: VehicleEnum, stageId: number) => void;
  removeZoneLayer: (missionId: number, type: ZoneType, index: number) => void;
  getStageLayer: (missionId: number, vehicle: VehicleEnum, stageId: number) => StageLayer | undefined;
  getZoneLayers: (missionId: number, type: ZoneType) => L.Polygon[];
}
const mapStore = createStore<MapStore>((set, get) => ({
  map: null,
  mapOrigin: [35.33004319829399, -120.75064544958856],
  zoom: 16,
  localTileURL: "http://localhost:8080/tile/{z}/{x}/{y}.png",
  layerTracking: {
    missions: {}
  },
  layers: L.geoJSON(
    {
      type: "FeatureCollection",
      features: []
    } as GeoJSON.FeatureCollection, // Initialize with empty FeatureCollection
    {
      onEachFeature: (feature, layer) => {
        // debug popup to show each polygons properties on click
        layer.bindPopup(feature.properties)
      },
      style: (feature) => {
        const featureType = feature?.properties?.type as keyof typeof layerStyling | undefined;
        // Default styling if no feature type is found
        if (!feature || !featureType)
          return {
            color: "#000000"
          };

        return layerStyling[featureType];
      }
    } as L.GeoJSONOptions // Type assertion to match GeoJSON options
  ),

  updateMapRef: (refValue: LeafletMapGeoman | null) => {
    // Should be called when map @ready event is triggered
    console.log("Map ref updated", refValue);
    const mapLeaflet = refValue?.leafletObject;
    if (!mapLeaflet) return;
    set({ map: refValue });
    // Assign preInitialized geoJSON to the map
    get().layers.addTo(mapLeaflet);
  },
  toggleDrawMode: () => {
    // Draws a polygon not linked to layers
    get().map?.leafletObject?.pm.enableDraw("Polygon");
  },
  updateZonePolygon: (missionId, type, zoneIndex) => {
    const map = get().map?.leafletObject;
    if (!map) return;

    // const mapZones = get().layerTracking.missions[missionId].zones[type]

    map.pm.enableDraw("Polygon");
    map.once("pm:create", (e) => {
      // As assertion here cause geoman types this as a generic layer
      // but we know its a polygon here
      const layer = e.layer as L.Polygon;
      const geoJSONFeature = layer.toGeoJSON(); // Convert to GeoJSON Feature
      console.log(layer);
      console.log(map.hasLayer(layer)); // should be true before remove()

      // Add custom properties
      // TODO: Modify later properties are in layerTracking
      geoJSONFeature.properties = {
        ...({
          missionId: 1,
          vehicle: "ERU",
          type: type,
          visibility: true
        } as layerProperties),
        ...geoJSONFeature.properties
      };
      // we know that L.LatLng is an array of [lat, long] since only 1 polygon
      // so we can unwrap the array to get the first and only polygon
      const latlngs = layer.getLatLngs()[0] as L.LatLng[] 
      // use layer latlng coords since GeoJson coords are different
      // map latlng to GeoCoordinateStruct
      const geoCoordinateStructs: GeoCoordinateStruct[] = latlngs.map(latlng => ({
        lat: latlng.lat,
        long: latlng.lng
      })) 
      console.log(geoCoordinateStructs)
      missionStore.updateZone(missionId, type, zoneIndex, geoCoordinateStructs)
      // get().addZoneLayer(1, type, layer); // TODO: Get current mission ID from mission store
      
      // copy polygon to layers
      get().layers?.addData(geoJSONFeature);
      // remove the original polygon that isnt assinged to layers
      // to clean duplicate polygons
      layer.remove();
    });
  },
  logMapStore: () => {
    const map = get().map?.leafletObject;
    if (!map) return;
    console.log(get());
    // polygons.eachLayer((layer) => {
    //   console.log(layer);
    // });
  },
  rerenderLayers: () => {
    const map = get().map?.leafletObject;
    if (!map) return;

    // Clear the geoJSON layer
    // get().layers.clearLayers();
    
    console.log("layerTracking", get().layerTracking.missions)

    // Repopulate geoJSON layer from layerTracking
    Object.values(get().layerTracking.missions).forEach((missionLayers) => {
      Object.values(missionLayers.zones).forEach((zoneArray) => {
        zoneArray.forEach((zone) => {
          if (zone && 'layer' in zone && zone.layer) {
            // Convert L.Polygon to GeoJSON feature
            const geoJSONFeature = zone.layer.toGeoJSON();
            // Attach properties if available
            if (zone.properties) {
              geoJSONFeature.properties = {
                ...zone.properties,
                ...geoJSONFeature.properties
              };
            }
            get().layers.addData(geoJSONFeature);
          }
        });
      });
    });
    map.addLayer(get().layers)
    console.log(get().layers)
  },

  createNewGeoJSON: () => {
    // TEST FUNCTION TO CREATE A NEW GEOJSON OBJECT
    const map = get().map?.leafletObject;
    if (!map) return;

    // converts polygon using latLng to GeoJson coordinates
    const polygon = L.polygon<LatLng>([
      [35.33004319829399, -120.75064544958856],
      [35.32982243794599, -120.75064544958856],
      [35.32982243794599, -120.75114544958856],
      [35.33004319829399, -120.75114544958856],
      [35.33004319829399, -120.75064544958856]
    ]).toGeoJSON().geometry.coordinates as GeoJSON.Position[][];

    var geojsonFeature: GeoJSON.Feature<GeoJSON.Polygon | GeoJSON.Point> = {
      type: "Feature",
      properties: {
        name: "Test Polygon"
      },
      geometry: {
        type: "Polygon",
        coordinates: polygon
      }
    };
    console.log(geojsonFeature.geometry.coordinates);
    get().layers.addData(geojsonFeature); // Add geoFeature to layers

    // L.geoJSON(geojsonFeature, {
    //   onEachFeature: (feature, layer) => {
    //     if (feature.properties && feature.properties.popupContent) {
    //       layer.bindPopup(feature.properties.name);
    //     }
    //   }
    // }).addTo(get().layers);
    console.log(geojsonFeature);
  },

  // Add new layer management methods
  addStageLayer: (missionId, vehicle, stageId, polygon) => {
    // const store = get();
    // if (!store.layerTracking.missions[missionId]) {
    //   store.layerTracking.missions[missionId] = {
    //     vehicles: { MEA: { stages: {} }, ERU: { stages: {} }, MRA: { stages: {} } },
    //     zones: { KeepIn: [], KeepOut: [] }
    //   };
    // }
    // if (!store.layerTracking.missions[missionId].vehicles[vehicle]) {
    //   store.layerTracking.missions[missionId].vehicles[vehicle] = {
    //     stages: {}
    //   };
    // }
    // store.layerTracking.missions[missionId].vehicles[vehicle].stages[stageId] = {
    //   stageId,
    //   polygon
    // };
    // set({ layerTracking: store.layerTracking });
  },

  removeStageLayer: (missionId, vehicle, stageId) => {
    // const store = get();
    // if (store.layerTracking.missions[missionId]?.vehicles[vehicle]?.stages[stageId]) {
    //   delete store.layerTracking.missions[missionId].vehicles[vehicle].stages[stageId];
    //   set({ layerTracking: store.layerTracking });
    // }
  },

  removeZoneLayer: (missionId, type, index) => {
    // const store = get();
    // const zoneType = type === "KeepIn" ? "keepIn" : "keepOut";
    // if (store.layerTracking.missions[missionId]?.zones[zoneType]) {
    //   store.layerTracking.missions[missionId].zones[zoneType].splice(index, 1);
    //   set({ layerTracking: store.layerTracking });
    // }
  },

  getStageLayer: (missionId, vehicle, stageId) => {
    return get().layerTracking.missions[missionId]?.vehicles[vehicle]?.stages[stageId];
  },

  getZoneLayers: (missionId, type) => {
    // const zoneType = type === "KeepIn" ? "keepIn" : "keepOut";
    // const layers = get().layerTracking.missions[missionId]?.zones[zoneType] || [];
    // // Filter out empty objects and return only actual polygons
    // return layers.filter((layer): layer is L.Polygon => 'addTo' in layer);
    return []
  }
}));

// TODO: Doing as is bad, need to fix type error on layers not having _map when first initialized
const store: MapStore = reactive(mapStore.getState()) as MapStore; // Create a reactive store

mapStore.subscribe((state) => {
  Object.assign(store, state);
});

// Listen for missionStore.state changes and update layerTracking
watch(
  () => missionStore.state,
  (newState) => {
    const newLayerTracking: LayerTracking = {
      missions: {}
    };

    // Update layerTracking based on new mission state
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

      // Update vehicle stages
      //implement vehicle stages
      
      // Update zones
      if (mission.zones.keep_in_zones) {
        const missionStoreKeepInZones = mission.zones.keep_in_zones
        const layerTrackedKeepInZones = newLayerTracking.missions[mission.mission_id].zones.KeepIn
        missionStoreKeepInZones.forEach(zone => {
          if (zone.length < 1) {
            layerTrackedKeepInZones.push({})
          }
          // add in logic to create L.Polygon from eacch 
        })
        store.rerenderLayers()
      }
      
      // Implement keepout
  })
},
  { deep: true }
);

export default store;
