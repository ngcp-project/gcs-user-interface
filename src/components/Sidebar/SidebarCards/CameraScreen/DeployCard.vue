<script setup lang="ts">
import { Card, CardContent, CardTitle } from '@/components/ui/card'
import CardFooter from '@/components/ui/card/CardFooter.vue';
import EmergencyStop from '@/components/VehicleStatus/EmergencyStop.vue';
import Battery from "@/components/VehicleStatus/VehicleBattery.vue";
import Connection from "@/components/VehicleStatus/VehicleConnection.vue";
import { constants } from 'os';
import { computed } from "vue";
import { missionStore } from "@/lib/MissionStore";

// Define Props
defineProps<{
  vehicleName: 'ERU' | 'MEA' | 'MRA'
  battery: number
  connection: number
  latitude: number
  longitude: number
  altitude: number
  airspeed: number
}>()

const currentMissionId = missionStore.view.currentMissionId;
const currentVehicleName = missionStore.view.currentVehicleName;

const currentStage = computed(() => {
  if (currentMissionId !== null && currentVehicleName !== null)
    return missionStore.getVehicleData(currentMissionId, currentVehicleName)?.stages[currentVehicleName.current_stage];
  else return "No Stages Available"; 
});

const vehicleData = computed(() => {
  if (currentMissionId !== null && currentVehicleName !== null) {
    return missionStore.getVehicleData(currentMissionId, currentVehicleName);
  }
  return null;
});

const isAuto = computed(() => {
  return vehicleData.value?.is_auto ?? false; // Default to false if not available
});
</script>

<template>
  <Card class="m-2 p-2 relative bg-sidebar-foreground text-foreground">
    <!-- Vehicle Name -->
    <CardTitle class="text-xl font-bold">{{ vehicleName }}</CardTitle>

    <CardContent class="mt-1 flex flex-col items-start space-y-3">
      <!-- Battery & Connection Info -->
      <section class="flex items-center justify-between py-1 gap-x-2">
        <Connection :latency="connection" :display-latency="false" />
        <span class="font-semibold">
          {{ `${connection}ms` }}
        </span>
        <Battery :percentage="battery" :charging="false" />
        <span class="font-semibold">
          {{ `${battery}%` }}
        </span>
      </section>

      <!-- Coordinates and Altitude/Airspeed in two columns -->
      <section class="mt-1 flex">
          <!-- First Column -->
          <div class="flex flex-col mr-4">
            <span class="font-semibold">
              LAT: {{ latitude }}
            </span>
            <span class="font-semibold">
              ALT: {{ altitude }}
            </span>
          </div>
          
          <!-- Second Column -->
          <div class="flex flex-col">
            <span class="font-semibold">
              LON: {{ longitude }}
            </span>
            <span class="font-semibold">
              TAS: {{ airspeed }}
            </span>
          </div>
        </section>

      <!-- Stage -->
      <section class="mt-1 flex">
        Stage: 
        {{ currentStage }}
      </section>

      <!-- Status -->
      <section class="mt-1 flex">
        Status: 
        {{ isAuto ? "Auto" : "Manual" }}
      </section>

      <CardFooter class="mt-4 flex w-full justify-center">
        <EmergencyStop :vehicle-name="vehicleName"/>
      </CardFooter>
    </CardContent>
  </Card>
</template>