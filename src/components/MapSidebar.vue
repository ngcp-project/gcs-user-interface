<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { Sidebar, SidebarContent, SidebarHeader, SidebarFooter } from "@/components/ui/sidebar";
import BreadcrumbNav from "@/components/Sidebar/SidebarCards/BreadcrumbNav.vue";
import MissionView from "@/components/Sidebar/Tabs/MissionView.vue";
import VehicleView from "@/components/Sidebar/Tabs/VehicleView.vue";
import StageView from "@/components/Sidebar/Tabs/StageView.vue";
// import ZoneView from '@/views/ZoneView.vue'
import { Plus } from "lucide-vue-next";

import { missionStore } from "@/lib/MissionStore";

const currentView = computed(() => missionStore.view.currentView);

// Function to add a stage via StageView's exposed method
const stageViewRef = ref<InstanceType<typeof StageView> | null>(null);

const addStage = () => {
  if (stageViewRef.value?.addStage) {
    stageViewRef.value.addStage();
  } else {
    console.warn("stageViewRef is not available yet.");
  }
};
</script>

<template>
  <!-- Mission View -->
  <Sidebar v-if="missionStore.view.currentView === 'mission'" side="right">
    <SidebarHeader class="bg-sidebar-background items-center">
      <BreadcrumbNav :currentState="currentView" />
    </SidebarHeader>

    <SidebarContent class="bg-sidebar-background">
      <MissionView ref="missionViewRef" />
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
  </Sidebar>

  <!-- Vehicle View -->
  <Sidebar v-if="missionStore.view.currentView === 'vehicle'" side="right">
    <SidebarHeader class="bg-sidebar-background items-center">
      <BreadcrumbNav :currentState="currentView" />
    </SidebarHeader>

    <SidebarContent class="bg-sidebar-background">
      <VehicleView ref="vehicleViewRef" />
    </SidebarContent>
  </Sidebar>

  <!-- Stage View -->
  <Sidebar v-if="missionStore.view.currentView === 'stage'" side="right">
    <SidebarHeader class="bg-sidebar-background items-center">
      <BreadcrumbNav :currentState="currentView" />
    </SidebarHeader>

    <SidebarContent class="bg-sidebar-background">
      <StageView ref="stageViewRef" />
    </SidebarContent>

    <SidebarFooter class="bg-sidebar-background">
      <Button
        @click="addStage"
        class="flex flex-col items-center bg-transparent text-background shadow-none"
      >
        <Plus class="h-5 w-5" />
        Add Stage
      </Button>
    </SidebarFooter>
  </Sidebar>

  <!-- Zone View -->
  <Sidebar v-if="missionStore.view.currentView === 'zone'" side="right">
    <SidebarHeader class="items-center"> Zones </SidebarHeader>
    <SidebarContent class="bg-sidebar-background">
      <!-- Zone View Component -->
    </SidebarContent>
  </Sidebar>
</template>
