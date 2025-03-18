<script setup lang="ts">
import { ref } from "vue";
import StageCard from "@/components/Sidebar/SidebarCards/StageCard.vue";

// Temporary stage data
const stages = ref<
  {
    StageName: string;
    StageStatus: "Incomplete" | "In Progress" | "Complete";
    SearchArea: string[];
  }[]
>([
  {
    StageName: "Takeoff",
    StageStatus: "Incomplete",
    SearchArea: ["0 0", "0 1", "1 1", "1 0"]
  },
  {
    StageName: "Search",
    StageStatus: "Incomplete",
    SearchArea: ["0 0", "0 1", "1 1", "1 0"]
  },
  {
    StageName: "Landing",
    StageStatus: "Incomplete",
    SearchArea: ["0 0", "0 1", "1 1", "1 0"]
  }
]);

// Function to add a new stage
const addStage = () => {
  const newStage = {
    StageName: `Stage ${stages.value.length + 1}`,
    StageStatus: "Incomplete" as const,
    SearchArea: []
  };
  stages.value.push(newStage);
};

defineExpose({ addStage }); // Expose addStage to side bar

// Function to delete a stage
const deleteStage = (stageIndex: number) => {
  stages.value.splice(stageIndex, 1);
};
</script>

<template>
  <div class="flex w-full flex-col items-center">
    <div v-if="stages.length > 0" class="w-full space-y-4">
      <StageCard
        v-for="(stage, index) in stages"
        :key="index"
        :missionNumber="index"
        :stageName="stage.StageName"
        :status="stage.StageStatus"
        :searchArea="stage.SearchArea"
        @deleteStage="deleteStage(index)"
      />
    </div>
  </div>
</template>
