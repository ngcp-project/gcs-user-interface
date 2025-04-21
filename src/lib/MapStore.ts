import { createStore } from "zustand/vanilla";
import { reactive } from "vue";
import { LMap } from "@vue-leaflet/vue-leaflet";
import { LatLngTuple as LatLng } from "leaflet";

// Merge Vue Leaflet Map with Geoman PM Map types
type LeafletMap = InstanceType<typeof LMap>;
export interface LeafletMapGeoman extends LeafletMap {
  leafletObject: LeafletMap["leafletObject"] & L.Map;
}

interface MapStore {
  map: LeafletMapGeoman | null;
  mapOrigin: LatLng;
  zoom: number;
  localTileURL: string;
  controlPosition: L.ControlPosition;
  setControlPosition: (position: L.ControlPosition) => void;
  updateMapRef: (ref: LeafletMapGeoman | null) => void;
  toggleDrawMode: () => void;
}
const mapStore = createStore<MapStore>((set, get) => ({
  map: null,
  mapOrigin: [35.33004319829399, -120.75064544958856],
  zoom: 16,
  localTileURL: "http://localhost:8080/tile/{z}/{x}/{y}.png",
  controlPosition: "topright",
  setControlPosition: (position: L.ControlPosition) => set({ controlPosition: position }),
  updateMapRef: (refValue: LeafletMapGeoman | null) => {
    set({ map: refValue });
  },
  toggleDrawMode: () => {
    get().map?.leafletObject?.pm.enableDraw("Polygon");
  }
}));

const store: MapStore = reactive(mapStore.getState());

mapStore.subscribe((state) => {
  Object.assign(store, state);
});

export default store;
