<script setup lang="ts">
import Navbar from "./components/Navbar.vue";
import { provide, ref } from "vue";
import { RouterView } from "vue-router";
import { initializeWSConnections } from "./Functions/webSocket";
import { Coordinate, Vehicle, Stage } from "./Functions/types";

initializeWSConnections(); // initialize 4 websocket connections for all 4 vehicles at entry point of project
console.log("Initialize 4 websocket connections from App.vue");

// --------- SEARCH AREA COORDINATES (used to select a search area from Map.vue) ------ //
const searchCoords = ref([""]);
const selectingSearch = ref(false);
function updateSearchCoords(coordinate: string[]) {
  // searchCoords.value = coordinate.toString();
  searchCoords.value = coordinate;
}
provide("SearchCoords", {
  searchCoords,
  selectingSearch,
  updateSearchCoords,
});

// --------- TARGET COORDINATE (used to select a target coordinate from Map.vue) ----------- //
const targetCoord = ref("");
const selectingTarget = ref(false);    // this indicates to MissionDropdown and Map6 that we are currently selecting a target coordinate 
provide("TargetCoord", {
  targetCoord,
  selectingTarget
});

// ---------------------------- MISSION INFORMATON ---------------------------- //
const MISSION_INFO = ref({"missionName": "",
                          "stages": [] as Stage[]});

// --- THIS ADDS A NEW STAGE TO MISSION_INFO'S "stages" LIST --- //
function addStage(stage: Stage) {
  MISSION_INFO.value['stages'].push(stage);
}

// --- THIS UPDATES SEARCH AREA COORDS FOR SPECIFIC VEHICLE IN SPECIFIED STAGE --- //
function updateSearchArea(stageName: string, vehicleName: string, newSearchAreaCoords: Coordinate[]) {
  // loop through 'stages' array to find specified stage
  for (let i = 0; i < MISSION_INFO.value['stages'].length; i++) {
    
    // if we found the stage, loop through that stage's vehicleKeys
    let currentStage = MISSION_INFO.value['stages'][i];
    if (currentStage["stageName"] == stageName) {
      // loop through the stage's vehicle keys (ERU, FRA, etc)
      for (let j = 0; j < currentStage["vehicleKeys"].length; j ++) {
          let vehicle = currentStage["vehicleKeys"][j];

          if (vehicle["vehicleName"] == vehicleName) {
            vehicle['searchArea'] = newSearchAreaCoords;
          }
      } //end for
    } // end if
  } // end for
} // end updateSearchArea

// --- THIS UPDATES TARGET COORDS FOR SPECIFIC VEHICLE IN SPECIFIED STAGE --- //
function updateTarget(stageName: string, vehicleName: string, newTargetCoord: [string, string]) {
  const newTarget: Coordinate = {latitude: newTargetCoord[0],
                                 longitude: newTargetCoord[1]}
  // loop through 'stages' array to find specified stage
  for (let i = 0; i < MISSION_INFO.value['stages'].length; i++) {
    
    // if we found the stage, loop through that stage's vehicleKeys
    let currentStage = MISSION_INFO.value['stages'][i];
    if (currentStage["stageName"] == stageName) {
      // loop through the stage's vehicle keys (ERU, FRA, etc)
      for (let j = 0; j < currentStage["vehicleKeys"].length; j ++) {
          let vehicle = currentStage["vehicleKeys"][j];

          if (vehicle["vehicleName"] == vehicleName) {
            vehicle['target'] = newTarget;
          }
      } //end for
    } // end if
  } // end for
} // end updateTarget

// --- THIS RETURNS ALL THE NAMES OF THE STAGES CURRENTLY IN MISSION_INFO (so we can use the Stage Names in the stage dropdown menu in MissionDropdown) --- //
function getStageNames() {
  let stage_names = [] as string[];
  for (let i = 0; i < MISSION_INFO.value['stages'].length; i++) {
    let stage_name = MISSION_INFO.value['stages'][i]["stageName"];
    stage_names.push(stage_name);
  }
  return stage_names;
}

// --- THIS PRINTS OUT ALL VEHICLES AND THEIR RESPECTIVE TARGET COORD AND SEARCH AREA COORDS FOR A SPECIFIED STAGE --- //
function getStageInfo(stage_name: string) {
  console.log("INFO FOR STAGE: " + stage_name);
  for (let i = 0; i < MISSION_INFO.value['stages'].length; i++) {
    // if we found the stage, loop through that stage's vehicleKeys
    let currentStage = MISSION_INFO.value['stages'][i];
    if (currentStage["stageName"] == stage_name) {
      // loop through the stage's vehicle keys (ERU, FRA, etc)
      for (let j = 0; j < currentStage["vehicleKeys"].length; j ++) {
          let vehicle = currentStage["vehicleKeys"][j];
          console.log("VEHICLE: " + vehicle["vehicleName"]);
          console.log(vehicle["target"]);
          console.log(vehicle["searchArea"]);
      } //end for
    } // end if
  } // end for
} // end getStageInfo

// --- THIS CHECKS IF A STAGE WITH THE SPECIFIED NAME ALREADY EXISTS IN THE CURRENT MISSION --- //
function checkStageExists(stage_name: string) {
  // loop through all stages of current mission
  for (let i = 0; i < MISSION_INFO.value['stages'].length; i++) {
    let currentStage = MISSION_INFO.value['stages'][i];
    if (currentStage["stageName"] == stage_name) {
      return true;
    } 
  } 
  return false;
}


provide("Mission Info", {
  MISSION_INFO,
  addStage,
  updateSearchArea,
  updateTarget,
  getStageNames,
  getStageInfo,
  checkStageExists
});

export type {
  Coordinate, Vehicle, Stage
}


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
