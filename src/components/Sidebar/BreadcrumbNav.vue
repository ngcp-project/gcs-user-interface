<script setup lang="ts">
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
      <BreadcrumbItem
        :active="missionStore.view.tabState.currentView === 'mission'"
        class="cursor-pointer text-secondary"
        @click="missionStore.view.tabState.setCurrentView('mission')"
      >
        Mission
      </BreadcrumbItem>
      <BreadcrumbSeparator class="text-secondary" />
      <BreadcrumbItem
        :active="missionStore.view.tabState.currentView === 'vehicle'"
        class="cursor-pointer text-secondary"
        @click="
          missionStore.view.tabState.currentView !== 'mission' &&
            missionStore.view.tabState.setCurrentView('vehicle')
        "
      >
        Vehicle
      </BreadcrumbItem>
      <BreadcrumbSeparator class="text-secondary" />
      <!-- Can only go up from hierarchy not down so add currentView checks -->
      <BreadcrumbItem
        :active="missionStore.view.tabState.currentView === 'stage'"
        class="cursor-pointer text-secondary"
        @click="
          missionStore.view.tabState.currentView !== 'mission' &&
            missionStore.view.tabState.currentView !== 'vehicle' &&
            missionStore.view.tabState.setCurrentView('stage')
        "
      >
        Stage
      </BreadcrumbItem>
    </BreadcrumbList>
  </Breadcrumb>
</template>
