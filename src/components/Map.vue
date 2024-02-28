<template>
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
</style>
