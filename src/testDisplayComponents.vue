<!-- Using this to see how the Battery and Connection components look -->

<script setup>
import Battery from './components/VehicleStatus/Battery.vue';
import Connection from './components/VehicleStatus/Connection.vue';
import Status from "./components/VehicleStatusComponent.vue";
import yuh from "./components/VehicleStatus/VehicleTitle.vue"
import Coordinate from './components/VehicleStatus/Coordinate.vue'


let testObject = {
        longitude: -177.9325790,
        latitude: 33.9325790
    }
</script>

<template>
  <!-- <Status :batteryPct=48 :vehicleName="'ERU'" :vehicleStatus="'Offline'"/> -->

  <Coordinate :coordinates="testObject"></Coordinate>


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

.border_div_2 {
  display: flex;
  border: 0.4em solid black;
  height: 180px;
  width: 380px;
  /* height: 100px;
  width: 200px;  */
}

.additional_battery_prop {
  top: 4%;
  margin-right: 2%;
}

</style>
