<script setup lang="ts">
import VehicleStatus from "../components/VehicleStatusComponent.vue";
import { onMounted, ref, Ref, onUnmounted } from "vue";
import Map from "../components/Map.vue";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";

// Type definitions
interface VehicleData {
  batteryPct: number;
  lastUpdated: number;
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
  lastUpdated: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});

const MEA_data = ref<VehicleData>({
  batteryPct: 0,
  lastUpdated: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});

const MRA_data = ref<VehicleData>({
  batteryPct: 0,
  lastUpdated: 0,
  coordinates: { longitude: 0, latitude: 0 },
  status: "Standby",
  yaw: 0,
  inKeepIn: false,
  inKeepOut: false
});

const FRA_data = ref<VehicleData>({
  batteryPct: 0,
  lastUpdated: 0,
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

// Store unlisten functions to clean up event listeners
const unlistenFunctions: UnlistenFn[] = [];

// Handle incoming telemetry data
function handleTelemetryUpdate(event: any) {
  console.log("Received telemetry update:", event);

  // Handle both direct payload and event wrapper formats
  const telemetryData = event.payload || event;
  const vehicleKey = telemetryData.vehicle_id?.toLowerCase();
  const vehicle = vehicleMap[vehicleKey];

  console.log("Processing update for vehicle:", vehicleKey, "with data:", telemetryData);

  if (vehicle) {
    // Update vehicle data with the new format
    vehicle.value = {
      ...vehicle.value,
      status: telemetryData.vehicleStatus || "Standby",
      batteryPct: telemetryData.batteryLife || 0,
      coordinates: {
        latitude: telemetryData.currentPosition?.latitude || 0,
        longitude: telemetryData.currentPosition?.longitude || 0
      },
      lastUpdated: telemetryData.lastUpdated || 0,
      yaw: telemetryData.yaw || 0,
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
    console.log("Received telemetry event:", event);
    handleTelemetryUpdate(event.payload);
  });

  unlistenFunctions.push(unlisten);

  // Initial telemetry fetch for each vehicle
  for (const vehicleKey of Object.keys(vehicleMap)) {
    try {
      const data = await invoke("get_telemetry", { vehicleId: vehicleKey });
      console.log("Initial telemetry data:", data);
      handleTelemetryUpdate(data);
    } catch (error) {
      console.error(`Failed to fetch initial telemetry for ${vehicleKey}:`, error);
    }
  }
}

// Update zone status functions
function updateIsInKeepIn(vehicleKey: string, isInZone: boolean) {
  vehicleMap[vehicleKey.toLowerCase()].value.inKeepIn = isInZone;
}

function updateIsInKeepOut(vehicleKey: string, isInZone: boolean) {
  vehicleMap[vehicleKey.toLowerCase()].value.inKeepOut = isInZone;
}

// Lifecycle hooks
onMounted(async () => {
  await initializeTelemetryListeners();
});

onUnmounted(() => {
  // Clean up all event listeners
  unlistenFunctions.forEach((unlisten) => unlisten());
});
</script>

<template>
  <div class="flex h-full w-[100dvw]">
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

    <div
      class="flex h-full w-fit max-w-[300px] flex-none flex-col gap-[6px] overflow-y-scroll bg-background p-[6px]"
    >
      <VehicleStatus
        v-for="(data, key) in vehicleMap"
        :key="key"
        :batteryPct="data.value.batteryPct"
        :latency="data.value.lastUpdated"
        :coordinates="data.value.coordinates"
        :vehicleName="String(key).toUpperCase()"
        :vehicleStatus="data.value.status"
        :isInKeepInZone="data.value.inKeepIn"
        :isInKeepOutZone="data.value.inKeepOut"
      />
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
