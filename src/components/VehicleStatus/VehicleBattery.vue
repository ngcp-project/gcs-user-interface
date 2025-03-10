<script lang="ts" setup>
import { defineProps } from "vue";

defineProps<{
  percentage: number;
  charging: boolean;
}>();
</script>

<template>
  <div class="relative flex flex-col h-full w-full items-center ">
    <div class="h-[1px] w-2 border-2 "></div>
    <div class="relative flex h-6 w-4 rounded-[4px] border-[3px] bg-white">
      <!-- <div :class="percentageCSS" :style="{ width: this.percentage + '%' }"></div> -->
      <div
        :class="percentageCSS"
        :style="{ width: Math.max(percentage * 100, 5) + '%' }"
        >
        <div class="absolute flex h-full w-full items-center justify-center">
          <img class="h-4/5 w-full" src="..\..\assets\lightning-icon-png-5.png" />
        </div>
    </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, defineProps } from "vue";

const props = defineProps<{
  percentage: number;
  charging: boolean;
}>();

const percentageCSS = computed(() => {
  if (props.percentage <= 0) {
    return "zeroPercent";
  } else if (props.percentage > 0 && props.percentage <= 0.15) {
    return "tenPercent";
  } else if (props.percentage > 0.15 && props.percentage <= 0.3) {
    return "twentyFivePercent";
  } else if (props.percentage > 0.3 && props.percentage <= 0.5) {
    return "fiftyPercent";
  } else {
    return "normalPercent";
  }
});

</script>

<style scoped>
/* Animation styles (unchanged) */
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

.battery_container {
  position: relative;
  display: flex;
  border: 0.1em solid black;
  height: 100%;
  width: 100%;
  border-radius: 12%;
  background-color: white;
}

#battery_progress {
  background-color: rgb(83, 255, 83);
  border-radius: 12%;
  height: 100%;
  width: 100%;
}

.zeroPercent {
  width: 0%;
}
.tenPercent {
  background-color: red;
  border-radius: 12%;
  height: 100%;
}
.twentyFivePercent {
  background-color: rgb(116, 115, 109);
  border-radius: 12%;
  height: 100%;
}
.fiftyPercent {
  background-color: rgb(245, 225, 44);
  border-radius: 12%;
  height: 100%;
}
.normalPercent {
  background-color: rgb(2, 8, 23);
  border-radius: 12%;
}
</style>