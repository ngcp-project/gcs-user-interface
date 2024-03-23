<!-- <script setup> -->
<script setup>
import Status from '../components/VehicleStatusComponent.vue';
import NavBar from '../components/Navbar.vue';

let testCoordinateObject = {
        longitude: -177.9325790,
        latitude: 33.9325790
    }

</script>
<template>
  <NavBar></NavBar>
  <div class="screen_div">
    <div class="map_div"></div>
    <div class="status_rightside">
        <Status :batteryPct=100 :latency=54 :coordinates="testCoordinateObject" :vehicleName="'ERU'" :vehicleStatus="'In Use'"/>
        <Status :batteryPct=48 :latency=30 :coordinates="testCoordinateObject" :vehicleName="'MEA'" :vehicleStatus="'Standby'"/>
        <Status :batteryPct=0 :latency=97 :coordinates="testCoordinateObject" :vehicleName="'MRA'" :vehicleStatus="'Offline'"/>
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
    width: 25%;
    margin-left: auto
}

.map_div {
    background-color: pink;
}

</style>
