<script setup lang="ts">
import { computed } from "vue";
import { Sidebar, SidebarHeader } from "@/components/ui/sidebar";
import BreadcrumbNav from "@/components/Sidebar/BreadcrumbNav.vue";
import MissionView from "@/components/Sidebar/Tabs/MissionView.vue";
import VehicleView from "@/components/Sidebar/Tabs/VehicleView.vue";
import StageView from "@/components/Sidebar/Tabs/StageView.vue";
// import ZoneView from '@/views/ZoneView.vue'

import { missionStore } from "@/lib/MissionStore";

const currentView = computed(() => missionStore.view.currentView);

// Whenever missionStore.state changes trigger a rerender fo the sidebar
const stateUpdate = computed((prev: boolean | undefined) => {
  // read from missionStore.state as a dependency
  missionStore.state;
  // return a boolean value that switches between true and false
  return !prev;
});

// change view and title based on currentView
const renderView = {
  mission: { component: MissionView, title: "Missions" },
  vehicle: { component: VehicleView, title: "Vehicles" },
  stage: { component: StageView, title: "Stages" },
  zone: { component: MissionView, title: "Zones" },
};

// current title based on currentView
const currentTitle = computed(() => renderView[currentView.value]?.title || "Title");

</script>

<template>
  <Sidebar side="right">
    <SidebarHeader class="bg-sidebar-background items-center">
      <span class="text-xl font-semibold">
        {{ currentTitle }}
      </span>
      <BreadcrumbNav :currentState="currentView" />
    </SidebarHeader>
    <!-- Access the component from renderView -->
    <component :key="String(stateUpdate)" :is="renderView[currentView].component" />
  </Sidebar>
</template>
