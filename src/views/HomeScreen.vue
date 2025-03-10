<script setup lang="ts">
import Battery from "../components/VehicleStatus/VehicleBattery.vue";
import Connection from "../components/VehicleStatus/VehicleConnection.vue";
import Camera from "../components/CameraFeed.vue";
import CameraCarousel from "../components/CameraCarousel.vue";
import HomeSidebar from "@/components/HomeSidebar.vue";

const vehicleDataExample: {
  vehicleName: 'ERU' | 'MEA' | 'MRA';
  cameraID: number;
  batteryPct: number;
  connection: number;
  coordinates: {
    latitude: number;
    longitude: number;
  };
}[] = [
  {
    vehicleName: 'ERU',
    cameraID: 12345,
    batteryPct: 90,
    connection: 0,
    coordinates: { latitude: 20, longitude: 40 },
  },
  {
    vehicleName: 'MEA',
    cameraID: 67890,
    batteryPct: 50,
    connection: 50,
    coordinates: { latitude: 60, longitude: 80 },
  },
  {
    vehicleName: 'MRA',
    cameraID: 98765,
    batteryPct: 20,
    connection: 300,
    coordinates: { latitude: 32, longitude: 48 },
  },
];
</script>

<template>
    <div class="flex w-full">
      <HomeSidebar :vehicles="vehicleDataExample" />
      <div class="grid grid-cols-2 grid-rows-2 w-full gap-1 p-1">
        <div
          v-for="(vehicle, index) in vehicleDataExample"
          :key="index"
          class="relative flex cursor-pointer"
        >
          <Camera :cameraID="vehicle.cameraID" />
          <div class="absolute left-4 top-4 flex items-center gap-2">
            <Battery :percentage="vehicle.batteryPct" :charging="false" />
            <Connection :signal_string="vehicle.connection" :displayLatency="false" />
          </div>
        </div>
      </div>
    </div>
</template>
