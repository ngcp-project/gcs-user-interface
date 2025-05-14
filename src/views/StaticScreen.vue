<script setup lang="ts">
import VehicleStatus from "../components/VehicleStatusComponent.vue";
import { onMounted, ref, Ref, onUnmounted } from "vue";
import Map from "../components/Map.vue";
import MapSidebar from "../components/MapSidebar.vue";

// Type definitions
interface VehicleData {
  batteryPct: number;
  //Signal String
  signal_string: number;
  coordinates: { longitude: number; latitude: number };
  status: string;
  yaw: number;
  inKeepIn: boolean;
  inKeepOut: boolean;
  fire_coordinates?: { longitude: number; latitude: number };
}

interface TelemetryEvent {
  vehicleKey: string;
  data: {
    batteryLife: number;
    vehicleStatus: string;
    currentPosition: {
      latitude: number;
      longitude: number;
    };
    lastUpdated: number;
    yaw: number;
    fireCoordinates?: {
      latitude: number;
      longitude: number;
    };
  };
}

// Initialize reactive variables for each vehicle's telemetry data
const ERU_data = ref<VehicleData>({
  batteryPct: 0,
  signal_string: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});

const MEA_data = ref<VehicleData>({
  batteryPct: 0,
  signal_string: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});

const MRA_data = ref<VehicleData>({
  batteryPct: 0,
  signal_string: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});

const FRA_data = ref<VehicleData>({
  batteryPct: 0,
  signal_string: 0,
  coordinates: { longitude: 0, latitude: 0 },
  fire_coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});

// Map vehicle names to their corresponding reactive refs
const vehicleMap: { [key: string]: Ref<VehicleData> } = {
  eru: ERU_data,
  mea: MEA_data,
  mra: MRA_data,
  fra: FRA_data
};

// let wsConnections: { [key: string]: WebSocket } = {};
// // this function runs once (in mounted) and adds event listeners for each vehicle WS connection, so that the reactive variables update whenever new data is received
// function addListeners() {

//   for (const [vehicleKey, webSocketConnection] of Object.entries(wsConnections)) {
//     // loops through each WS connection and adds an event listener to it
//     webSocketConnection.addEventListener("message", (event) => {
//       const receivedData = JSON.parse(event.data);

//       vehicleMap[vehicleKey].value.status = getVehicleStatus(receivedData.vehicleStatus);
//       vehicleMap[vehicleKey].value.batteryPct = parseFloat(receivedData.batteryLife);
//       vehicleMap[vehicleKey].value.coordinates.latitude = parseFloat(
//         receivedData.currentPosition.latitude
//       );
//       vehicleMap[vehicleKey].value.coordinates.longitude = parseFloat(
//         receivedData.currentPosition.longitude
//       );
//       // vehicleMap[vehicleKey].value.lastUpdated = parseInt(receivedData.dummyConnection);   // <-- uncomment to use dummyConnection value from mockWebsock.cjs
//       vehicleMap[vehicleKey].value.lastUpdated = parseInt(receivedData.lastUpdated);
//       vehicleMap[vehicleKey].value.yaw = parseInt(receivedData.yaw);

//       // FRA is sending additional fire coordinates
//       if (vehicleKey == "fra") {
//         vehicleMap[vehicleKey].value.fire_coordinates.latitude = parseFloat(
//           receivedData.fireCoordinates.latitude
//         );
//         vehicleMap[vehicleKey].value.fire_coordinates.longitude = parseFloat(
//           receivedData.fireCoordinates.longitude
//         );
//       }
//     });
//   } // end for loop
// } // end addListeners

// gets all 4 websocket connections and adds event listeners to each of them once Static Screen finishes initial rendering

function updateIsInKeepIn(vehicleKey: string, isInZone: boolean) {
  vehicleMap[vehicleKey.toLowerCase()].value.inKeepIn = isInZone;
}

function updateIsInKeepOut(vehicleKey: string, isInZone: boolean) {
  vehicleMap[vehicleKey.toLowerCase()].value.inKeepOut = isInZone;
}

// function syncKeepOutZones(vehicle: string){
//     const polygons = zoneOut
// }

// Lifecycle hooks
onMounted(async () => {
  // KeepOutZones("mra"); // or "eru", "fra", etc.
  await initializeTelemetryListeners();
  KeepOutZones("mra");
  KeepOutZones("eru");
  KeepOutZones("fra");
});

onUnmounted(() => {
  // Clean up all event listeners
  unlistenFunctions.forEach((unlisten) => unlisten());
});
</script>

<template>
  <div class="flex flex-row h-full w-[100dvw]">
    <div class="flex-grow">
      <Map
        :ERU_coords="ERU_data.coordinates"
        :ERU_yaw="ERU_data.yaw"
        :MEA_coords="MEA_data.coordinates"
        :MEA_yaw="MEA_data.yaw"
        :MRA_coords="MRA_data.coordinates"
        :MRA_yaw="MRA_data.yaw"
        :FRA_coords="FRA_data.coordinates"
        :firePoint="FRA_data.fire_coordinates"
        :FRA_yaw="FRA_data.yaw"
        @keepIn="updateIsInKeepIn"
        @keepOut="updateIsInKeepOut"
      />
    </div>

    <div>
      <MapSidebar side="right"/>
    </div>
  </div>
</template>

<style scoped>
.camera-container {
  height: 90vh;
  width: 75%;
  display: flex;
  justify-content: center;
  align-items: center;
}

.screen_div {
  display: flex;
  border: 0.003em solid black;
  height: 89vh;
  width: 99vw;
}

.four-status-rightside {
  display: flex;
  flex-direction: column;
  border-left: 0.003em solid black;
  height: 100%;
  width: 23%;
  margin-left: auto;
}

.map_div {
  width: 77%;
}
</style>
