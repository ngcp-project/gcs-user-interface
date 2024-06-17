<script setup lang="ts">
import Navbar from "./components/HeaderNavbar.vue";
import { provide, ref } from "vue";
import { RouterView } from "vue-router";
import { initializeWSConnections } from "./Functions/webSocket";
import { Coordinate, Vehicle, Stage } from "./types";
import { SearchCoordsProvider } from "./types/search-coords-provider";
import { TargetCoordsProvider } from "./types/target-coords.provider";
import { MissionInformation } from "./types/mission-info";

initializeWSConnections(); // initialize 4 websocket connections for all 4 vehicles at entry point of project
console.log("Initialize 4 websocket connections from App.vue");

// --------- SEARCH AREA COORDINATES (used to select a search area from Map.vue) ------ //
const searchCoords = ref([""]);
const selectingSearch = ref(false);
function updateSearchCoords(coordinate: string[]) {
  // searchCoords.value = coordinate.toString();
  searchCoords.value = coordinate;
}

provide<SearchCoordsProvider>("search-coords-provider", {
  searchCoords,
  selectingSearch,
  updateSearchCoords
});

// --------- TARGET COORDINATE (used to select a target coordinate from Map.vue) ----------- //
const targetCoord = ref("");
const selectingTarget = ref(false); // this indicates to MissionDropdown and Map6 that we are currently selecting a target coordinate
provide<TargetCoordsProvider>("target-coords-provider", {
  targetCoord,
  selectingTarget
});

// ---------------------------- MISSION INFORMATON ---------------------------- //
const MISSION_INFO = ref<MissionInformation>({ missionName: "", stages: [] });

// saves MISSION_INFO to localstorage
function save_MISSION_INFO() {
  localStorage.setItem("MISSION_INFO_STORED", JSON.stringify(MISSION_INFO.value));
  console.log("Saving MISSION_INFO to localstorage: ");
}

// loads mission info object from localstorage and saves to MISSION_INFO
function load_MISSION_INFO() {
  console.log("Loadng MISSION_INFO from localstorage: ");
  const data = localStorage.getItem("MISSION_INFO_STORED");
  if (data) {
    const parsed_data = JSON.parse(data || "");
    MISSION_INFO.value["missionName"] = parsed_data["missionName"];
    MISSION_INFO.value["stages"] = parsed_data["stages"];
    console.log(MISSION_INFO.value);
  }
}

// GET request to get Stages for the current mission (MISSION_NAME)
// async function refresh_MISSION() {
//   localStorage.removeItem("MISSION_INFO_STORED");
//   MISSION_INFO.value["missionName"] = "";
//   MISSION_INFO.value["stages"] = [] as Stage[];
//   const mission_name = localStorage.getItem('MISSION_NAME');

//   try {
//         const response = await fetch(`http://localhost:5135/MissionInfo?missionName=${mission_name}`, {
//           method: 'GET',
//         });
//         if (!response.ok) {
//           throw new Error('Network response was not ok');
//         }
//         // const res = await response.json();
//         console.log("fuck kngcp")
//         console.log(response);
//       }
//       catch (error) {
//         console.error('Unable to get Mission and its stages:', error);
//       }
// }

// --- THIS ADDS A NEW STAGE TO MISSION_INFO'S "stages" LIST --- //
function addStage(stage: Stage) {
  MISSION_INFO.value["stages"].push(stage);
  save_MISSION_INFO();
}

