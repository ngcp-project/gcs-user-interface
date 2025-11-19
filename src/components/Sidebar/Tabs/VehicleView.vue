<script setup lang="ts">
import VehicleCard from "@/components/Sidebar/SidebarCards/VehicleCard.vue";
import { SidebarContent, SidebarGroup } from "@/components/ui/sidebar";
import { VehicleEnum } from "@/lib/bindings";
import { missionStore } from "@/lib/StoresSync";

const handleClick = (vehicleName: VehicleEnum) => {
  missionStore.setCurrentView("stage");
  missionStore.setCurrentVehicleName(vehicleName);
};
</script>

<template>
  <SidebarContent class="bg-sidebar-background">
    <SidebarGroup>
      <VehicleCard vehicleName="ERU" @click="handleClick('ERU')" />
      <VehicleCard vehicleName="MEA" @click="handleClick('MEA')" />
      <VehicleCard vehicleName="MRA" @click="handleClick('MRA')" />
    </SidebarGroup>
  </SidebarContent>
</template>
