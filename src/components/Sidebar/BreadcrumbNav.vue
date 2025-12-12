<script setup lang="ts">
import { computed } from "vue";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbSeparator
} from "@/components/ui/breadcrumb";
import { missionStore } from "@/lib/StoresSync";



const currentView = missionStore.getCurrentView();

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

const currentMissionId = missionStore.getCurrentMissionId();
const currentVehicleName = missionStore.getCurrentVehicleName();

const breadcrumbMissionName = computed(() => {
  if (currentMissionId.value !== null && currentView.value !== "mission") {
    const missionName = missionStore.getMissionData(currentMissionId.value).value?.mission_name;
    return missionName && missionName.length > 8 
      ? `${missionName.substring(0, 8)}...` 
      : missionName || "Missions";
  } else {
    return "Missions";
  }
});

const breadcrumbVehicleName = computed(() => {
  if (currentVehicleName.value !== null && currentView.value === "stage") {
    return currentVehicleName.value;
  } else {
    return "Vehicles";
  }
});

</script>

<template>
  <Breadcrumb>
    <BreadcrumbList>
      <BreadcrumbItem 
        class="cursor-pointer text-secondary" 
        :class="{ 'underline': currentView === 'mission' }"
        @click="handleClick('mission')"
      >
        {{ breadcrumbMissionName }}
      </BreadcrumbItem>
      <BreadcrumbSeparator class="text-secondary" />
      <BreadcrumbItem 
        class="cursor-pointer text-secondary" 
        :class="{ 'underline': currentView === 'vehicle' }"
        @click="handleClick('vehicle')"
      >
        {{ breadcrumbVehicleName }}
      </BreadcrumbItem>
      <BreadcrumbSeparator class="text-secondary" />
      <BreadcrumbItem 
        class="cursor-pointer text-secondary" 
        :class="{ 'underline': currentView === 'stage' }"
        @click="handleClick('stage')"
      >
        Stages
      </BreadcrumbItem>
    </BreadcrumbList>
  </Breadcrumb>
</template>
