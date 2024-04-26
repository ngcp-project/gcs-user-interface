<script setup lang="ts">
import Status from '../components/VehicleStatusComponent.vue';
import { onMounted, onBeforeUnmount, ref, reactive, Ref } from 'vue';
import Map from '../components/Map.vue';
// import { getAllConnections, closeConnections } from "../Functions/webSocket";
import { getConnection } from "../Functions/webSocket";

// initialize reactive variables for each vehicle's telemetry data (the object is reactive, so each key/value pair is also reactive)
const ERU_data = ref({batteryPct: 0, connection: 0, coordinates: {longitude: 0, latitude: 0}, status: 'Standby'});
const MEA_data = ref({batteryPct: 0, connection: 0, coordinates: {longitude: 0, latitude: 0}, status: 'Standby'});
const MRA_data = ref({batteryPct: 0, connection: 0, coordinates: {longitude: 0, latitude: 0}, status: 'Standby'});
const FRA_data = ref({batteryPct: 0, connection: 0, coordinates: {longitude: 0, latitude: 0}, status: 'Standby'});

// maps vehicle name to corresponding reactive variable so that addListeners can more easily set EventListeners to update variables
const vehicleMap: { [key: string]: Ref<any> } = {
    'eru': ERU_data,
    'mea': MEA_data,
    'mra': MRA_data,
    'fra': FRA_data
};

// let wsConnections: { [key: string]: WebSocket } = {};
let wsConnection: WebSocket;
// this function runs once (in mounted) and adds event listeners for each vehicle WS connection, so that the reactive variables update whenever new data is received
function addListeners() {
    wsConnection = getConnection();
    wsConnection.addEventListener("message", (event) => {
    const receivedData = JSON.parse(event.data);

    ERU_data.value.status = receivedData.vehicleStatus;   
    ERU_data.value.batteryPct = parseFloat(receivedData.batteryLife);
    ERU_data.value.coordinates.latitude = parseFloat(receivedData.currentPosition.latitude);
    ERU_data.value.coordinates.longitude = parseFloat(receivedData.currentPosition.longitude);
    ERU_data.value.connection = parseInt(receivedData.dummyConnection);   
    });

    // wsConnections = getAllConnections();        // gets all 4 Websocket connections that were initialized in App.vue (when Vue project first ran)

    // for (const [vehicleKey, webSocketConnection] of Object.entries(wsConnections)) {        // loops through each WS connection and adds an event listener to it
    //     webSocketConnection.addEventListener("message", (event) => {
    //     const receivedData = JSON.parse(event.data);
 
    //     vehicleMap[vehicleKey].value.status = receivedData.vehicleStatus;   
    //     vehicleMap[vehicleKey].value.batteryPct = parseFloat(receivedData.batteryLife);
    //     vehicleMap[vehicleKey].value.coordinates.latitude = parseFloat(receivedData.currentPosition.latitude);
    //     vehicleMap[vehicleKey].value.coordinates.longitude = parseFloat(receivedData.currentPosition.longitude);
    //     vehicleMap[vehicleKey].value.connection = parseInt(receivedData.dummyConnection);   
    //     });
    // } // end for loop
} // end addListeners

// gets all 4 websocket connections and adds event listeners to each of them once Static Screen finishes initial rendering
onMounted(() => {
    addListeners();
});
</script>

<template>
  <div class="screen_div">
    <div class="map_div">
        <Map></Map>
    </div>

    <div class="four-status-rightside">     
        <Status :batteryPct=ERU_data.batteryPct :latency=ERU_data.connection :coordinates=ERU_data.coordinates :vehicleName="'ERU'" :vehicleStatus="ERU_data.status"/>
        <Status :batteryPct=MEA_data.batteryPct :latency=MEA_data.connection :coordinates=MEA_data.coordinates :vehicleName="'MEA'" :vehicleStatus="MEA_data.status"/>
        <Status :batteryPct=MRA_data.batteryPct :latency=MRA_data.connection :coordinates=MRA_data.coordinates :vehicleName="'MRA'" :vehicleStatus="MRA_data.status"/>
        <Status :batteryPct=FRA_data.batteryPct :latency=FRA_data.connection :coordinates=FRA_data.coordinates :vehicleName="'FRA'" :vehicleStatus="FRA_data.status"/>
    </div>
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
    border: 0.003em solid black;
    height: 100%;
    width: 23%;
    margin-left: auto
}

.map_div {
    width: 77%;
}
</style>