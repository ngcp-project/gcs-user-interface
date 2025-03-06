<script setup lang="ts">
import Battery from "../components/VehicleStatus/VehicleBattery.vue";
import Connection from "../components/VehicleStatus/VehicleConnection.vue";
import Camera from "../components/CameraFeed.vue";
import CameraCarousel from "../components/CameraCarousel.vue";
import HomeSidebar from "@/components/HomeSidebar.vue";
import SidebarProvider from "@/components/ui/sidebar/SidebarProvider.vue";

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
  },
  {
    cameraID: 43210,
    batteryPct: 50,
    connection: 20
  }
];
</script>

<template>
    <div class="flex w-full h-full">
      <HomeSidebar />
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
