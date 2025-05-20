<script setup lang="ts">
import Cameras from "@/components/Cameras.vue";
import HomeSidebar from "@/components/HomeSidebar.vue";
import { onMounted, ref, Ref, onUnmounted } from "vue";
import { core } from "@tauri-apps/api";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

// //vehicles: {
//     vehicleName: "ERU" | "MEA" | "MRA";
//     cameraID: number;
//     batteryPct: number;
//     connection: number;
//     coordinates: {
//       latitude: number;
//       longitude: number;
//     };
//   }[];

interface VehicleData {
  vehicleName: "ERU" | "MEA" | "MRA" | "FRA";
  cameraID: number;
  batteryPct: number;
  connection: number;
  coordinates: { longitude: number; latitude: number };
  // status: string;
  // yaw: number;
  // inKeepIn: boolean;
  // inKeepOut: boolean;
  // fire_coordinates?: { longitude: number; latitude: number };
}

const unlistenFunctions: UnlistenFn[] = [];

// Initialize reactive variables for each vehicle's telemetry data
const ERU_data = ref({
  vehicleName: "ERU" as const,
  cameraID: 12345,
  batteryPct: 0,
  connection: 0,
  coordinates: { longitude: 0, latitude: 0 },
  altitude: 0,
  airspeed: 0
  // status: "Standby",
  // yaw: 0,
  // inKeepIn: false,
  // inKeepOut: false
});

const MEA_data = ref({
  vehicleName: "MEA" as const,
  cameraID: 67890,
  batteryPct: 0,
  connection: 0,
  coordinates: { longitude: 0, latitude: 0 },
  altitude: 0,
  airspeed: 0
  // status: "Standby"
  // yaw: 0,
  // inKeepIn: false,
  // inKeepOut: false
});

const MRA_data = ref({
  vehicleName: "MRA" as const,
  cameraID: 98765,
  batteryPct: 0,
  connection: 0,
  coordinates: { longitude: 0, latitude: 0 },
  altitude: 0,
  airspeed: 0
  // status: "Standby"
  // yaw: 0,
  // inKeepIn: false,
  // inKeepOut: false
});

// const FRA_data = ref({
//   vehicleName: "FRA" as const,
//   cameraID: 54321,
//   batteryPct: 0,
//   connection: 0,
//   coordinates: { longitude: 0, latitude: 0 },
//   fire_coordinates: { longitude: 0, latitude: 0 },
//   // status: "Standby"
//   // yaw: 0,
//   // inKeepIn: false,
//   // inKeepOut: false
// });

// Create an array of the vehicle data for the sidebar
const vehicleData = ref([ERU_data.value, MEA_data.value, MRA_data.value]);

// Maps vehicle name to corresponding reactive variable
const vehicleMap: { [key: string]: Ref<any> } = {
  eru: ERU_data,
  mea: MEA_data,
  mra: MRA_data
};

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
      batteryPct: telemetryData.battery_life || vehicle.value.battery_life,
      connection: telemetryData.signal_string || vehicle.value.signal_string,
      coordinates: {
        latitude:
          telemetryData.currentPosition?.latitude.toFixed(2) ||
          telemetryData.current_position?.latitude.toFixed(2) ||
          telemetryData.coordinates?.latitude.toFixed(2) ||
          vehicle.value.coordinates.latitude.toFixed(2),

        longitude:
          telemetryData.currentPosition?.longitude.toFixed(2) ||
          telemetryData.current_position?.longitude.toFixed(2) ||
          telemetryData.coordinates?.longitude.toFixed(2) ||
          vehicle.value.coordinates.longitude.toFixed(2)
      },
      altitude: telemetryData.altitude.toFixed(2) || vehicle.value.altitude.toFixed(2),
      airspeed: telemetryData.speed.toFixed(2) || vehicle.value.speed.toFixed(2)
      // yaw: telemetryData.yaw || vehicle.value.yaw,
      // inKeepIn: vehicle.value.inKeepIn,
      // inKeepOut: vehicle.value.inKeepOut
    };

    // Handle FRA fire coordinates
    if (vehicleKey === "fra" && telemetryData.fireCoordinate) {
      vehicle.value.fire_coordinates = {
        latitude: telemetryData.fireCoordinate.latitude || 0,
        longitude: telemetryData.fireCoordinate.longitude || 0
      };
    }

    // Update the vehicleData array to reflect changes
    vehicleData.value = [ERU_data.value, MEA_data.value, MRA_data.value];

    console.log(`Updated ${vehicleKey} data:`, vehicle.value);
  } else {
    console.error("Vehicle not found:", vehicleKey);
  }
}

// async function initializeTelemetryListeners() {
//   const unlisten = await listen("telemetry_update", (event: any) => {
//     console.log("Received telemetry event:", JSON.stringify(event));
//     handleTelemetryUpdate(event.payload);
//     console.log(event.payload);
//   });

//   unlistenFunctions.push(unlisten);

//   // Initial telemetry fetch for each vehicle
//   for (const vehicleKey of Object.keys(vehicleMap)) {
//     try {
//       console.log(`Initializing telemetry for ${vehicleKey}`);
//       console.log(`Initial telemetry data: for ${vehicleKey}`);
//     } catch (error) {
//       console.error(`Failed to fetch initial telemetry for ${vehicleKey}:`, error);
//     }
//   }
// }

function updateIsInKeepIn(vehicleKey: string, isInZone: boolean) {
  vehicleMap[vehicleKey.toLowerCase()].value.inKeepIn = isInZone;
}

function updateIsInKeepOut(vehicleKey: string, isInZone: boolean) {
  vehicleMap[vehicleKey.toLowerCase()].value.inKeepOut = isInZone;
}

onMounted(async () => {
  // await initializeTelemetryListeners();
  // console.log("Telemetry listeners initialized");
});

onUnmounted(() => {
  // Clean up all event listeners
  unlistenFunctions.forEach((unlisten) => unlisten());
});
</script>

<template>
  <div class="flex w-full">
    <HomeSidebar :vehicles="vehicleData" />
    <Cameras />
  </div>
</template>
