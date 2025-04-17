<template>
  <div class="relative flex h-5 w-fit items-end gap-1"> <!-- Smaller container -->
    <div class="relative flex h-full w-fit flex-grow justify-center gap-[1px]"> <!-- Tighter gap -->
      <div :class="cn('bar h-1/4 bg-background', latency <= 90 && 'bg-foreground')" />
      <div :class="cn('bar h-1/2 bg-background', latency <= 70 && 'bg-foreground')" />
      <div :class="cn('bar h-3/4 bg-background', latency <= 60 && 'bg-foreground')" />
      <div :class="cn('bar h-full bg-background', latency <= 40 && 'bg-foreground')" />
    </div>
    <span v-if="displayLatency" class="flex w-fit flex-none items-end text-xs"> <!-- Smaller text -->
      {{ calculatedLatency }} ms
    </span>
  </div>
</template>

<script setup lang="ts">
import { cn } from "@/lib/utils";
import { ref, watch } from "vue";

const props = defineProps<{
  latency: number;
  displayLatency: boolean;
}>();

const calculatedLatency = ref(0);

watch(
  () => props.latency,
  (newLatency) => {
    calculatedLatency.value = Date.now() - newLatency;
  }
);
</script>

<style scoped>
.bar {
  @apply mt-auto w-[6px] rounded-[1px] border border-foreground; 
  /* Slimmer bars */
}
</style>