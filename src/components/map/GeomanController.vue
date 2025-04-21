<template>
  <l-control v-model:position="mapStore.controlPosition">
    <button @click="() => console.log(mapStore)">State Log</button>
    <button @click="mapStore.setControlPosition('bottomleft')">
      Set Control Position to Top Right
    </button>
  </l-control>
</template>
<script setup lang="ts">
import L from "leaflet";
import "@geoman-io/leaflet-geoman-free/dist/leaflet-geoman.css";
import "@geoman-io/leaflet-geoman-free";
import { LControl } from "@vue-leaflet/vue-leaflet";
import mapStore from "@/lib/MapStore";
import { onMounted } from "vue";

interface opts extends L.ControlOptions {
  position: L.ControlPosition;
  drawCircle?: boolean;
  oneBlock?: boolean;
}

const Geoman = L.Control.extend({
  options: {},
  initialize(options: opts) {
    L.setOptions(this, options);
  },
  addTo(map: L.Map) {
    if (!map.pm) return;
    map.pm.addControls({
      position: "topleft",
      drawCircle: false,
      drawCircleMarker: false,
      drawPolyline: false,
      drawRectangle: false,
      drawPolygon: true,
      editMode: true,
      dragMode: true,
      cutPolygon: false,
      removalMode: true
    });
  }
});

const geomanControls = new Geoman();

onMounted(() => {
  if (mapStore.map) {
    // geomanControls.addTo(mapStore.map.leafletObject);
  }
});
</script>
