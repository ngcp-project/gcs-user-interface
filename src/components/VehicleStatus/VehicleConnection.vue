<template>
  <div class="relative flex h-6 w-fit items-end gap-1">
    <div class="relative flex h-full w-fit flex-grow justify-center gap-[1px]">
      <div :class="cn('bar h-1/4 bg-foreground', signal_string === 0 && 'bg-foreground/50')" />
      <div
        :class="
          cn(
            'bar h-1/2 bg-foreground',
            (signal_string >= 70 || signal_string === 0) && 'bg-foreground/50'
          )
        "
      />
      <div
        :class="
          cn(
            'bar h-3/4 bg-foreground',
            (signal_string >= 60 || signal_string === 0) && 'bg-foreground/50'
          )
        "
      />
      <div
        :class="
          cn(
            'bar h-full bg-foreground',
            (signal_string >= 40 || signal_string === 0) && 'bg-foreground/50'
          )
        "
      />
    </div>
    <span v-if="displayLatency" class="flex w-fit flex-none items-end">
      {{ signal_string.toFixed(2) }} dbm
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
  @apply mt-auto w-[6px] rounded-sm border;
}
</style>
