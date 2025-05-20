<script setup lang="ts">
import VehicleStatus from "../components/VehicleStatusComponent.vue";
import { onMounted, ref, Ref, onUnmounted } from "vue";
import Map from "../components/Map.vue";
import MapSidebar from "../components/MapSidebar.vue";
import { core } from "@tauri-apps/api";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

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

const unlistenFunctions: UnlistenFn[] = [];
// initialize reactive variables for each vehicle's telemetry data (the object is reactive, so each key/value pair is also reactive)
const ERU_data = ref({
  batteryPct: 0,
  signal_string: 0,
  lastUpdated: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});
const MEA_data = ref({
  batteryPct: 0,
  signal_string: 0,
  lastUpdated: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});
const MRA_data = ref({
  batteryPct: 0,
  signal_string: 0,
  lastUpdated: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});
const FRA_data = ref({
  batteryPct: 0,
  signal_string: 0,
  lastUpdated: 0,
  coordinates: { longitude: 0, latitude: 0 },
  fire_coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});

// maps vehicle name to corresponding reactive variable so that addListeners can more easily set EventListeners to update variables
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

function handleTelemetryUpdate(event: any) {
  console.log("Received telemetry update:", event);

  // Handle both direct payload and event wrapper formats
  const payload = event.payload || event;
  const vehicleKey = payload.vehicle_id?.toLowerCase();
  const telemetryData = payload.telemetry;
  const vehicle = vehicleMap[vehicleKey];
  if (!telemetryData) {
    console.error("No telemetry data found in the payload:", payload);
    return;
  }
  console.log("Processing update for vehicle:", vehicleKey, "with data:", telemetryData);
  if (vehicle) {
    console.log(telemetryData.signal_string);
    // Update vehicle data with the new format
    vehicle.value = {
      ...vehicle.value,
      status: telemetryData.vehicle_status || "Standby",
      batteryPct: telemetryData.battery_life || vehicle.value.batteryPct,
      signal_string: telemetryData.signal_string || vehicle.value.signal_string,
      coordinates: {
        latitude:
          telemetryData.currentPosition?.latitude ||
          telemetryData.current_position?.latitude ||
          telemetryData.coordinates?.latitude ||
          vehicle.value.coordinates.latitude,

        longitude:
          telemetryData.currentPosition?.longitude ||
          telemetryData.current_position?.longitude ||
          telemetryData.coordinates?.longitude ||
          vehicle.value.coordinates.longitude
      },
      yaw: telemetryData.yaw || vehicle.value.yaw,
      inKeepIn: vehicle.value.inKeepIn,
      inKeepOut: vehicle.value.inKeepOut
    };
    // Handle FRA fire coordinates
    if (vehicleKey === "fra" && telemetryData.fireCoordinate) {
      vehicle.value.fire_coordinates = {
        latitude: telemetryData.fireCoordinate.latitude || 0,

        longitude: telemetryData.fireCoordinate.longitude || 0
      };
    }
    console.log(`Updated ${vehicleKey} data:`, vehicle.value);
  } else {
    console.error("Vehicle not found:", vehicleKey);
  }
}

// Modify the event listener setup

async function initializeTelemetryListeners() {
  const unlisten = await listen("telemetry_update", (event: any) => {
    console.log("Received telemetry event:", JSON.stringify(event));

    handleTelemetryUpdate(event.payload);

    console.log(event.payload);
  });
  unlistenFunctions.push(unlisten);
  // Initial telemetry fetch for each vehicle
  for (const vehicleKey of Object.keys(vehicleMap)) {
    try {
      console.log(`Initializing telemetry for ${vehicleKey}`);
      console.log(`Initial telemetry data: for ${vehicleKey}`);
      // handleTelemetryUpdate(data);
    } catch (error) {
      console.error(`Failed to fetch initial telemetry for ${vehicleKey}:`, error);
    }
  }
}

function updateIsInKeepIn(vehicleKey: string, isInZone: boolean) {
  vehicleMap[vehicleKey.toLowerCase()].value.inKeepIn = isInZone;
}

function updateIsInKeepOut(vehicleKey: string, isInZone: boolean) {
  vehicleMap[vehicleKey.toLowerCase()].value.inKeepOut = isInZone;
}

onMounted(async () => {
  await initializeTelemetryListeners();
  console.log("Telemetry listeners initialized");
  // KeepOutZones("mra");
  // KeepOutZones("eru");
  // KeepOutZones("fra");
});

onUnmounted(() => {
  // Clean up all event listeners
  unlistenFunctions.forEach((unlisten) => unlisten());
});
</script>

<template>
  <div class="flex h-full w-[100dvw] flex-row">
    <div class="flex-grow">
      <!-- should be fire coords -->
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
      ></Map>
    </div>

    <div>
      <MapSidebar side="right" />
    </div>
  </div>
</template>

<style scoped>
.camera-container {
  height: 90vh; /* Set the height of the container to 100% of the viewport height */
  width: 75%;
  display: flex; /* Use flexbox to align items vertically */
  justify-content: center; /* Center the child element horizontally */
  align-items: center; /* Center the child element vertically */
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
