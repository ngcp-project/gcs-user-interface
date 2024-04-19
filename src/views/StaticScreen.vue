<script setup lang="ts">
import Camera from "../components/Camera.vue";
import Status from '../components/VehicleStatusComponent.vue';
import NavBar from '../components/Navbar.vue';

let testCoordinateObject = {
        longitude: -177.9325790,
        latitude: 33.9325790
    }
let testCoordinateObject2 = {
    longitude: 40.748440,
    latitude: -73.984559
}
</script>

<template>
  <div class="screen_div">
    <!-- Map component will be placed below -->
    <div class="map_div"></div>

    <div class="four-status-rightside">
        <!-- For final product, pass in a Vehicle Object instead that contains all of the information for the VehicleStatusComponent to display-->
        <Status :batteryPct=100 :latency=50 :coordinates="testCoordinateObject2" :vehicleName="'ERU'" :vehicleStatus="'In Use'"/>
        <Status :batteryPct=48 :latency=30 :coordinates="testCoordinateObject" :vehicleName="'MEA'" :vehicleStatus="'Standby'"/>
        <Status :batteryPct=0 :latency=100 :coordinates="testCoordinateObject2" :vehicleName="'MRA'" :vehicleStatus="'Offline'"/>
        <Status :batteryPct=10 :latency=0 :coordinates="testCoordinateObject" :vehicleName="'FRA'" :vehicleStatus="'Offline'"/>
    </div>
  </div>

    <!-- <div class="camera-container">
       this is static screen
    </div> -->
</template>
  
<style scoped>
.camera-container {
    height: 90vh; /* Set the height of the container to 100% of the viewport height */
    width: 75%;
    display: flex; /* Use flexbox to align items vertically */
    justify-content: center; /* Center the child element horizontally */
    align-items: center; /* Center the child element vertically */
}

.screen_div {
    display: flex;
    border: 0.003em solid black;
    height: 89vh;
    width: 99vw;
}

.four-status-rightside {
    display: flex;
    flex-direction: column;
    border: 0.003em solid black;
    height: 100%;
    width: 23%;
    margin-left: auto
}

.map_div {
    width: 77%;
}
</style>