<script setup lang="ts">
import { computed, watch, reactive } from "vue";
import { Button } from "@/components/ui/button";
import StageCard from "@/components/Sidebar/SidebarCards/StageCard.vue";
import { SidebarContent, SidebarFooter } from "@/components/ui/sidebar";
import { Plus } from "lucide-vue-next";
import { missionStore } from "@/lib/MissionStore";

const currentMissionId = missionStore.view.currentMissionId;
const currentVehicleName = missionStore.view.currentVehicleName;
const stages = currentMissionId !== null && currentVehicleName !== null ? missionStore.getVehicleData(currentMissionId, currentVehicleName)?.stages : [];

</script>

<template>
  <SidebarContent class="bg-sidebar-background">
    <div class="flex w-full flex-col items-center">
      <div v-if="stages && stages.length > 0" class="w-full space-y-4">
        <StageCard v-for="(stage, index) in stages" :key="index" :stageID="stage.stage_id" />
      </div>
      <div v-else class="w-full text-center">
        <span>No Stages Created</span>
      </div>

    </div>
  </SidebarContent>
  <SidebarFooter class="bg-sidebar-background">
    <Button class="flex flex-col items-center bg-transparent text-background shadow-none hover:bg-transparent" @click="
      currentMissionId !== null &&
      currentVehicleName !== null &&
      missionStore.addStage(currentMissionId, currentVehicleName)
      ">
      <Plus class="h-5 w-5" />
      Add Stage
    </Button>
  </SidebarFooter>
</template>
