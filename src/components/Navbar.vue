<script lang="ts">
import MissionStatus from "./MissionStage/MissionStatus.vue";
import EmergencyStopModal from '../components/VehicleStatus/EmergencyStopModal.vue';
export default {
  components: {
    EmergencyStopModal,
    MissionStatus
  },
  data() {
    return {
      isNavbarOpen: false,
      misson_one_status: "Done",
      misson_two_status: "In Progress",
      misson_three_status: "initiated",
      showModal: false,   // track whether to display EmergencyStopModal
      stop_all: "all"     // send to EmergencyStopModal to indicate we want to stop all vehicles
    };
  },
  methods: {
    toggleNavbar() {
      this.isNavbarOpen = !this.isNavbarOpen;
    },
    showEmergencyModal() {
      this.showModal = true;
    },
    closeEmergencyModal() {
      this.showModal = false;
    }
  },
};
</script>

<template>
  <nav style="background-color: #011949; padding: 10px">
    <div style="display: flex; align-items: center">
      <router-link to="/" style="text-decoration: none">
        <span style="font-weight: bold; font-size: 1.2rem; margin-left: 10px">NG</span>
        <span style="font-weight: bold; font-size: 1.2rem; color: white">CP</span>
      </router-link>
      <MissionStatus :missionNumber="1" :status="misson_one_status" />
      <MissionStatus :missionNumber="2" :status="misson_two_status" />
      <MissionStatus :missionNumber="3" :status="misson_three_status" />
      <button
        style="border: 2px solid rgb(255, 0, 0); background-color: rgba(255, 0, 0); margin-left: auto; color: rgb(255, 255, 255)" type="button" @click="showEmergencyModal">
        <span style="font-size: 18px">STOP ALL</span>
      </button>
      <EmergencyStopModal :vehicle-name="stop_all" v-show="showModal" @close="closeEmergencyModal"></EmergencyStopModal>
      <button style="border: 2px solid rgb(52, 49, 49); background-color: rgba(38, 36, 36, 0.25); margin-left: auto; color: rgb(52, 49, 49)" type="button" @click="toggleNavbar">
        <span style="font-size: 18px">&#9776;</span>
      </button>
    </div>
    <div v-if="isNavbarOpen" style="margin-top: 10px">
      <ul style="list-style-type: none; padding-left: 0">
        <li><router-link to="/" style="text-decoration: none">Home</router-link></li>
        <li><router-link to="/StaticScreen" style="text-decoration: none">Map Screen</router-link></li>
        <li><router-link to="/MissionInitialization" style="text-decoration: none">Mission Initialization</router-link></li>
        <li><router-link to="/test" style="text-decoration: none">Test</router-link></li>
      </ul>
    </div>
  </nav>
</template>
