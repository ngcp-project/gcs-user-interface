<script setup lang="ts">
import { ref } from 'vue';
import Battery from '../components/VehicleStatus/Battery.vue';
import Connection from '../components/VehicleStatus/Connection.vue';
import Camera from "../components/Camera.vue";
import { useRouter } from 'vue-router';

const router = useRouter();

const handleClick = (cameraID:number) => {
  console.log("camera:" , cameraID);
  router.push(`/CamFocus/${cameraID}`);}
</script>



<template>
    <div class="grid">
        <div class="hover" style="position: relative; display: flex;" @click="handleClick(2)">
            <Camera :cameraID="2"/>
            <Battery :percentage=.85 :charging="false" class="battery_test"/>
            <Connection :latency=65 class="connection_test"/>   
            <!-- the Battery and Connection component's size is dependent on its parent element. So changing 'status_div' size will change their size-->
            <!-- <div class="status_div">        
                <Battery :percentage=89 :charging="false" class="adjust_Battery"/>
                <Connection :latency=65 />   
            </div> -->
        </div>

        <div class="hover" style="position: relative; display: flex;" @click="handleClick(1)">
            <Camera :cameraID="1"/>
            <Battery :percentage=.12 :charging="false" class="battery_test"/>
            <Connection :latency=3 class="connection_test"/>  
            <!-- <div class="status_div">
                <Battery :percentage=12 :charging="false" class="adjust_Battery"/>
                <Connection :latency=3 />   
            </div> -->
        </div>

        <div class="hover" style="position: relative; display: flex;" @click="handleClick(1)">
            <Camera :cameraID="1"/>
            <Battery :percentage=.46 :charging="false" class="battery_test"/>
            <Connection :latency=82 class="connection_test"/>   
            <!-- <div class="status_div">
                <Battery :percentage=46 :charging="false" class="adjust_Battery"/>
                <Connection :latency=82 />   
            </div> -->
        </div>

        <div class="hover" style="position: relative; display: flex;" @click="handleClick(1)">
            <Camera :cameraID="1"/>
            <Battery :percentage=0 :charging="false" class="battery_test"/>
            <Connection :latency=5 class="connection_test"/>   
            <!-- <div class="status_div">
                <Battery :percentage=0 :charging="false" class="adjust_Battery"/>
                <Connection :latency=5 />   
            </div> -->
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

  .battery_test {
    position:absolute; 
    height: 7%;
    width: 6.3%;
    top: 4%; 
    left: 2%;
  }

  .connection_test {
    position:absolute; 
    height: 11%;
    width: 4%; 
    top: 1%;
    left: 9.5%;
  }

  .hover{
    cursor: pointer;
  }
</style>