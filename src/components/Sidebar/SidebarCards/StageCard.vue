<script setup lang="ts">
import { Card, CardContent, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { computed, ref } from "vue";
import { Trash2, Eye, EyeOff, Pencil } from "lucide-vue-next";
import { missionStore } from "@/lib/MissionStore";
import mapStore from "@/lib/MapStore";

const props = defineProps<{
  stageID: number;
  stageIndex: number;
}>();

// Status Styles
const statusStyles = {
  statusColor: {
    Inactive: "text-muted-foreground font-semibold",
    Failed: "text-destructive font-semibold",
    Active: "text-chart-4 font-semibold",
    Complete: "text-chart-2 font-semibold"
  }
};

// Toggle Eye Icon
const isVisible = ref(true); // Track visibility state

const toggleVisibility = () => {
  isVisible.value = !isVisible.value;
};

// Get id of the current mission
const currentMissionId = missionStore.view.currentMissionId;

// Get the current vehicle name
const currentVehicleName = currentMissionId !== null ? missionStore.view.currentVehicleName : null;

// Get stage data
const stage = currentMissionId !== null && currentVehicleName !== null ? missionStore.getStageData(
  currentMissionId, 
  currentVehicleName, 
  props.stageID
) : null;

// Get vehicle data
const vehicleData = currentMissionId !== null && currentVehicleName !== null ? missionStore.getVehicleData(
  currentMissionId, 
  currentVehicleName
) : null;

// Get list of stages from vehicle data
const stageList = vehicleData !== undefined && vehicleData !== null ? vehicleData.stages : null;

// Get index of active stage
const currStageIndex = vehicleData !== undefined && vehicleData !== null ? vehicleData.current_stage : null;

// Compare current stage index with active stage index
const searchAreaEditable =
stageList !== undefined && stageList !== null && 
currStageIndex !== undefined && currStageIndex !== null ? 
props.stageIndex >= currStageIndex : null;

// Handle stage name change
const handleStageNameChange = (event: Event) => {
  if (currentMissionId === null || currentVehicleName === null) return;
  const newName = (event.target as HTMLInputElement).value;
  missionStore.renameStage(
    currentMissionId, 
    currentVehicleName, 
    props.stageID, 
    newName
  );
};

const handleDeleteStage = () => {
  if (currentMissionId === null || currentVehicleName === null) return;
  missionStore.deleteStage(currentMissionId, currentVehicleName, props.stageID);
  mapStore.removeStageLayer(currentMissionId, currentVehicleName, props.stageID);
};
</script>

<template>
  <Card v-if="stage" class="relative m-2 p-2">
    <!-- Stage Title -->
    <CardTitle class="flex items-center gap-2">
      <Input
        v-if="stage.stage_status === 'Inactive'"
        @blur="handleStageNameChange"
        @keyup.enter="handleStageNameChange"
        v-model="stage.stage_name"
        class="flex-1"
      />
      <span v-else class="flex-1">
        {{ stage.stage_name }}
      </span>

      <!-- Trash Icon -->
      <Trash2
        v-if="stage.stage_status === 'Inactive' && currentMissionId !== null && currentVehicleName !== null"
        @click="handleDeleteStage"
        class="h-5 w-5 cursor-pointer text-foreground hover:text-destructive"
      />
    </CardTitle>

    <!-- Status Section -->
    <CardContent class="mt-2 flex flex-col items-start">
      <span class="font-semibold">
        Status:
        <span :class="statusStyles.statusColor[stage.stage_status]">{{ stage.stage_status }}</span>
      </span>
      <div class="flex w-full items-center justify-between">
        <span class="font-semibold">Search Area</span>
        <div class="flex gap-x-2">
          <Pencil
          v-if="searchAreaEditable"
          class="h-5 w-5 cursor-pointer text-secondary-foreground hover:text-secondary-foreground/80"
          />
          <component
            :is="isVisible ? Eye : EyeOff"
            class="h-5 w-5 cursor-pointer text-secondary-foreground hover:text-secondary-foreground/80"
            @click="toggleVisibility"
          />
        </div>
      </div>
    </CardContent>
  </Card>
</template>
