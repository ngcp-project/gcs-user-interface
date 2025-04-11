<script setup lang="ts">
import { Card, CardContent, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { computed, ref } from "vue";
import { Trash2, Eye, EyeOff, Pencil } from "lucide-vue-next";
import { missionStore } from "@/lib/MissionStore";

const props = defineProps<{
  stageID: number;
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


const currentMissionId = missionStore.view.currentMissionId;
const currentVehicleName = missionStore.view.currentVehicleName;

const stage = computed(() => {
  if (currentMissionId !== null && currentVehicleName !== null)
    return missionStore.getStageData(currentMissionId, currentVehicleName, props.stageID);
});

</script>

<template>
  <Card v-if="stage" class="relative m-2 p-2">
    <!-- Stage Title -->
    <CardTitle class="flex items-center gap-2">
      <Input v-model="stage.stage_name" class="flex-1" />
      <!-- Trash Icon -->
      <Trash2 
        @click="missionStore.deleteStage(currentMissionId, currentVehicleName, props.stageID)"
        class="h-5 w-5 text-foreground hover:text-destructive cursor-pointer" 
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
          <Pencil class="h-5 w-5 cursor-pointer text-secondary-foreground hover:text-secondary-foreground/80" />
          <component :is="isVisible ? Eye : EyeOff"
            class="h-5 w-5 cursor-pointer text-secondary-foreground hover:text-secondary-foreground/80"
            @click="toggleVisibility" />
        </div>
      </div>
    </CardContent>
  </Card>
</template>
