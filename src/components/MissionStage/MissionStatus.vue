<template>
  <div
    style="
      width: 12rem;
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
      <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0)">Mission {{ missionNumber }}</span>
      <button
        style="border: 2px solid rgb(0, 0, 0); margin-left: 1.2rem; color: rgb(0, 0, 0); padding: 3px 6px; font-size: 0.8rem"
        type="button"
        @click="showPopup = true"
      >
        <span style="font-weight: bold; font-size: 0.8rem" :style="{ color: status === 'initiated' ? 'black' : 'blue' }">OPEN</span>
      </button>
    </div>
    <div>
      <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0)">Status: {{ status }}</span>
    </div>
  </div>
  <div>
    <div v-if="showPopup" class="popup">
      <button style="float: right" @click="closePopup()">X</button>
      <MissionDropdown :missionNumber="missionNumber" />
    </div>
  </div>
</template>

<script>
import MissionDropdown from "./MissionDropdown.vue";
import { inject } from "vue";
export default {
  setup() {
    const { selectingTarget } = inject('TargetCoord');

    return { selectingTarget };
  },

  name: "MissionStatus",
  data() {
    return {
      showPopup: false,
    };
  },
  components: {
    MissionDropdown,
  },
  props: {
    missionNumber: {
      type: Number,
      required: true,
    },
    status: {
      type: String,
      required: true,
    },
  },
  methods: {
    // make sure to set selectingTarget back to false
    closePopup() {
      this.selectingTarget = false;
      this.showPopup = false;
    }
  }
};
</script>
<style scoped>
.popup {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: rgba(0, 0, 0, 0.5);
  padding: 20px;
  z-index: 1000;
}
</style>
