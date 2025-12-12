<template>
   <l-control :position="mapStore.mapState.controlPosition"
    > <Button @click="mapStore.logMapStore">State Log</Button> <Button
      @click="mapStore.toggleDrawMode"
      >Draw Mode</Button> 
      <Button
      @click="mapStore.rerenderLayers"
      >Rerender Map</Button
    > </l-control
  >
</template>

<script setup lang="ts">
import L from "leaflet";
import "@geoman-io/leaflet-geoman-free/dist/leaflet-geoman.css";
import "@geoman-io/leaflet-geoman-free";
import { Button } from "@/components/ui/button";
import { LControl } from "@vue-leaflet/vue-leaflet";
import { mapStore } from "@/lib/StoresSync";
// import { onMounted } from "vue";

interface opts extends L.ControlOptions {
  position: L.ControlPosition;
  drawCircle?: boolean;
  oneBlock?: boolean;
}

// TODO: Proof of concept for Geoman controls remove later
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

// const geomanControls = new Geoman();

// onMounted(() => {
//   if (mapStore.map) {
//     // geomanControls.addTo(mapStore.map.leafletObject);
//   }
// });
</script>

