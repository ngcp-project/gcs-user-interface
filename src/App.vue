<script setup lang="ts">
import Navbar from "./components/Navbar.vue";
import { provide, ref } from "vue";
import { RouterView } from "vue-router";
import { initializeWSConnections } from "./Functions/webSocket";

initializeWSConnections(); // initialize 4 websocket connections for all 4 vehicles at entry point of project
console.log("Initialize 4 websocket connections from App.vue");

// Use provide/inject to pass data between components
const searchCoords = ref("");
function updateSearchCoords(coordinate: string) {
  searchCoords.value = coordinate.toString();
  // console.log("changed Coordinates", searchCoords.value);
}
provide("SearchCoords", {
  searchCoords,
  updateSearchCoords,
});

// FOR TARGET COORDINATE
const targetCoord = ref("");
const selectingTarget = ref(false);    // this indicates to MissionDropdown and Map6 that we are currently selecting a target coordinate 
provide("TargetCoord", {
  targetCoord,
  selectingTarget
});
</script>

<template>
  <div>
    <Navbar />
  </div>
  <RouterView />
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
.grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr); /* 2 columns */
  grid-template-rows: repeat(2, 46vh); /* 2 rows */
  max-width: 100vw; /* Limit width to viewport width */
  max-height: 100vh; /* Limit height to viewport height */
}
</style>
