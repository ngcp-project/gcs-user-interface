<!-- Using this to see how the our components look -->

<script setup>
import Coordinate from './components/VehicleStatus/Coordinate.vue';
import Altitude from './components/FlightComponents/Altitude.vue';
import Altimeter from './components/FlightComponents/Altimeter.vue';
import Airspeed from './components/FlightComponents/Airspeed.vue';
import IndicatorComponent from './components/IndicatorComponent.vue';

let testObject = {
        longitude: -177.9325790,
        latitude: 33.9325790
    }
</script>

<template>
  <div class="testing">
    <IndicatorComponent></IndicatorComponent>
  </div>

  <!-- <div class="altitude-outer-div">
    <Altitude :pitch=2 :roll=10></Altitude>
  </div>

  <div class="altimeter-outer-div">
    <Altimeter :altitude=200></Altimeter>
  </div>

  <div class="altitude-outer-div">
    <Airspeed :airspeed=100></Airspeed>
  </div> -->
  

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

.altitude-outer-div {
  height: 50;
  width: 10%;
}

.altimeter-outer-div {
  height: 50;
  width: 10%;
}

.testing {
  width: 50%;
  height: 50%;
}

</style>
