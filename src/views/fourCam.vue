<script setup lang="ts">
import Battery from '../components/VehicleStatus/Battery.vue';
import Connection from '../components/VehicleStatus/Connection.vue';

import Camera from "../components/Camera.vue";
</script>

<template>
    <div class="grid">
        <div style="position: relative;">
            <Camera :cameraNumber="2"/>
            <div class="status_div">        <!-- the Battery and Connection component's size is dependent on its parent element. So changing 'status_div' size will change their size-->
                <Battery :percentage=89 :charging="false" class="adjust_Battery"/>
                <Connection :latency=65 />   
            </div>
        </div>

        <div style="position: relative;">
            <Camera :cameraNumber="1"/>
            <div class="status_div">
                <Battery :percentage=12 :charging="false" class="adjust_Battery"/>
                <Connection :latency=3 />   
            </div>
        </div>

        <div style="position: relative;">
            <Camera :cameraNumber="1"/>
            <div class="status_div">
                <Battery :percentage=46 :charging="false" class="adjust_Battery"/>
                <Connection :latency=82 />   
            </div>
        </div>

        <div style="position: relative;">
            <Camera :cameraNumber="2"/>
            <div class="status_div">
                <Battery :percentage=0 :charging="false" class="adjust_Battery"/>
                <Connection :latency=5 />   
            </div>
        </div>
    </div>
</template>

<style scoped>
/* currently Battery and Connection's sizes are dependent on the parent element (status_div). If we want their own fixed size, can change 
it directly in their file component's style*/
  .status_div {
    display: flex;
    position: absolute;
    top: 1%;
    left:4%;
    /* border: 0.4em solid black; */
    height: 130px;
    width: 260px;
  }

  .adjust_Battery {
    top: 5%;
    margin-right: 3%; 
  }
</style>