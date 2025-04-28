import { createStore } from "zustand/vanilla";
import { reactive } from "vue";
import { LMap } from "@vue-leaflet/vue-leaflet";
import { LatLngTuple as LatLng } from "leaflet";
import * as L from "leaflet";
import { VehicleEnum, ZoneType } from "./bindings";

// Merge Vue Leaflet Map with Geoman PM Map types
type LeafletMap = InstanceType<typeof LMap>;
export interface LeafletMapGeoman extends LeafletMap {
  leafletObject: LeafletMap["leafletObject"] & L.Map;
}

interface layerProperties {
  missionId: number;
  vehicle: VehicleEnum;
  type: ZoneType | "Search";
  visibility: boolean;
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
  updateMapRef: (ref: LeafletMapGeoman | null) => void;
  toggleDrawMode: () => void;
  createNewGeoJSON: () => void;
  rerenderLayers: () => void;
  logMapStore: () => void;
  addZonePolygon: (type: ZoneType) => void;
}
const mapStore = createStore<MapStore>((set, get) => ({
  map: null,
  mapOrigin: [35.33004319829399, -120.75064544958856],
  zoom: 16,
  localTileURL: "http://localhost:8080/tile/{z}/{x}/{y}.png",
  layers: L.geoJSON(
    {
      type: "FeatureCollection",
      features: []
    } as GeoJSON.FeatureCollection, // Initialize with empty FeatureCollection
    {
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
  addZonePolygon: (type) => {
    const map = get().map?.leafletObject;
    if (!map) return;

    map.pm.enableDraw("Polygon");
    map.once("pm:create", (e) => {
      // As assertion here cause geoman types this as a generic layer
      // but we know its a polygon here
      const layer = e.layer as L.Polygon;
      const geoJSONFeature = layer.toGeoJSON(); // Convert to GeoJSON Feature
      console.log(layer);

      // Add custom properties
      geoJSONFeature.properties = {
        ...({
          missionId: 1,
          vehicle: "ERU",
          type: type,
          visibility: true
        } as layerProperties),
        ...geoJSONFeature.properties
      };
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

    // Clean all layers
    map.eachLayer((layer) => {
      layer.remove();
    });
    // Readd geoJSON layer
    map.addLayer(get().layers);
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
  }
}));

// TODO: Doing as is bad, need to fix type error on layers not having _map when first initialized
const store: MapStore = reactive(mapStore.getState()) as MapStore; // Create a reactive store

mapStore.subscribe((state) => {
  Object.assign(store, state);
});

export default store;
