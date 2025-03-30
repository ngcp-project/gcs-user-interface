<script setup lang="ts">
import { computed } from "vue";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbSeparator
} from "@/components/ui/breadcrumb";
import { missionStore } from "@/lib/MissionStore";



const currentView = computed(() => missionStore.view.currentView);

const handleClick = (targetView: "mission" | "vehicle" | "stage") => {
  if (targetView === "mission") {
    missionStore.setCurrentView("mission");
  } else if (targetView === "vehicle" && currentView.value == "stage") {
    missionStore.setCurrentVehicleName(null);
    missionStore.setCurrentView("vehicle");
  } else {
    console.error("Invalid targetView")
  }
  return;
}

</script>

<template>
  <Breadcrumb>
    <BreadcrumbList>
      <BreadcrumbItem class="cursor-pointer text-secondary" @click="handleClick('mission')">
        Mission
      </BreadcrumbItem>
      <BreadcrumbSeparator class="text-secondary" />
      <BreadcrumbItem class="cursor-pointer text-secondary" @click="handleClick('vehicle')">
        Vehicle
      </BreadcrumbItem>
      <BreadcrumbSeparator class="text-secondary" />
      <!-- Can only go up from hierarchy not down so add currentView checks -->
      <BreadcrumbItem class="cursor-pointer text-secondary" @click="handleClick('stage')">
        Stage
      </BreadcrumbItem>
    </BreadcrumbList>
  </Breadcrumb>
</template>
