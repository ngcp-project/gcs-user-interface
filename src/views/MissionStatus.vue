<script setup lang="ts">
import MissionDropdown from "@/components/MissionStage/MissionDropdown.vue";
import { useRoute } from "vue-router";

const route = useRoute();
const missionNumber = route.params.id;
</script>

<template>
  <div class="flex w-full flex-col items-center p-4">
    <MissionDropdown :missionNumber="Number(missionNumber)" />
  </div>
</template>
