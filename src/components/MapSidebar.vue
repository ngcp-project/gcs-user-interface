<script setup lang="ts">
import { computed, render } from "vue";
import { Sidebar, SidebarHeader } from "@/components/ui/sidebar";
import BreadcrumbNav from "@/components/Sidebar/BreadcrumbNav.vue";
import MissionView from "@/components/Sidebar/Tabs/MissionView.vue";
import VehicleView from "@/components/Sidebar/Tabs/VehicleView.vue";
import StageView from "@/components/Sidebar/Tabs/StageView.vue";
import { ChevronLeft } from "lucide-vue-next";
import ZoneView from "@/components/Sidebar/Tabs/ZoneView.vue";
import { missionStore } from "@/lib/StoresSync";

const currentView = missionStore.getCurrentView();

// Whenever missionStore.state changes trigger a rerender fo the sidebar
const stateUpdate = computed((prev: boolean | undefined) => {
  // read from missionStore.state as a dependency
  missionStore.getAllMissions().value;
  // return a boolean value that switches between true and false
  return !prev;
});

// change view and title based on currentView
const renderView = {
  mission: { component: MissionView, title: "Missions" },
  vehicle: { component: VehicleView, title: "Vehicles" },
  stage: { component: StageView, title: "Stages" },
  zone: { component: ZoneView, title: "Zones" }
};

// current title based on currentView
const currentTitle = computed(() => renderView[currentView.value]?.title || "Title");

// handle back button
const handleBack = () => {
  if (currentView.value === 'stage') {
    missionStore.setCurrentView('vehicle');
  } else if (currentView.value === 'vehicle') {
    missionStore.setCurrentView('mission');
  }else if (currentView.value === 'zone') {
    missionStore.setCurrentView('mission');
  }
};
</script>

<template>
  <Sidebar :key="String(stateUpdate)" side="right">
    <SidebarHeader class="bg-sidebar-background items-center">
      <div class="flex items-center gap-2">
        <!-- Back Button -->
        <button
          v-if="currentView === 'stage' || currentView === 'vehicle' || currentView === 'zone'"
          @click="handleBack"
          class="absolute left-4 text-primary-foreground bg-transparent"
        >
          <ChevronLeft class="w-5 h-5" />
        </button>
        <!-- Title -->
        <span class="text-xl font-semibold">
          {{ currentTitle }}
        </span>
      </div>
      <BreadcrumbNav 
      v-if="currentView !== 'zone'"
      :currentState="currentView" 
      />
    </SidebarHeader>
    <!-- Access the component from renderView -->
    <component :is="renderView[currentView].component" />
  </Sidebar>
</template>
