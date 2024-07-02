<template>
  <div
    style="
      /* width: 12rem; */
      height: 4rem;
      border: 2px solid rgb(52, 49, 49);
      background-color: rgb(255, 255, 255);
      margin-left: auto;
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: flex-start;
    "
  >
    <div>
      <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0)"
        >Mission: {{ MISSION_INFO["missionName"] }}</span
      >
      <button
        style="
          border: 2px solid rgb(0, 0, 0);
          margin-left: 1.2rem;
          color: rgb(0, 0, 0);
          padding: 3px 6px;
          font-size: 0.8rem;
        "
        type="button"
        @click="showPopup = true"
      >
        <span
          style="font-weight: bold; font-size: 0.8rem"
          :style="{ color: status === 'initiated' ? 'black' : 'blue' }"
          >OPEN</span
        >
      </button>
    </div>
    <div>
      <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0)"
        >Current Stage: {{ this.getLastStage() }}</span
      >
    </div>
  </div>
  <div>
    <div v-if="showPopup" class="popup">
      <button style="float: right" @click="closePopup()">X</button>
      <MissionDropdown :missionNumber="missionNumber" @close="closePopup()" />
    </div>
  </div>
</template>

<script lang="ts">
import MissionDropdown from "./MissionDropdown.vue";
import { inject } from "vue";
import { SearchCoordsProvider } from "@/types/search-coords-provider";
import { TargetCoordsProvider } from "@/types/target-coords.provider";
import { MissionInfoProvider } from "@/types/mission-info-provider";

export default {
  setup() {
    const { searchCoords, selectingSearch, updateSearchCoords } =
      inject<SearchCoordsProvider>("search-coords-provider")!;
    const { targetCoord, selectingTarget } =
      inject<TargetCoordsProvider>("target-coords-provider")!;
    const { MISSION_INFO, getStageNames } = inject<MissionInfoProvider>("mission-info-provider")!;

    return { selectingTarget, selectingSearch, MISSION_INFO, getStageNames };
  },

  name: "MissionStatus",
  data() {
    return {
      showPopup: false
    };
  },
  components: {
    MissionDropdown
  },
  props: {
    missionNumber: {
      type: Number,
      required: true
    },
    status: {
      type: String,
      required: true
    }
  },
  methods: {
    // make sure to set selectingTarget back to false
    closePopup() {
      this.selectingTarget = false;
      this.selectingSearch = false;
      this.showPopup = false;
    },
    getLastStage() {
      // console.log(this.MISSION_INFO['stages'][this.MISSION_INFO['stages'].length - 1]);
      const stage_names = this.getStageNames();

      return stage_names[stage_names.length - 1];
    }
  }
};
</script>
<style scoped>
.popup {
  position: fixed;
  top: 70%;
  left: 68%;
  transform: translate(-50%, -50%);
  background: rgba(0, 0, 0, 0.5);
  padding: 20px;
  z-index: 1000;
}
span {
  width: 10em;
  overflow: hidden;
}
</style>
