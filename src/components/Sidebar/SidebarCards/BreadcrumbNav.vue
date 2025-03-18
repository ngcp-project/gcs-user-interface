<script setup lang="ts">
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbSeparator
} from "@/components/ui/breadcrumb";
import { missionStore } from "@/lib/MissionStore";

defineProps<{
  currentState: string;
}>();
</script>

<template>
  <Breadcrumb>
    <BreadcrumbList>
      <BreadcrumbItem
        :active="currentState === 'mission'"
        class="cursor-pointer text-secondary"
        @click="missionStore.view.setCurrentView('mission')"
      >
        Mission
      </BreadcrumbItem>
      <BreadcrumbSeparator class="text-secondary" />
      <BreadcrumbItem
        :active="currentState === 'vehicle'"
        class="cursor-pointer text-secondary"
        @click="missionStore.view.setCurrentView('vehicle')"
      >
        Vehicle
      </BreadcrumbItem>
      <BreadcrumbSeparator class="text-secondary" />
      <BreadcrumbItem
        :active="currentState === 'stage'"
        class="cursor-pointer text-secondary"
        @click="missionStore.view.setCurrentView('stage')"
      >
        Stage
      </BreadcrumbItem>
    </BreadcrumbList>
  </Breadcrumb>
</template>
