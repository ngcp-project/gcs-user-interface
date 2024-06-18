<script setup lang="ts">
import { ref, Ref, onMounted } from "vue";
import Battery from "../components/VehicleStatus/VehicleBattery.vue";
import Connection from "../components/VehicleStatus/VehicleConnection.vue";
import Camera from "../components/Camera.vue";
import { useRouter } from "vue-router";
import { getAllConnections } from "../Functions/webSocket";

const router = useRouter();

const handleCameraClick = (cameraID: number, vehicleKey: string) => {
  console.log("camera:", cameraID);
  // router.push(`/CamFocus/${cameraID}`);}
  router.push(`/CamFocus/${cameraID}/${vehicleKey}`);
};

// initialize reactive variables for each vehicle's telemetry data (the object is reactive, so each key/value pair is also reactive)
const vehiclesData = ref([
  { key: "eru", cameraID: 2, batteryPct: 0, connection: 0 },
  { key: "mea", cameraID: 1, batteryPct: 0, connection: 0 },
  { key: "mra", cameraID: 1, batteryPct: 0, connection: 0 },
  { key: "fra", cameraID: 2, batteryPct: 0, connection: 0 }
]);

let wsConnections: { [key: string]: WebSocket } = {};
// this function runs once (in mounted) and adds event listeners for each vehicle WS connection, so that the reactive variables update whenever new data is received
function addListeners() {
  wsConnections = getAllConnections(); // gets all 4 Websocket connections that were initialized in App.vue (when Vue project first ran)

  for (const [vehicleKey, webSocketConnection] of Object.entries(wsConnections)) {
    webSocketConnection.addEventListener("message", (event) => {
      const receivedData = JSON.parse(event.data);

      const vehicle = vehiclesData.value.find((v) => v.key === vehicleKey);
      if (vehicle) {
        vehicle.batteryPct = parseFloat(receivedData.batteryLife);
        vehicle.connection = parseInt(receivedData.dummyConnection);
      }
    });
  }
}

// gets all 4 websocket connections and adds event listeners to each of them once Static Screen finishes initial rendering
onMounted(() => {
  addListeners();
});
</script>

<template>
  <div class="grid h-full w-full gap-1 p-1">
    <div
      v-for="(vehicle, index) in vehiclesData"
      :key="index"
      class="relative flex cursor-pointer"
      @click="handleCameraClick(vehicle.cameraID, vehicle.key)"
    >
      <Camera :cameraID="vehicle.cameraID" />
      <div class="absolute left-4 top-4 flex items-center gap-2">
        <Battery :percentage="vehicle.batteryPct" :charging="false" />
        <Connection :latency="vehicle.connection" :displayLatency="false" />
      </div>
    </div>
  </div>
</template>
