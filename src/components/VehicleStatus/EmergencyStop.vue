<template>
  <NgButton variant="destructive" size="xs" @click="showEmergencyModal">EMERGENCY STOP</NgButton>
  <EmergencyStopModal
    :vehicle-name="vehicleName"
    v-show="showModal"
    @close="closeEmergencyModal"
  ></EmergencyStopModal>
</template>

<script lang="ts">
import { NgButton } from "../ui/button";
import EmergencyStopModal from "./EmergencyStopModal.vue";

export default {
  components: {
    EmergencyStopModal,
    NgButton
  },
  data() {
    return {
      showModal: false
    };
  },
  props: {
    vehicleName: { required: true, type: String }
  },
  methods: {
    showEmergencyModal() {
      this.showModal = true;
    },
    closeEmergencyModal() {
      this.showModal = false;
    }
  }
};
</script>

<style scoped>
.emergency-stop-div {
  position: relative;
  display: flex;
  height: 20%;
  width: 78%;
  text-align: center;
}

.emergency-button {
  position: relative;
  display: inline-block;
  width: 100%;
  height: 100%;
  background-color: rgb(255, 57, 57);
  font-size: 0.96em;
  transition: background-color 0.2s;
  white-space: nowrap;
  overflow: hidden;
}

.emergency-button:hover {
  background-color: rgb(192, 40, 40);
}
</style>
