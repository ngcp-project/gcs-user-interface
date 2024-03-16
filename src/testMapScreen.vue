<!-- <script setup> -->
<script setup>
import Status from './components/VehicleStatusComponent.vue';
import NavBar from './components/Navbar.vue';


</script>
<template>
  <NavBar></NavBar>
  <div class="screen_div">
    <div class="map_div"></div>
    <div class="status_rightside">
        <!-- <Status :vehicle-name="'ERU'" />
        <Status :vehicle-name="'MRA'" />
        <Status :vehicle-name="'MEA'" />
        <Status :vehicle-name="'FRA'" /> -->
        <Status :batteryPct=100 :vehicleName="'ERU'" :vehicleStatus="'In Use'"/>
        <Status :batteryPct=48 :vehicleName="'MEA'" :vehicleStatus="'Standby'"/>
        <Status :batteryPct=0 :vehicleName="'MRA'" :vehicleStatus="'Offline'"/>
        <Status :batteryPct=10 :vehicleName="'FRA'" :vehicleStatus="'Offline'"/>

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
