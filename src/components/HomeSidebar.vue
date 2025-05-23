<script setup lang="ts">
import { Sidebar, SidebarContent } from "@/components/ui/sidebar";
import DeployCard from "@/components/Sidebar/SidebarCards/CameraScreen/DeployCard.vue";
import SidebarHeader from "@/components/ui/sidebar/SidebarHeader.vue";
import EmergencyStopDialog from "@/components/VehicleStatus/EmergencyStopDialog.vue";

defineProps<{
  vehicles: {
    vehicleName: "ERU" | "MEA" | "MRA";
    cameraID: number;
    batteryPct: number;
    connection: number;
    coordinates: {
      latitude: number;
      longitude: number;
    };
    altitude: number;
    airspeed: number;
  }[];
}>();
</script>

<template>
  <Sidebar>
    <SidebarHeader class="bg-sidebar-background">
      <EmergencyStopDialog :vehicle-name="'all'" />
    </SidebarHeader>

    <SidebarContent class="bg-sidebar-background">
      <div v-for="(vehicle, index) in vehicles" :key="index">
        <DeployCard
          :vehicle-name="vehicle.vehicleName"
          :battery="vehicle.batteryPct"
          :connection="vehicle.connection"
          :latitude="vehicle.coordinates.latitude"
          :longitude="vehicle.coordinates.longitude"
          :altitude="vehicle.altitude"
          :airspeed="vehicle.airspeed"
        />
      </div>
    </SidebarContent>
  </Sidebar>
</template>
