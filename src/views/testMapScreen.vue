<!-- <script setup> -->
<script setup>
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
  <NavBar></NavBar>
  <div class="screen_div">
    <!-- Map component will be placed below -->
    <div class="map_div"></div>

    <div class="status_rightside">
        <Status :batteryPct=100 :latency=50 :coordinates="testCoordinateObject2" :vehicleName="'ERU'" :vehicleStatus="'In Use'"/>
        <Status :batteryPct=48 :latency=30 :coordinates="testCoordinateObject" :vehicleName="'MEA'" :vehicleStatus="'Standby'"/>
        <Status :batteryPct=0 :latency=97 :coordinates="testCoordinateObject2" :vehicleName="'MRA'" :vehicleStatus="'Offline'"/>
        <Status :batteryPct=10 :latency=0 :coordinates="testCoordinateObject" :vehicleName="'FRA'" :vehicleStatus="'Offline'"/>

    </div>
  
  </div>



</template>

<style scoped>

.screen_div {
    display: flex;
    border: 0.003em solid black;
    height: 85vh;
    width: 99vw;
}

.status_rightside {
    display: flex;
    flex-direction: column;
    border: 0.003em solid black;
    height: 100%;
    width: 23%;
    margin-left: auto
}

.map_div {
    background-color: pink;
}

</style>
