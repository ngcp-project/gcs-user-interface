<script setup lang="ts">
import MissionCard from "@/components/SidebarCards/MissionCard.vue";
import { missionStore } from "@/lib/MissionStore";
import { computed } from "vue";

const missions = computed(() => missionStore.view.getAllMissions());
</script>

<template>
  <div class="flex w-full flex-col items-center">
    <div v-if="missions.length > 0" class="w-full space-y-4">
      <MissionCard
        v-for="(mission, index) in missions"
        :key="index"
        :missionNumber="index"
        :missionName="mission.mission_name"
        :status="mission.mission_status"
      />
    </div>
  </div>
</template>
