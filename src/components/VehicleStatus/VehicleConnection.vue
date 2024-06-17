<template>
  <div class="relative flex h-full w-full items-end gap-1">
    <div class="relative flex h-full w-full flex-grow justify-center gap-[1px]">
      <div v-if="latency == 0" class="grayed_bar" style="height: 25%"></div>
      <div v-else class="bar" style="height: 25%"></div>

      <div v-if="latency >= 70 || latency == 0" class="grayed_bar" style="height: 50%"></div>
      <div v-else class="bar" style="height: 50%"></div>

      <div v-if="latency >= 60 || latency == 0" class="grayed_bar" style="height: 75%"></div>
      <div v-else class="bar" style="height: 75%"></div>

      <div v-if="latency >= 40 || latency == 0" class="grayed_bar" style="height: 100%"></div>
      <div v-else class="bar" style="height: 100%"></div>
    </div>
    <span v-if="displayLatency" class="flex w-fit flex-none items-end">
      {{ calculatedLatency }} ms</span
    >
  </div>
</template>

<script lang="ts">
export default {
  data() {
    return {
      calculatedLatency: 0
    };
  },
  props: {
    latency: { required: true, type: Number },
    displayLatency: { required: true, type: Boolean } // true - if u want to display latency value | i set it to false for the 4 camera screen
  },
  watch: {
    // whenever latency prop is updated, calculate the actual latency
    latency: function () {
      this.calculatedLatency = Date.now() - this.latency;
    }
  }
};
</script>

<style scoped>
.outer_div {
  display: flex;
  position: relative;
  height: 25%;
  width: 10%;
}
.connection-container {
  position: relative;
  justify-content: center;
  display: flex;
  gap: 0.05em;
  height: 100%;
  width: 100%;
  border-radius: 12%;
  /* background-color: white; */
}
.bar {
  width: 100%;
  background-color: white;
  border: 0.1em solid black;
  margin-top: auto;
  border-radius: 20%;
}
.grayed_bar {
  width: 100%;
  background-color: rgb(136, 135, 135);
  border: 0.1em solid black;
  margin-top: auto;
  opacity: 0.2;
  border-radius: 20%;
}
.connection_number {
  position: absolute;
  left: 110%;
  bottom: 0%;
  width: 180%;
  /* font-size:0.8em; */
  font-size: 1em;
}
</style>
