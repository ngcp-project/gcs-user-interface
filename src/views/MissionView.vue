<script setup lang="ts">
import { ref } from 'vue'
import MissionCard from '@/components/SidebarCards/MissionCard.vue'

// Temporary mission data
const missions = ref<{ 
  MissionName: string; 
  MissionStatus: 'Active' | 'Inactive' | 'Complete' | 'Failed'; 
}[]>([
  {
    MissionName: "Mission 1",
    MissionStatus: "Complete",
  },
  {
    MissionName: "Mission 2",
    MissionStatus: "Inactive",
  }
])

// Function to add a new mission
const addMission = () => {
  const newMission = {
    MissionName: `Mission ${missions.value.length + 1}`,
    MissionStatus: 'Inactive' as const,
    Zones: {
      KeepInZones: [],
      KeepOutZones: []
    }
  }
  missions.value.push(newMission)
}

defineExpose({ addMission }); // Expose addMission to side bar

// Function to delete a mission
const deleteMission = (missionIndex: number) => {
  missions.value.splice(missionIndex, 1)
}
</script>

<template>
  <div class="flex flex-col items-center w-full">
    <div v-if="missions.length > 0" class="w-full space-y-4">
      <MissionCard 
        v-for="(mission, index) in missions" 
        :key="index" 
        :missionNumber="index" 
        :missionName="mission.MissionName" 
        :status="mission.MissionStatus"
        @deleteMission="deleteMission(index)"
      />
    </div>
  </div>
</template>
