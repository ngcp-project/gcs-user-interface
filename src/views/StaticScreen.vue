<script setup lang="ts">
import VehicleStatus from "../components/VehicleStatusComponent.vue";
import { onMounted, ref, Ref } from "vue";
import Map from "../components/Map.vue";
import { Button } from "@/components/ui/button";

// initialize reactive variables for each vehicle's telemetry data (the object is reactive, so each key/value pair is also reactive)
const ERU_data = ref({
  batteryPct: 0,
  lastUpdated: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});
const MEA_data = ref({
  batteryPct: 0,
  lastUpdated: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});
const MRA_data = ref({
  batteryPct: 0,
  lastUpdated: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});
const FRA_data = ref({
  batteryPct: 0,
  lastUpdated: 0,
  coordinates: { longitude: 0, latitude: 0 },
  fire_coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});

// maps vehicle name to corresponding reactive variable so that addListeners can more easily set EventListeners to update variables
const vehicleMap: { [key: string]: Ref<any> } = {
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

// Add ref for the map component
const mapRef = ref();

// Add methods to control the map
const handleDrawPolygon = () => {
  mapRef.value?.toggleDrawMode();
};

const handleEditPolygon = () => {
  mapRef.value?.toggleEditMode();
};

const handleDragPolygon = () => {
  mapRef.value?.toggleDragMode();
};

const handleRemovePolygon = () => {
  mapRef.value?.toggleRemoveMode();
};

const handleZoneIn = async () => {
  try {
    await mapRef.value?.sendZoneInPolygonPoints();
  } catch (error) {
    console.error("Error handling Zone In:", error);
  }
};

const handleZoneOut = async () => {
  try {
    await mapRef.value?.sendZoneOutPolygonPoints();
  } catch (error) {
    console.error("Error handling Zone Out:", error);
  }
};

const handleClearPolygons = async () => {
  try {
    await mapRef.value?.clearPolygons();
  } catch (error) {
    console.error("Error handling Clear All:", error);
  }
};
</script>

<template>
  <div class="flex h-full w-[100dvw]">
    <div class="flex-grow">
      <!-- should be fire coords -->
      <Map
        ref="mapRef"
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

    <div class="flex h-full w-fit max-w-[300px] flex-none flex-col gap-[6px] overflow-y-scroll bg-background p-[6px]">
      <!-- Add Geoman Controls Section -->
      <div class="flex flex-col gap-2 p-2 bg-secondary/10 rounded-lg">
        <h3 class="text-lg font-semibold">Map Controls</h3>
        <div class="grid grid-cols-2 gap-2">
          <Button 
            variant="outline" 
            size="sm"
            @click="handleDrawPolygon"
          >
            Draw
          </Button>
          <Button 
            variant="outline" 
            size="sm"
            @click="handleEditPolygon"
          >
            Edit
          </Button>
          <Button 
            variant="outline" 
            size="sm"
            @click="handleDragPolygon"
          >
            Move
          </Button>
          <Button 
            variant="outline" 
            size="sm"
            @click="handleRemovePolygon"
          >
            Delete
          </Button>
        </div>
        <div class="grid grid-cols-2 gap-2 mt-2">
          <Button 
            variant="outline" 
            size="sm"
            class="bg-green-500/10 hover:bg-green-500/20"
            @click="handleZoneIn"
          >
            Zone In
          </Button>
          <Button 
            variant="outline" 
            size="sm"
            class="bg-red-500/10 hover:bg-red-500/20"
            @click="handleZoneOut"
          >
            Zone Out
          </Button>
        </div>
        <Button 
          variant="outline" 
          size="sm"
          @click="handleClearPolygons"
        >
          Clear All
        </Button>
      </div>

      <!-- Existing Vehicle Status Components -->
      <VehicleStatus
        :batteryPct="ERU_data.batteryPct"
        :latency="ERU_data.lastUpdated"
        :coordinates="ERU_data.coordinates"
        :vehicleName="'ERU'"
        :vehicleStatus="ERU_data.status"
        :isInKeepInZone="ERU_data.inKeepIn"
        :isInKeepOutZone="ERU_data.inKeepOut"
      />
      <VehicleStatus
        :batteryPct="MEA_data.batteryPct"
        :latency="MEA_data.lastUpdated"
        :coordinates="MEA_data.coordinates"
        :vehicleName="'MEA'"
        :vehicleStatus="MEA_data.status"
        :isInKeepInZone="MEA_data.inKeepIn"
        :isInKeepOutZone="MEA_data.inKeepOut"
      />
      <VehicleStatus
        :batteryPct="MRA_data.batteryPct"
        :latency="MRA_data.lastUpdated"
        :coordinates="MRA_data.coordinates"
        :vehicleName="'MRA'"
        :vehicleStatus="MRA_data.status"
        :isInKeepInZone="MRA_data.inKeepIn"
        :isInKeepOutZone="MRA_data.inKeepOut"
      />
      <VehicleStatus
        :batteryPct="FRA_data.batteryPct"
        :latency="FRA_data.lastUpdated"
        :coordinates="FRA_data.coordinates"
        :vehicleName="'FRA'"
        :vehicleStatus="FRA_data.status"
        :isInKeepInZone="FRA_data.inKeepIn"
        :isInKeepOutZone="FRA_data.inKeepOut"
      />
    </div>

    <!-- <div @keepIn="receiveKeepInEmit"></div> -->
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
