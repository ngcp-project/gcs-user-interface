<!-- <template>
  <div class="map">
    <l-map
      ref="map"
      v-model:zoom="zoom"
      :use-global-leaflet="false"
      :center="mapOrigin"
    >
      <l-tile-layer
        :url="tileServer"
        layer-type="base"
        name="OpenStreetMap"
      ></l-tile-layer>
    </l-map>
  </div>
</template>

<script lang="ts">
import "leaflet/dist/leaflet.css";
import { LMap, LTileLayer } from "@vue-leaflet/vue-leaflet";

export default {
  components: {
    LMap,
    LTileLayer,
  },
  data() {
    return {
      tileServer:
        import.meta.env.VITE_OSM_SERVER ||
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
      mapOrigin: [34.058, -117.819],
      zoom: 17,
    };
  },
};
</script>

<style scoped>
.map {
  height: 100%;
}
</style> -->

<!-- offline -->
<template>
  <div class="map">
    <l-map
      ref="map"
      v-model:zoom="zoom"
      :use-global-leaflet="false"
      :center="mapOrigin"
      @click="handleMapClick"
    >
      <l-tile-layer
        :url="localTileURL"
        :minZoom="14"
        :maxZoom="16"
        layer-type="base"
        name="CustomTiles"
      ></l-tile-layer>
    </l-map>
  </div>
</template>

<script lang="ts">
import "leaflet/dist/leaflet.css";
import { LMap, LTileLayer } from "@vue-leaflet/vue-leaflet";
import { LeafletMouseEvent } from "leaflet";


export default {
  components: {
    LMap,
    LTileLayer,
  },
  data() {
    return {
      mapOrigin: [34.058, -117.819],
      zoom: 16,
      localTileURL: "http://localhost:8001/{z}/{x}/{y}.png", // Update to your local server URL
    };
  },
  methods: {
    handleMapClick(event: LeafletMouseEvent) {
      const lat = event.latlng.lat;
      const lng = event.latlng.lng;
      console.log("Clicked coordinates:", lat, lng);
      // You can now use the lat and lng values as needed
    },
  },
};
</script>

<style scoped>
.map {
  height: 100%;
}
</style>