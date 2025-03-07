<script setup lang="ts">
import { Sidebar, SidebarContent } from '@/components/ui/sidebar';
import DeployCard from './SidebarCards/CameraScreen/DeployCard.vue';
import SidebarHeader from './ui/sidebar/SidebarHeader.vue';
import EmergencyStopDialog from './VehicleStatus/EmergencyStopDialog.vue';

defineProps<{
  vehicles: {
    vehicleName: 'ERU' | 'MEA' | 'MRA';
    cameraID: number;
    batteryPct: number;
    connection: number;
    coordinates: {
      latitude: number;
      longitude: number;
    }
  }[];
}>();
</script>

<template>
  <Sidebar>
    <SidebarHeader>
      <EmergencyStopDialog :vehicle-name="'all'" />
    </SidebarHeader>

    <SidebarContent>
      <div v-for="(vehicle, index) in vehicles" :key="index">
        <DeployCard
          :vehicle-name="vehicle.vehicleName"
          :battery="vehicle.batteryPct"
          :connection="vehicle.connection"
          :latitude="vehicle.coordinates.latitude" 
          :longitude="vehicle.coordinates.longitude"
        />
      </div>
    </SidebarContent>
  </Sidebar>
</template>