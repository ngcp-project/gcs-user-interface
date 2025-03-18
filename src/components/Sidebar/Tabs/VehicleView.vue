<script setup lang="ts">
import VehicleCard from "@/components/Sidebar/SidebarCards/VehicleCard.vue";
</script>

<template>
  <div>
    <SidebarGroup>
      <VehicleCard vehicleName="ERU" patientStatus="Secured" />
      <VehicleCard vehicleName="MEA" patientStatus="Unsecured" />
      <VehicleCard vehicleName="MRA" patientStatus="Unsecured" />
    </SidebarGroup>
  </div>
</template>
