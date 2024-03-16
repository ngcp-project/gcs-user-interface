<!-- Using this to see how the Battery and Connection components look -->

<script setup>
import Battery from './components/VehicleStatus/Battery.vue';
import Connection from './components/VehicleStatus/Connection.vue';
import Status from "./components/VehicleStatusComponent.vue";
import yuh from "./components/VehicleStatus/VehicleTitle.vue"
</script>

<template>
  <Status :batteryPct=48 :vehicleName="'ERU'" :vehicleStatus="'Offline'"/>
  <!-- <yuh :vehicleName="'ERU'" :vehicleStatus="'Standby'"></yuh> -->

  <!-- <h2>Below is the Connection component</h2>
  <div class="border_div">
    <Connection :latency=100 />                    
  </div> -->


  <!-- <h2>Below is the Battery component</h2>
  <div class="border_div">
    <Battery :percentage=12 :charging="false"/>    pass in the current percentage into percentage prop. charging is a boolean ! -->
  <!-- </div> -->

  <!-- <h2>Below is both in one div</h2>
  <div class="border_div_2">
    <Battery :percentage=12 :charging="false" class="additional_battery_prop"/>    
    <Connection :latency=56 />                    
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