// --- THIS UPDATES SEARCH AREA COORDS FOR SPECIFIC VEHICLE IN SPECIFIED STAGE --- //
function updateSearchArea(
  stageName: string,
  vehicleName: string,
  newSearchAreaCoords: Coordinate[]
) {
  // loop through 'stages' array to find specified stage
  for (let i = 0; i < MISSION_INFO.value["stages"].length; i++) {
    // if we found the stage, loop through that stage's vehicleKeys
    let currentStage = MISSION_INFO.value["stages"][i];
    if (currentStage["stageName"] == stageName) {
      // loop through the stage's vehicle keys (ERU, FRA, etc)
      for (let j = 0; j < currentStage["vehicleKeys"].length; j++) {
        let vehicle = currentStage["vehicleKeys"][j];

        if (vehicle["vehicleName"] == vehicleName) {
          if (newSearchAreaCoords.length == 0) {
            vehicle["searchArea"] = null;
          } else {
            vehicle["searchArea"] = newSearchAreaCoords;
          }
        }
      }
    }
  }
  save_MISSION_INFO();
} // end updateSearchArea

// --- THIS UPDATES TARGET COORDS FOR SPECIFIC VEHICLE IN SPECIFIED STAGE --- //
function updateTarget(stageName: string, vehicleName: string, newTargetCoord: [string, string]) {
  let newTarget: Coordinate = {
    latitude: newTargetCoord[0],
    longitude: newTargetCoord[1]
  };
  // loop through 'stages' array to find specified stage
  for (let i = 0; i < MISSION_INFO.value["stages"].length; i++) {
    // if we found the stage, loop through that stage's vehicleKeys
    let currentStage = MISSION_INFO.value["stages"][i];
    if (currentStage["stageName"] == stageName) {
      // loop through the stage's vehicle keys (ERU, FRA, etc)
      for (let j = 0; j < currentStage["vehicleKeys"].length; j++) {
        let vehicle = currentStage["vehicleKeys"][j];

        if (vehicle["vehicleName"] == vehicleName) {
          if (newTargetCoord[0] == undefined) {
            vehicle["target"] = null;
          } else {
            vehicle["target"] = newTarget;
          }
        }
      }
    }
  }
  save_MISSION_INFO();
} // end updateTarget

// --- THIS RETURNS ALL THE NAMES OF THE STAGES CURRENTLY IN MISSION_INFO (so we can use the Stage Names in the stage dropdown menu in MissionDropdown) --- //
function getStageNames() {
  let stage_names = [] as string[];
  for (let i = 0; i < MISSION_INFO.value["stages"].length; i++) {
    let stage_name = MISSION_INFO.value["stages"][i]["stageName"];
    stage_names.push(stage_name);
  }
  return stage_names;
}

// --- THIS PRINTS OUT ALL VEHICLES AND THEIR RESPECTIVE TARGET COORD AND SEARCH AREA COORDS FOR A SPECIFIED STAGE --- //
function getStageInfo(stageName: string) {
  console.log("INFO FOR STAGE: " + stageName);
  for (let i = 0; i < MISSION_INFO.value["stages"].length; i++) {
    // if we found the stage, loop through that stage's vehicleKeys
    let currentStage = MISSION_INFO.value["stages"][i];
    if (currentStage["stageName"] == stageName) {
      // loop through the stage's vehicle keys (ERU, FRA, etc)
      for (let j = 0; j < currentStage["vehicleKeys"].length; j++) {
        let vehicle = currentStage["vehicleKeys"][j];
        console.log("VEHICLE: " + vehicle["vehicleName"]);
        console.log(vehicle["target"]);
        console.log(vehicle["searchArea"]);
      } //end for
    } // end if
  } // end for
} // end getStageInfo

// --- THIS CHECKS IF A STAGE WITH THE SPECIFIED NAME ALREADY EXISTS IN THE CURRENT MISSION --- //
function checkStageExists(stageName: string) {
  // loop through all stages of current mission
  for (let i = 0; i < MISSION_INFO.value["stages"].length; i++) {
    let currentStage = MISSION_INFO.value["stages"][i];
    if (currentStage["stageName"] == stageName) {
      return true;
    }
  }
  return false;
}

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

export type { Coordinate, Vehicle, Stage };
</script>

<template>
  <div class="flex h-[100dvh] flex-col">
    <div class="flex-shrink-0">
      <Navbar />
    </div>
    <div class="flex-grow overflow-y-auto">
      <RouterView />
    </div>
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
