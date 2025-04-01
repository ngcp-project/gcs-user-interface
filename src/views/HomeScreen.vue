<script setup lang="ts">
import Battery from "../components/VehicleStatus/VehicleBattery.vue";
import Connection from "../components/VehicleStatus/VehicleConnection.vue";
import Camera from "../components/CameraFeed.vue";
import Cameras from "../components/Cameras.vue";

const vehicleDataExample = [
  {
    cameraID: 12345,
    batteryPct: 90,
    connection: 0
  },
  {
    cameraID: 67890,
    batteryPct: 80,
    connection: 50
  },
  {
    cameraID: 98765,
    batteryPct: 70,
    connection: 300
  }
];
</script>

<template>
  <Cameras />
  <!-- <div class="grid h-full w-full gap-1 p-1">
    <div
      v-for="(vehicle, index) in vehicleDataExample"
      :key="index"
      class="relative flex cursor-pointer"
    >
      <
      <Camera :cameraID="vehicle.cameraID" />
      <div class="absolute left-4 top-4 flex items-center gap-2">
        <Battery :percentage="vehicle.batteryPct" :charging="false" />
        <Connection :latency="vehicle.connection" :displayLatency="false" />
      </div>
    </div>
  </div> -->
</template>
