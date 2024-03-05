<!-- Using this to see how the Battery and Connection components look -->

<script setup>
import Battery from './components/VehicleStatus/Battery.vue';
import Connection from './components/VehicleStatus/Connection.vue';
</script>

<template>
  <h2>Below is the Connection component</h2>
  <div class="border_div">
    <Connection :latency=56 />                    <!-- pass in latency into latency prop !-->
  </div>


  <h2>Below is the Battery component</h2>
  <div class="border_div">
    <Battery :percentage=12 :charging="false"/>    <!-- pass in the current percentage into percentage prop. charging is a boolean !-->
  </div>

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
