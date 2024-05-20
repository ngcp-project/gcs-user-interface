<script setup lang="ts">
import { ref, Ref, onMounted } from 'vue';
import Battery from '../components/VehicleStatus/Battery.vue';
import Connection from '../components/VehicleStatus/Connection.vue';
import Camera from "../components/Camera.vue";
import { useRouter } from 'vue-router';
import { getAllConnections } from "../Functions/webSocket";

const router = useRouter();

const handleClick = (cameraID:number, vehicleKey: string) => {
  console.log("camera:" , cameraID);
  // router.push(`/CamFocus/${cameraID}`);}
  router.push(`/CamFocus/${cameraID}/${vehicleKey}`);}

// initialize reactive variables for each vehicle's telemetry data (the object is reactive, so each key/value pair is also reactive)
const ERU_data = ref({batteryPct: 0, connection: 0});
const MEA_data = ref({batteryPct: 0, connection: 0});
const MRA_data = ref({batteryPct: 0, connection: 0});
const FRA_data = ref({batteryPct: 0, connection: 0});

// maps vehicle name to corresponding reactive variable so that addListeners can more easily set EventListeners to update variables
const vehicleMap: { [key: string]: Ref<any> } = {
    'eru': ERU_data,
    'mea': MEA_data,
    'mra': MRA_data,
    'fra': FRA_data
};

let wsConnections: { [key: string]: WebSocket } = {};
// this function runs once (in mounted) and adds event listeners for each vehicle WS connection, so that the reactive variables update whenever new data is received
function addListeners() {
    wsConnections = getAllConnections();        // gets all 4 Websocket connections that were initialized in App.vue (when Vue project first ran)

    for (const [vehicleKey, webSocketConnection] of Object.entries(wsConnections)) {        // loops through each WS connection and adds an event listener to it
        webSocketConnection.addEventListener("message", (event) => {
        const receivedData = JSON.parse(event.data);
 
        vehicleMap[vehicleKey].value.batteryPct = parseFloat(receivedData.batteryLife);
        vehicleMap[vehicleKey].value.connection = parseInt(receivedData.dummyConnection);   
        });
    } // end for loop
} // end addListeners

// gets all 4 websocket connections and adds event listeners to each of them once Static Screen finishes initial rendering
onMounted(() => {
    addListeners();
});
</script>


<template>
    <div class="grid">
        <div class="hover" style="position: relative; display: flex;" @click="handleClick(2, 'eru')">
            <Camera :cameraID="1"/>
            <Battery :percentage=ERU_data.batteryPct :charging="false" class="battery_test"/>
            <Connection :latency=ERU_data.connection :displayLatency="false" class="connection_test"/>   
        </div>

        <div class="hover" style="position: relative; display: flex;" @click="handleClick(1, 'mea')">
            <Camera :cameraID="1"/>
            <Battery :percentage=MEA_data.batteryPct :charging="false" class="battery_test"/>
            <Connection :latency=MEA_data.connection :displayLatency="false" class="connection_test"/>  
        </div>

        <div class="hover" style="position: relative; display: flex;" @click="handleClick(1, 'mra')">
            <Camera :cameraID="1"/>
            <Battery :percentage=MRA_data.batteryPct :charging="false" class="battery_test"/>
            <Connection :latency=MRA_data.connection :displayLatency="false" class="connection_test"/>   
        </div>

        <div class="hover" style="position: relative; display: flex;" @click="handleClick(1, 'fra')">
            <Camera :cameraID="1"/>
            <Battery :percentage=FRA_data.batteryPct :charging="false" class="battery_test"/>
            <Connection :latency=FRA_data.connection :displayLatency="false" class="connection_test"/>   
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