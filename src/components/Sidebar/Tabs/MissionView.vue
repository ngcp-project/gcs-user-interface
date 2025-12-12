<script setup lang="ts">
import MissionCard from "@/components/Sidebar/SidebarCards/MissionCard.vue";
import { missionStore} from "@/lib/StoresSync";
import { ref, watch } from "vue";
import { SidebarContent, SidebarFooter, SidebarGroup } from "@/components/ui/sidebar";
import { Plus } from "lucide-vue-next";
import { Button } from "@/components/ui/button";

const missions = missionStore.getAllMissions().value?.missions;
const handleClick = (missionId: number) => {
  missionStore.setCurrentView("vehicle");
  missionStore.setCurrentMissionID(missionId);
};


</script>

<template>
  <SidebarContent class="bg-sidebar-background">
  <SidebarGroup>
    <div class="flex w-full flex-col items-center">
      <div v-if="missions && missions.length > 0" class="w-full space-y-4">
        <MissionCard v-for="(mission, index) in missions" :key="index" :missionId="mission.mission_id"
          @click="handleClick(mission.mission_id)" />
      </div>
      <div v-else class="w-full text-center">
        <span>No Mission Created</span>
      </div>
    </div>
</SidebarGroup>
  </SidebarContent>
  <SidebarFooter class="bg-sidebar-background">
    <Button @click="missionStore.createNewMission('new mission')"
      class="flex flex-col items-center bg-transparent text-background shadow-none hover:bg-transparent">
      <Plus class="h-5 w-5" />
      Add Mission
    </Button>
  </SidebarFooter>
</template>
