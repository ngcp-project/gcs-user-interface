<script setup lang="ts">
import { ref, computed } from "vue";
import { Card, CardContent, CardTitle, CardFooter } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { VehicleEnum } from "@/lib/bindings";
import { missionStore } from "@/lib/MissionStore";

// Define Props
const props = defineProps<{
  vehicleName: VehicleEnum;
}>();

const missionId = missionStore.view.tabState.currentMissionId;

const vehicle =
  missionId !== null ? missionStore.getVehicleData(missionId, props.vehicleName) : null;

const currentVehicleStage = computed(() => {
  if (vehicle === undefined || vehicle === null || missionId === null) {
    return null;
  }
  return missionStore.getStageData(missionId, props.vehicleName, vehicle.current_stage);
});

// const currentVehicleStageName = currentVehicleStage.value?.stage_name;
// Status Styles
const patientStatusStyles = computed(() => ({
  statusColor: {
    Secured: "text-chart-2 font-semibold",
    Unsecured: "text-destructive font-semibold"
  }
}));
console.log(vehicle, currentVehicleStage.value);
</script>

<template>
  <Card
    v-if="vehicle && missionId !== null"
    class="relative m-2 bg-sidebar-foreground p-2 text-foreground"
  >
    <!-- Vehicle Name -->
    <CardTitle class="text-xl font-bold">{{ vehicleName }}</CardTitle>

    <!-- Vehicle Stage & Patient Status -->
    <CardContent class="mt-2 flex flex-col items-start">
      <span
        v-if="
          vehicle.stages.length > 0 &&
          (currentVehicleStage !== null || currentVehicleStage !== undefined)
        "
        class="font-semibold"
      >
        Stage: {{ currentVehicleStage?.stage_name }}
      </span>
      <!-- MEA & ERU are the only vehicles with patient status -->
      <span v-if="vehicle.patient_status !== null" class="font-semibold">
        Patient Status:
        <span :class="patientStatusStyles.statusColor[vehicle.patient_status]">
          {{ vehicle.patient_status }}
        </span>
      </span>
    </CardContent>

    <!-- Next Stage Button -->
    <CardFooter class="mt-4 justify-start">
      <!-- TODO: Add logic to disable if vehicle is at last stage or if mission is not submitted or not started-->
      <Button
        :disabled="vehicle.current_stage === vehicle.stages.length - 1"
        @click.stop="missionStore.nextStage(missionId, props.vehicleName)"
      >
        Next Stage
      </Button>
    </CardFooter>
  </Card>
</template>
