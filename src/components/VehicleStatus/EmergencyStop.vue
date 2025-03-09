<template>
  <EmergencyStopDialog :vehicle-name="vehicleName">
    <Button class="h-8" variant="destructive">STOP</Button>
  </EmergencyStopDialog>
</template>

<script setup lang="ts">
import { Button } from "../ui/button";
import EmergencyStopDialog from "./EmergencyStopDialog.vue";

defineProps<{
  vehicleName: string;
}>();
</script>
