<script setup lang="ts">
import { Card, CardContent, CardTitle } from "@/components/ui/card";
import CardFooter from "@/components/ui/card/CardFooter.vue";
import EmergencyStop from "@/components/VehicleStatus/EmergencyStop.vue";
import Battery from "@/components/VehicleStatus/VehicleBattery.vue";
import Connection from "@/components/VehicleStatus/VehicleConnection.vue";
import { constants } from "os";
import { computed, ref, watch } from "vue";
import { missionStore } from "@/lib/MissionStore";
import { telemetryStore } from "@/lib/TelemetryStore";
import { VehicleEnum } from "@/lib/bindings";

// Define Props
const props = defineProps<{
  vehicleName: VehicleEnum;
  battery: number;
  connection: number;
  latitude: number;
  longitude: number;
  altitude: number;
  airspeed: number;
}>();

const vehicleTelemetry = computed(() => telemetryStore.state[props.vehicleName]);


const currentVehicleName = missionStore.view.currentVehicleName;
const currentMissionId = missionStore.state.current_mission
const currentStageId = missionStore.getVehicleData(currentMissionId, props.vehicleName)?.current_stage

const currentStage = (currentStageId 
  && missionStore.getStageData(currentMissionId, props.vehicleName, currentStageId)?.stage_name) 
  
const currentStageName = currentStage || "No Stage Available"
const vehicleData = computed(() => {
  if (currentMissionId !== null && currentVehicleName !== null) {
    return missionStore.getVehicleData(currentMissionId, currentVehicleName);
  }
  return null;
});

const isAuto = computed(() => {
  return vehicleData.value?.is_auto ?? false; // Default to false if not available
});

// const stateUpdate = computed((prev: boolean | undefined) => {
//   // read from missionStore.state as a dependency
//   missionStore;
//   // return a boolean value that switches between true and false
//   return !prev;
// });
// watch(() => missionStore.state, () => {
//   stateUpdate.value = ! s
// }, { immediate: true });
</script>

<template>
  <Card class="relative m-2 bg-sidebar-foreground p-2 text-foreground">
    <!-- Vehicle Name -->
    <CardTitle class="text-xl font-bold">{{ vehicleName }}</CardTitle>

    <CardContent class="mt-1 flex flex-col items-start space-y-3">
      <!-- Battery & Connection Info -->
      <section class="flex items-center justify-between gap-x-2 py-1">
        <!-- <Connection :latency="connection" :display-latency="false" /> -->
        <span class="font-semibold">
          {{ `${vehicleTelemetry.signal_strength}ms` }}
        </span>
        <Battery :percentage="vehicleTelemetry.battery_life" :charging="false" />
        <span class="font-semibold">
          {{ `${vehicleTelemetry.battery_life}%` }}
        </span>
      </section>

      <!-- Coordinates and Altitude/Airspeed in two columns -->
      <section class="mt-1 flex">
        <!-- First Column -->
        <div class="mr-4 flex flex-col">
          <span class="font-semibold">
            <!-- .toFixed(3) truncates number to 3 decimal places, this is just a bandage fix to not mess up the format with long numbers -->
            LAT: {{ Number(vehicleTelemetry.current_position.latitude).toFixed(3) }}
          </span>
          <span class="font-semibold"> ALT: {{ Number(vehicleTelemetry.altitude).toFixed(3) }} </span>
        </div>

        <!-- Second Column -->
        <div class="flex flex-col">
          <!-- .toFixed(3) truncates number to 3 decimal places, this is just a bandage fix to not mess up the format with long numbers -->
          <span class="font-semibold"> LON: {{ Number(vehicleTelemetry.current_position.longitude).toFixed(3) }} </span>
          <span class="font-semibold"> TAS: {{ Number(vehicleTelemetry.speed).toFixed(3) }} </span>
        </div>
      </section>

      <!-- Stage -->
      <!-- <section class="mt-1 flex">
        Stage:
        {{ currentStage }}
      </section> -->

      <!-- Status -->
      <section class="mt-1 flex">
        Status:
        {{ isAuto ? "Auto" : "Manual" }}
      </section>

      <CardFooter class="mt-4 flex w-full justify-center">
        <EmergencyStop :vehicle-name="vehicleName" />
      </CardFooter>
    </CardContent>
  </Card>
</template>
