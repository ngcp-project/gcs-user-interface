<script setup>
import Battery from './components/Battery.vue';

</script>

<template>
  <h2>Below is the Connection component</h2>
  <!-- <div class="bruh">
    <Connection :latency=20 />
  </div> -->


  <h2>Below is the Battery component</h2>
  <div class="border_div">
    <Battery :percentage=10 :charging="false"/>
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
