<script setup lang="ts">
import Navbar from "./components/HeaderNavbar.vue";
import { provide, ref } from "vue";
import { RouterView } from "vue-router";
import { Coordinate, Vehicle, Stage } from "./types";
import { SearchCoordsProvider } from "./types/search-coords-provider";
import { TargetCoordsProvider } from "./types/target-coords.provider";
import { MissionInformation } from "./types/mission-info";
import SidebarProvider from "./components/ui/sidebar/SidebarProvider.vue";
import { useColorMode } from "@vueuse/core";
import { Toaster } from '@/components/ui/sonner'

// --------- SEARCH AREA COORDINATES (used to select a search area from Map.vue) ------ //
const searchCoords = ref([""]);
const selectingSearch = ref(false);
function updateSearchCoords(coordinate: string[]) {
  console.log("Updating search coordinates:", coordinate);
}

provide<SearchCoordsProvider>("search-coords-provider", {
  searchCoords,
  selectingSearch,
  updateSearchCoords
});

const targetCoord = ref("");
const selectingTarget = ref(false);
provide<TargetCoordsProvider>("target-coords-provider", {
  targetCoord,
  selectingTarget
});

const MISSION_INFO = ref<MissionInformation>({ missionName: "", stages: [] });

function save_MISSION_INFO() {
  console.log("Saving MISSION_INFO to localstorage");
}

// loads mission info object from localstorage and saves to MISSION_INFO
function load_MISSION_INFO() {
  console.log("Loading MISSION_INFO from localstorage");
}

// --- THIS ADDS A NEW STAGE TO MISSION_INFO'S "stages" LIST --- //
function addStage(stage: Stage) {
  console.log("Adding stage:", stage);
}

// --- THIS UPDATES SEARCH AREA COORDS FOR SPECIFIC VEHICLE IN SPECIFIED STAGE --- //
function updateSearchArea(
  stageName: string,
  vehicleName: string,
  newSearchAreaCoords: Coordinate[]
) {
  console.log(
    "Updating search area for stage:",
    stageName,
    "and vehicle:",
    vehicleName,
    "with coords:",
    newSearchAreaCoords
  );
}

// --- THIS UPDATES TARGET COORDS FOR SPECIFIC VEHICLE IN SPECIFIED STAGE --- //
function updateTarget(stageName: string, vehicleName: string, newTargetCoord: [string, string]) {
  console.log(
    "Updating target for stage:",
    stageName,
    "and vehicle:",
    vehicleName,
    "with coords:",
    newTargetCoord
  );
}

// --- THIS RETURNS ALL THE NAMES OF THE STAGES CURRENTLY IN MISSION_INFO --- //
function getStageNames() {
  console.log("Getting stage names");
  return [];
}

// --- THIS PRINTS OUT ALL VEHICLES AND THEIR RESPECTIVE TARGET COORD AND SEARCH AREA COORDS FOR A SPECIFIED STAGE --- //
function getStageInfo(stageName: string) {
  console.log("Getting stage info for stage:", stageName);
}

// --- THIS CHECKS IF A STAGE WITH THE SPECIFIED NAME ALREADY EXISTS IN THE CURRENT MISSION --- //
function checkStageExists(stageName: string) {
  console.log("Checking if stage exists:", stageName);
  return false;
}

// OLD PROVIDE FUNCTIONS
provide("mission-info-provider", {
  MISSION_INFO,
  addStage,
  updateSearchArea,
  updateTarget,
  getStageNames,
  getStageInfo,
  checkStageExists,
  save_MISSION_INFO,
  load_MISSION_INFO
});

// DEFAULT COLOR SCHEME TO LIGHT MODE
useColorMode().value = "light";

export type { Coordinate, Vehicle, Stage };
</script>

<template>
  <Toaster />
  <div class="flex h-[100dvh] flex-col">
    <SidebarProvider class="min-h-0 flex-grow overflow-y-hidden">
      <RouterView />
    </SidebarProvider>
  </div>
</template>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr); /* 2 columns */
  grid-template-rows: repeat(2, 46vh); /* 2 rows */
  max-width: 100vw; /* Limit width to viewport width */
  max-height: 100vh; /* Limit height to viewport height */
}
</style>
