<!-- offline -->
<template>
  <div class="h-full w-full">
    <l-map
      ref="mapRef"
      :zoom=16
      :use-global-leaflet="true"
      :center="[mapStore.mapState.mapOrigin[0] as LatLng[0], mapStore.mapState.mapOrigin[1] as LatLng[1]]"
      @ready="handleReady"
    >
      <GeomanController />
      <l-tile-layer
        :url="mapStore.mapState.localTileURL"
        :minZoom="14"
        :maxZoom="16"
        layer-type="base"
        name="CustomTiles"
      />
      <l-marker
        v-for="(coords, vehicle) in mapStore.mapState.vehicleMarkers"
        :key="vehicle"
        :lat-lng="coords"
        :icon="getVehicleIcon(vehicle)"
      />
    </l-map>
  </div>
</template>

<script setup lang="ts">
import "leaflet/dist/leaflet.css";

import { ref, watch, onMounted, reactive } from "vue";
import { LMap, LTileLayer, LPolygon, LMarker, LControl } from "@vue-leaflet/vue-leaflet";
import { LeafletMouseEvent, LatLngTuple as LatLng, icon } from "leaflet";
import { mapStore } from "@/lib/StoresSync";
import { LeafletMapGeoman } from "@/lib/MapStore.types";
import GeomanController from "@/components/map/GeomanController.vue";
import { VehicleEnum } from "@/lib/bindings";

// Import vehicle icons
import MEAIcon from "@/assets/MEA.png";
import ERUIcon from "@/assets/ERU.png";
import MRAIcon from "@/assets/MRA.png";

// Updates the store with templateRef when map component renders
const mapRef = ref<LeafletMapGeoman | null>(null);
const handleReady = () => {
  mapStore.updateMapRef(mapRef.value);
};

// Function to get vehicle-specific icon
const getVehicleIcon = (vehicle: VehicleEnum) => {
  const iconMap = {
    MEA: MEAIcon,
    ERU: ERUIcon,
    MRA: MRAIcon
  };
  
  return icon({
    iconUrl: iconMap[vehicle],
    iconSize: [32, 32],
    iconAnchor: [16, 16]
  });
};

</script>

<style>
.leaflet-pm-toolbar {
  z-index: 9999 !important;
}

.leaflet-pm-draw {
  display: block !important;
}
</style>
