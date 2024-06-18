<template>
  <div class="relative flex h-6 w-fit items-end gap-1">
    <div class="relative flex h-full w-fit flex-grow justify-center gap-[1px]">
      <div :class="cn('bar h-1/4 bg-foreground', latency === 0 && 'bg-foreground/50')" />
      <div
        :class="
          cn('bar h-1/2 bg-foreground', (latency >= 70 || latency === 0) && 'bg-foreground/50')
        "
      />
      <div
        :class="
          cn('bar h-3/4 bg-foreground', (latency >= 60 || latency === 0) && 'bg-foreground/50')
        "
      />
      <div
        :class="
          cn('bar h-full bg-foreground', (latency >= 40 || latency === 0) && 'bg-foreground/50')
        "
      />
    </div>
    <span v-if="displayLatency" class="flex w-fit flex-none items-end">
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
  @apply mt-auto w-[6px] rounded-sm border;
}
</style>
