<script setup lang="ts">
import { ref, computed, onMounted, defineComponent } from "vue";
import { Sidebar, SidebarContent, SidebarHeader, SidebarFooter } from "@/components/ui/sidebar";
import BreadcrumbNav from "@/components/Sidebar/BreadcrumbNav.vue";
import MissionView from "@/components/Sidebar/Tabs/MissionView.vue";
import VehicleView from "@/components/Sidebar/Tabs/VehicleView.vue";
import StageView from "@/components/Sidebar/Tabs/StageView.vue";
// import ZoneView from '@/views/ZoneView.vue'

import { missionStore } from "@/lib/MissionStore";

const currentView = computed(() => missionStore.view.currentView);

// Function to add a stage via StageView's exposed method
const stageViewRef = ref<InstanceType<typeof StageView> | null>(null);

// change view based on currentView
const renderView = {
  mission: MissionView,
  vehicle: VehicleView,
  stage: StageView,
  zone: MissionView
};
</script>

<template>
  <!-- Mission View -->
  <Sidebar side="right">
    <SidebarHeader class="bg-sidebar-background items-center">
      <BreadcrumbNav :currentState="currentView" />
    </SidebarHeader>
    <component :is="renderView[currentView]" />
  </Sidebar>
</template>
