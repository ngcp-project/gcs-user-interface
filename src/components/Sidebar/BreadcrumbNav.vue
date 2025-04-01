<script setup lang="ts">
import { computed } from "vue";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbSeparator
} from "@/components/ui/breadcrumb";
import { missionStore } from "@/lib/MissionStore";

</script>

<template>
  <Breadcrumb>
    <BreadcrumbList>
      <BreadcrumbItem :active="missionStore.view.currentView === 'mission'" class="cursor-pointer text-secondary"
        @click="missionStore.setCurrentView('mission')">
        Mission
      </BreadcrumbItem>
      <BreadcrumbSeparator class="text-secondary" />
      <BreadcrumbItem :active="missionStore.view.currentView === 'vehicle'" class="cursor-pointer text-secondary"
        @click="
          missionStore.view.currentView !== 'mission' &&
          missionStore.setCurrentView('vehicle')
          ">
        Vehicle
      </BreadcrumbItem>
      <BreadcrumbSeparator class="text-secondary" />
      <!-- Can only go up from hierarchy not down so add currentView checks -->
      <BreadcrumbItem :active="missionStore.view.currentView === 'stage'" class="cursor-pointer text-secondary" @click="
        missionStore.view.currentView !== 'mission' &&
        missionStore.view.currentView !== 'vehicle' &&
        missionStore.setCurrentView('stage')
        ">
        Stage
      </BreadcrumbItem>
    </BreadcrumbList>
  </Breadcrumb>
</template>
