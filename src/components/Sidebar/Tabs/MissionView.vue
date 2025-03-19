<script setup lang="ts">
import MissionCard from "@/components/Sidebar/SidebarCards/MissionCard.vue";
import { missionStore } from "@/lib/MissionStore";
import { computed } from "vue";
import { SidebarContent, SidebarFooter } from "@/components/ui/sidebar";
import { Plus } from "lucide-vue-next";

const missions = computed(() => missionStore.view.getAllMissions());

const handleClick = (missionId: number) => {
  missionStore.view.setCurrentView("vehicle");
  missionStore.view.setCurrentMissionId(missionId);
};
</script>

<template>
  <SidebarContent class="bg-sidebar-background">
    <div class="flex w-full flex-col items-center">
      <div v-if="missions.length > 0" class="w-full space-y-4">
        <MissionCard
          v-for="(mission, index) in missions"
          :key="index"
          :missionId="mission.mission_id"
          @click="handleClick(mission.mission_id)"
        />
      </div>
    </div>
  </SidebarContent>
  <SidebarFooter class="bg-sidebar-background">
    <Button
      @click="missionStore.view.addMission"
      class="flex flex-col items-center bg-transparent text-background shadow-none"
    >
      <Plus class="h-5 w-5" />
      Add Mission
    </Button>
  </SidebarFooter>
</template>
