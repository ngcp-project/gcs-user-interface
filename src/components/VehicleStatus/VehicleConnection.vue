<template>
  <div class="relative flex h-7 w-fit items-end gap-1">
    <div class="relative flex h-full w-fit flex-grow justify-center gap-[1.75px]">
      <div :class="cn('bar h-1/4 bg-white', latency <= 90 && 'bg-background')" />
      <div
        :class="
          cn('bar h-1/2 bg-white', (latency <= 70) && 'bg-background')
        "
      />
      <div
        :class="
          cn('bar h-3/4 bg-white', (latency <= 60) && 'bg-background')
        "
      />
      <div
        :class="
          cn('bar h-full bg-white', (latency <= 40) && 'bg-background')
        "
      />
    </div>
    <span v-if="displayLatency" class="flex w-fit flex-none items-end text-xs"> <!-- Smaller text -->
      {{ calculatedLatency }} ms
    </span>
  </div>
</template>

<script setup lang="ts">
import { cn } from "@/lib/utils";
import { Signal } from "lucide-vue-next";
import { ref, watch } from "vue";

const props = defineProps<{
  signal_string: number;
  displayLatency: boolean;
}>();

// const calculatedLatency = ref(0);

// watch(
//   () => props.signal_string,
//   (newLatency) => {
//     calculatedLatency.value = Date.now() - newLatency;
//   }
// );
</script>

<style scoped>
.bar {
  @apply mt-auto w-[9px] rounded-[2px] border-2;
}
</style>