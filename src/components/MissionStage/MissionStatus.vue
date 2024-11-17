<template>
  <div class="rounded-md bg-foreground px-2 py-1 text-background">
    <div class="flex items-center justify-between gap-4">
      <div>Mission: {{ !!MISSION_INFO.missionName ? MISSION_INFO.missionName : "N/A" }}</div>
      <Button size="sm" variant="secondary" type="button" @click="openStatusWindow">OPEN</Button>
    </div>
    <div>Current Stage: {{ getLastStage() ?? "N/A" }}</div>
  </div>
  <div>
    <div v-if="showPopup" class="popup">
      <button style="float: right" @click="closePopup()">X</button>
      <MissionDropdown :missionNumber="missionNumber" @close="closePopup()" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject } from "vue";
import MissionDropdown from "./MissionDropdown.vue";
import { SearchCoordsProvider } from "@/types/search-coords-provider";
import { TargetCoordsProvider } from "@/types/target-coords.provider";
import { MissionInfoProvider } from "@/types/mission-info-provider";
import Button from "../ui/button/Button.vue";

const showPopup = ref(false);

const { selectingSearch } = inject<SearchCoordsProvider>("search-coords-provider")!;
const { targetCoord, selectingTarget } = inject<TargetCoordsProvider>("target-coords-provider")!;
const { MISSION_INFO, getStageNames } = inject<MissionInfoProvider>("mission-info-provider")!;

const props = defineProps<{
  missionNumber: number;
  status: string;
}>();

const closePopup = () => {
  selectingTarget.value = false;
  selectingSearch.value = false;
  showPopup.value = false;
};

const getLastStage = () => {
  const stage_names = getStageNames();
  return stage_names[stage_names.length - 1];
};

const openStatusWindow = () => {
  showPopup.value = true;
};
</script>

<style scoped>
.container {
  height: 4rem;
  border: 2px solid rgb(52, 49, 49);
  background-color: rgb(255, 255, 255);
  margin-left: auto;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: flex-start;
}
.popup {
  position: fixed;
  top: 70%;
  left: 68%;
  transform: translate(-50%, -50%);
  background: rgba(0, 0, 0, 0.5);
  padding: 20px;
  z-index: 1000;
}
span {
  width: 10em;
  overflow: hidden;
}
</style>
