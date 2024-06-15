<template>
  <div class="flex w-full items-center justify-between gap-2 text-2xl">
    <div class="flex flex-wrap items-center gap-1 font-semibold">
      {{ vehicleName }}
      <div class="flex items-center gap-1">
        <img :src="getIconPath" class="h-[30px] w-[30px] flex-none" />
        <!-- <img
          v-if="!isInKeepInZone || isInKeepOutZone"
          src="../../assets/warning.png"
          class="h-[30px] w-[30px] flex-none p-1"
        /> -->
        <Icon icon="mdi:warning-circle-outline" class="h-auto w-[30px] text-red-500" />
      </div>
    </div>
    <div class="flex flex-col items-end text-base font-medium">
      <div class="text-sm opacity-80">Status</div>
      {{ vehicleStatus }}
    </div>
  </div>
</template>

<script lang="ts">
// import {isInKeepInZone, isInKeepInZone2, isInKeepOutZone2} from "../../Functions/geofence";
import { Icon } from "@iconify/vue";
export default {
  props: {
    vehicleName: { required: true, type: String },
    vehicleStatus: { required: true, type: String },
    // vehicleIcon: { type: Object },
    isInKeepInZone: { required: true, type: Boolean },
    isInKeepOutZone: { required: true, type: Boolean }
  },
  components: { Icon },
  computed: {
    getIconPath() {
      return new URL("../../assets/" + this.vehicleName + ".png", import.meta.url).href;
    }
  }
};
</script>

<style scoped>
.vehicle-title-container {
  position: relative;
  width: 100%;
  padding-top: 6%;
  padding-left: 6%;
}

.vehicle-name-div {
  position: relative;
  display: flex;
  font-size: 2.3em;
  align-items: center;
  gap: 2%;
}

.vehicle-status-div {
  position: relative;
  font-size: 1.4em;
  margin-top: 3%;
}

.vehicle-icon {
  position: relative;
  width: 16%;
}

.warning-icon {
  position: relative;
  width: 16%;
}
</style>
