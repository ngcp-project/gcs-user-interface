<script lang="ts" setup>
import { defineProps } from "vue";

defineProps<{
  percentage: number;
  charging: boolean;
}>();
</script>

<script lang="ts" setup>
import { defineProps } from "vue";

defineProps<{
  percentage: number;
  charging: boolean;
}>();
</script>

<template>
  <div class="relative flex flex-col h-full w-full items-center">
    <div class="h-[1px] w-2 border-2 border-[#020817]"></div>
    <div class="relative flex h-6 w-4 rounded-[4px] border-[3px] border-[#020817] items-end">
      <!-- <div :class="percentageCSS" :style="{ width: this.percentage + '%' }"></div> -->
      <div
        class="bg-background w-full"
        :style="{ height: Math.max(percentage , 5) + '%' }"
      />
      <div class="absolute flex h-full w-full items-center justify-center">
        <img class="h-4/5 w-full" src="..\..\assets\lightning-icon-png-5.png" />
      </div>
    </div>
  </div>
</template>

<style scoped>
/* .battery_icon {
        position: absolute;
        width: 40%;
        left: 30%;
        top: 15%;
        animation: blinker 1s linear infinite;
        visibility: hidden;
    } */
.dead {
  visibility: visible;
  animation: blinker 1s linear infinite;
}
.charging {
  visibility: visible;
}

@keyframes blinker {
  50% {
    opacity: 0;
  }
}
</style>