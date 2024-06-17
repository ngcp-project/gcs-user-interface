<template>
  <EmergencyStopDialog :vehicle-name="vehicleName">
    <NgButton variant="destructive" size="xs">EMERGENCY STOP</NgButton>
  </EmergencyStopDialog>
</template>

<script setup lang="ts">
import { NgButton } from "../ui/button";
import EmergencyStopDialog from "./EmergencyStopDialog.vue";

defineProps<{
  vehicleName: string;
}>();
</script>
