<script setup lang="ts">
import MissionCard from "@/components/Sidebar/SidebarCards/MissionCard.vue";
import { missionStore } from "@/lib/MissionStore";
import { ref, watch } from "vue";
import { SidebarContent, SidebarFooter } from "@/components/ui/sidebar";
import { Plus } from "lucide-vue-next";
import { Button } from "@/components/ui/button";

const missions = missionStore.getAllMissions();
const handleClick = (missionId: number) => {
  missionStore.setCurrentView("vehicle");
  missionStore.setCurrentMissionID(missionId);
};


</script>

<template>
  <SidebarContent class="bg-sidebar-background">
    <div class="flex w-full flex-col items-center">
      <div v-if="missions && missions.length > 0" class="w-full space-y-4">
        <MissionCard v-for="(mission, index) in missions" :key="index" :missionId="mission.mission_id"
          @click="handleClick(mission.mission_id)" />
      </div>
    </div>
  </SidebarContent>
  <SidebarFooter class="bg-sidebar-background">
    <Button @click="missionStore.createNewMission('new mission')"
      class="flex flex-col items-center bg-transparent text-background shadow-none hover:bg-transparent">
      <Plus class="h-5 w-5" />
      Add Mission
    </Button>
  </SidebarFooter>
</template>
