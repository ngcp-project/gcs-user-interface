<script setup lang="ts">
import { ref, computed} from 'vue';
import { Sidebar, SidebarContent, SidebarHeader, SidebarFooter } from '@/components/ui/sidebar';
import BreadcrumbNav from '@/components/SidebarCards/BreadcrumbNav.vue'
import MissionView from '@/views/MissionView.vue'
import VehicleView from '@/views/VehicleView.vue'
import StageView from '@/views/StageView.vue'
import ZoneView from '@/views/ZoneView.vue'
import { Plus } from 'lucide-vue-next'

const state = ref('mission'); // Default state

const currentView = computed(() => {
  switch (state.value) {
    case 'mission':
      return MissionView;
    case 'vehicle':
      return VehicleView;
    case 'stage':
      return StageView;
    case 'zone':
      return ZoneView;
    default:
      return null;
  }

});

// Function to add a mission via MissionView's exposed method
const missionViewRef = ref<InstanceType<typeof MissionView> | null>(null);

const addMission = () => {
  if (missionViewRef.value?.addMission) {
    missionViewRef.value.addMission();
  } else {
    console.warn("missionViewRef is not available yet.");
  }
};

// Function to add a stage via StageView's exposed method
const stageViewRef = ref<InstanceType<typeof StageView> | null>(null);

const addStage = () => {
  if (stageViewRef.value?.addStage) {
    stageViewRef.value.addStage();
  } else {
    console.warn("stageViewRef is not available yet.");
  }
};

// Function to change views using the breadcrumb
const navigateTo = (view: string) => {
  state.value = view;
};

</script>

<template>
  <!-- Mission View -->
  <Sidebar v-if="state === 'mission'" side="right">
    <SidebarHeader class="items-center bg-sidebar-background">
      <BreadcrumbNav :currentState="state" :navigateTo="navigateTo" />
    </SidebarHeader>

    <SidebarContent class="bg-sidebar-background">
      <component :is="currentView" ref="missionViewRef"/>
    </SidebarContent>

    <SidebarFooter class="bg-sidebar-background">
      <Button 
        @click="addMission"
        class="bg-transparent shadow-none text-background flex flex-col items-center "
      >
        <Plus class="w-5 h-5" />
        Add Mission
      </Button>
    </SidebarFooter>

  </Sidebar>

  <!-- Vehicle View -->
  <Sidebar v-if="state === 'vehicle'" side="right">
    <SidebarHeader class="bg-sidebar-background items-center">
      <BreadcrumbNav :currentState="state" :navigateTo="navigateTo" />
    </SidebarHeader>

    <SidebarContent class="bg-sidebar-background">
      <component :is="currentView" />
    </SidebarContent>
  </Sidebar>

  <!-- Stage View -->
  <Sidebar v-if="state === 'stage'" side="right">
    <SidebarHeader class="bg-sidebar-background items-center">
      <BreadcrumbNav :currentState="state" :navigateTo="navigateTo" />
    </SidebarHeader>

    <SidebarContent class="bg-sidebar-background">
      <component :is="currentView" ref="stageViewRef"/>
    </SidebarContent>

    <SidebarFooter class="bg-sidebar-background">
      <Button 
        @click="addStage"
        class="bg-transparent shadow-none text-background flex flex-col items-center"
      >
        <Plus class="w-5 h-5" />
        Add Stage
      </Button>
    </SidebarFooter>
  </Sidebar>

  <!-- Zone View -->
  <Sidebar v-if="state === 'zone'" side="right">
    <SidebarHeader class=" items-center">
      Zones
    </SidebarHeader>
    <SidebarContent class="bg-sidebar-background">
      <component :is="currentView" />
    </SidebarContent>
  </Sidebar>
</template>
