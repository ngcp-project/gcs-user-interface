<script setup lang="ts">
import Status from '../components/VehicleStatusComponent.vue';
import { onMounted, onBeforeUnmount, ref, reactive, Ref } from 'vue';
import Map from '../components/Map.vue';
import { getAllConnections, closeConnections, getVehicleStatus } from "../Functions/webSocket";

// initialize reactive variables for each vehicle's telemetry data (the object is reactive, so each key/value pair is also reactive)
const ERU_data = ref({batteryPct: 0, lastUpdated: 0, coordinates: {longitude: 0, latitude: 0}, status: 'Standby', yaw: 0, inKeepIn: false, inKeepOut: false});
const MEA_data = ref({batteryPct: 0, lastUpdated: 0, coordinates: {longitude: 0, latitude: 0}, status: 'Standby', yaw: 0, inKeepIn: false, inKeepOut: false});
const MRA_data = ref({batteryPct: 0, lastUpdated: 0, coordinates: {longitude: 0, latitude: 0}, status: 'Standby', yaw: 0, inKeepIn: false, inKeepOut: false});
const FRA_data = ref({batteryPct: 0, lastUpdated: 0, coordinates: {longitude: 0, latitude: 0}, fire_coordinates: {longitude: 0, latitude: 0}, status: 'Standby', yaw: 0, inKeepIn: false, inKeepOut: false});

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
 
        vehicleMap[vehicleKey].value.status = getVehicleStatus(receivedData.vehicleStatus);   
        vehicleMap[vehicleKey].value.batteryPct = parseFloat(receivedData.batteryLife);
        vehicleMap[vehicleKey].value.coordinates.latitude = parseFloat(receivedData.currentPosition.latitude);
        vehicleMap[vehicleKey].value.coordinates.longitude = parseFloat(receivedData.currentPosition.longitude);
        // vehicleMap[vehicleKey].value.lastUpdated = parseInt(receivedData.dummyConnection);   // <-- uncomment to use dummyConnection value from mockWebsock.cjs 
        vehicleMap[vehicleKey].value.lastUpdated = parseInt(receivedData.lastUpdated);  
        vehicleMap[vehicleKey].value.yaw = parseInt(receivedData.yaw); 

        // FRA is sending additional fire coordinates
        if (vehicleKey == 'fra') {
            vehicleMap[vehicleKey].value.fire_coordinates.latitude = parseFloat(receivedData.fireCoordinates.latitude);
            vehicleMap[vehicleKey].value.fire_coordinates.longitude = parseFloat(receivedData.fireCoordinates.longitude);   
        }

        });
    } // end for loop
} // end addListeners

// gets all 4 websocket connections and adds event listeners to each of them once Static Screen finishes initial rendering
onMounted(() => {
    addListeners();
    // setInterval(() => {
    //     console.log("ERU KEEP IN: " + ERU_data.value.inKeepIn);
    //     console.log("ERU KEEP IN: " + ERU_data.value.inKeepIn);
    //     console.log("ERU KEEP IN: " + ERU_data.value.inKeepIn);
    //     console.log("ERU KEEP IN: " + ERU_data.value.inKeepIn);
    //     console.log("ERU KEEP IN: " + ERU_data.value.inKeepIn);
    // }, 1000);
});

function updateIsInKeepIn(vehicleKey: string, isInZone: boolean) {
    vehicleMap[vehicleKey.toLowerCase()].value.inKeepIn = isInZone;
}

function updateIsInKeepOut(vehicleKey: string, isInZone: boolean) {
    vehicleMap[vehicleKey.toLowerCase()].value.inKeepOut = isInZone;
}

</script>

<template>
  <div class="screen_div">
    <div class="map_div">
        <!-- should be fire coords -->
        <Map :ERU_coords=ERU_data.coordinates :ERU_yaw="ERU_data.yaw"
             :MEA_coords=MEA_data.coordinates :MEA_yaw="MEA_data.yaw"
             :MRA_coords=MRA_data.coordinates :MRA_yaw="MRA_data.yaw"
             :FRA_coords=FRA_data.coordinates :firePoint=FRA_data.fire_coordinates :FRA_yaw="FRA_data.yaw"
             @keepIn="updateIsInKeepIn" @keepOut="updateIsInKeepOut"></Map>
    </div>

    <div class="four-status-rightside">     
        <Status :batteryPct=ERU_data.batteryPct :latency=ERU_data.lastUpdated :coordinates=ERU_data.coordinates :vehicleName="'ERU'" :vehicleStatus="ERU_data.status" :isInKeepInZone="ERU_data.inKeepIn" :isInKeepOutZone="ERU_data.inKeepOut"/>
        <Status :batteryPct=MEA_data.batteryPct :latency=MEA_data.lastUpdated :coordinates=MEA_data.coordinates :vehicleName="'MEA'" :vehicleStatus="MEA_data.status" :isInKeepInZone="MEA_data.inKeepIn" :isInKeepOutZone="MEA_data.inKeepOut"/>
        <Status :batteryPct=MRA_data.batteryPct :latency=MRA_data.lastUpdated :coordinates=MRA_data.coordinates :vehicleName="'MRA'" :vehicleStatus="MRA_data.status" :isInKeepInZone="MRA_data.inKeepIn" :isInKeepOutZone="MRA_data.inKeepOut"/>
        <Status :batteryPct=FRA_data.batteryPct :latency=FRA_data.lastUpdated :coordinates=FRA_data.coordinates :vehicleName="'FRA'" :vehicleStatus="FRA_data.status" :isInKeepInZone="FRA_data.inKeepIn" :isInKeepOutZone="FRA_data.inKeepOut"/>
    </div>

    <!-- <div @keepIn="receiveKeepInEmit"></div> -->
  </div>

</template>
  
<style scoped>
.camera-container {
    height: 90vh; /* Set the height of the container to 100% of the viewport height */
    width: 75%;
    display: flex; /* Use flexbox to align items vertically */
    justify-content: center; /* Center the child element horizontally */
    align-items: center; /* Center the child element vertically */
}

.screen_div {
    display: flex;
    border: 0.003em solid black;
    height: 89vh;
    width: 99vw;
}

.four-status-rightside {
    display: flex;
    flex-direction: column;
    border-left: 0.003em solid black;
    height: 100%;
    width: 23%;
    margin-left: auto
}

.map_div {
    width: 77%;
}
</style>