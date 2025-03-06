<template>
  <EmergencyStopDialog :vehicle-name="vehicleName">
    <Button variant="destructive" size="xs">EMERGENCY STOP</Button>
  </EmergencyStopDialog>
</template>

<script setup lang="ts">
import { Button } from "../ui/button";
import EmergencyStopDialog from "./EmergencyStopDialog.vue";

defineProps<{
  vehicleName: string;
}>();
</script>
