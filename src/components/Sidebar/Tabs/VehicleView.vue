<script setup lang="ts">
import VehicleCard from "@/components/Sidebar/SidebarCards/VehicleCard.vue";
import { SidebarContent, SidebarGroup } from "@/components/ui/sidebar";
import { missionStore } from "@/lib/MissionStore";

const currentMissionId = missionStore.view.tabState.currentMissionId;

const currentMission =
  currentMissionId !== null ? missionStore.getMissionData(currentMissionId) : null;
</script>

<template>
  <SidebarContent class="bg-sidebar-background">
    <SidebarGroup>
      <VehicleCard vehicleName="ERU" patientStatus="Secured" />
      <VehicleCard vehicleName="MEA" patientStatus="Unsecured" />
      <VehicleCard vehicleName="MRA" patientStatus="Unsecured" />
    </SidebarGroup>
  </SidebarContent>
</template>
