<script setup lang="ts">
import Camera from "../components/CameraFeed.vue";
import IndicatorComponent from "../components/IndicatorComponent.vue";
import Status from "../components/VehicleStatusComponent.vue";
import { useRoute } from "vue-router";
import { onMounted, ref } from "vue";
import { getConnection, getVehicleStatus } from "../Functions/webSocket";

const route = useRoute();
const cameraID = Number(route.params.id); // Assuming we're using camera Number
const vehicleID = String(route.params.vehicleID);

const vehicleData = ref<any>(null);

const wsConnection = getConnection(vehicleID); // get websocket connection for this specific vehicle
console.log("Got connection for " + vehicleID + " | " + typeof wsConnection);
wsConnection.addEventListener("message", (event) => {
  vehicleData.value = JSON.parse(event.data);
});
</script>

<template>
  <div class="cam-focus-screen-div">
    <div class="camera-container">
      <div class="camera-wrapper">
        <router-link to="/" class="back">Back</router-link>
        <Camera :cameraID="cameraID" />
        <!-- Back button -->
      </div>
    </div>

    <div class="vehicle-info-container">
      <div class="vehicle-status-parent">
        <Status
          :batteryPct="parseFloat(vehicleData.batteryLife)"
          :latency="parseFloat(vehicleData.lastUpdated)"
          :coordinates="vehicleData.currentPosition"
          :vehicleName="vehicleID.toUpperCase()"
          :vehicleStatus="getVehicleStatus(vehicleData.vehicleStatus)"
          :isInKeepInZone="true"
          :isInKeepOutZone="false"
        />
      </div>

      <IndicatorComponent
        class="adjust-indicator"
        :vehicleName="vehicleID.toUpperCase()"
        :pitch="parseInt(vehicleData.pitch)"
        :roll="parseInt(vehicleData.roll)"
        :altitude="parseInt(vehicleData.altitude)"
        :airspeed="parseInt(vehicleData.speed)"
        :yaw="parseInt(vehicleData.yaw)"
      ></IndicatorComponent>
    </div>
  </div>
</template>

<style scoped>
.cam-focus-screen-div {
  display: flex;
  height: 90vh;
  width: 100%;
}

.camera-container {
  /* height: 90vh;  */
  height: 100%;
  width: 77%;
  display: flex; /* Use flexbox to align items vertically */
  justify-content: center; /* Center the child element horizontally */
  align-items: center; /* Center the child element vertically */
}

.camera-wrapper {
  position: relative; /* For positioning the button relative to the camera */
  height: 100%;
  width: 100%;
}

.back {
  position: absolute; /* Position the button relative to the container */
  top: 10px;
  left: 10px;
  padding: 5px 10px;
  border: none;
  background-color: lightgray;
  color: black;
  cursor: pointer;
}

.vehicle-info-container {
  display: flex;
  flex-direction: column;
  gap: 1%;
  height: 100%;
  width: 23%;
}

.adjust-indicator {
  margin-top: auto;
  margin-bottom: 1%;
}
.vehicle-status-parent {
  height: 15em;
  /* padding-top: 4.5em; */
  margin-bottom: auto;
}
</style>
