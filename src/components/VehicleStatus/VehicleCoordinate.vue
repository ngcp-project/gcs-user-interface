<template>
  <div class="flex w-full flex-col">
    <div
      v-for="(info, index) in displayInfo"
      :key="index"
      class="flex w-full items-center justify-between gap-1 font-medium text-secondary-foreground"
    >
      <div class="opacity-80">{{ info.label }}:</div>
      <div>{{ info.value }}</div>
    </div>
    <!-- <div class="coordinate-div">Longitude: {{ coordinates.longitude.toFixed(6) }}</div>
    <div class="coordinate-div">Latitude: {{ coordinates.latitude.toFixed(6) }}</div> -->
  </div>
</template>

<script setup lang="ts">
import { computed, defineProps } from "vue";

const props = defineProps<{
  coordinates: { longitude: number; latitude: number };
}>();

const displayInfo = computed(() => [
  {
    label: "Longitude",
    value: props.coordinates.longitude.toFixed(6)
  },
  {
    label: "Latitude",
    value: props.coordinates.latitude.toFixed(6)
  }
]);
</script>

<style scoped>
.coordinate-container {
  position: relative;
  display: flex;
  flex-direction: column;
  width: 92%;
}

.coordinate-div {
  @apply text-secondary-foreground/80;
}
</style>
