<!-- Using this to see how the our components look -->

<script setup>
import Coordinate from './components/VehicleStatus/Coordinate.vue';
import Altitude from './components/FlightComponents/Altitude.vue';
import Altimeter from './components/FlightComponents/Altimeter.vue'
import Airspeed from './components/FlightComponents/Airspeed.vue'

let testObject = {
        longitude: -177.9325790,
        latitude: 33.9325790
    }
</script>

<template>
  <!-- <Status :batteryPct=48 :vehicleName="'ERU'" :vehicleStatus="'Offline'"/> -->

  <Altitude :pitch=2 :roll=10></Altitude>
  <Altimeter :altitude=200></Altimeter>
  <Airspeed :airspeed=60></Airspeed>

</template>

<style scoped>

.border_div {
  /* display: flex; */
  border: 0.4em solid black;
  height: 200px;
  width: 400px;
  /* height: 100px;
  width: 200px;  */
}


</style>
