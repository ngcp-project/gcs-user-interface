<script setup lang="ts">
import { ref } from "vue";
import StageCard from "@/components/Sidebar/SidebarCards/StageCard.vue";
import { SidebarContent, SidebarFooter } from "@/components/ui/sidebar";
import { Plus } from "lucide-vue-next";
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
  <SidebarContent class="bg-sidebar-background">
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
  </SidebarContent>
  <SidebarFooter class="bg-sidebar-background">
    <Button class="flex flex-col items-center bg-transparent text-background shadow-none">
      <Plus class="h-5 w-5" />
      Add Stage
    </Button>
  </SidebarFooter>
</template>
