<script setup lang="ts">
import Camera from "../components/Camera.vue";
import Status from '../components/VehicleStatusComponent.vue';
import NavBar from '../components/Navbar.vue';
import { onMounted, ref } from 'vue';

const receivedData = ref<any>(null);
const battery1 = ref(0);
const latitude = ref<number>(7.7);
const longitude = ref<number>(8.8);
// create websocket connection once Static Screen finishes initial rendering
onMounted(() => {
    let client = new WebSocket('ws://localhost:5135/');
    console.log("Connected to server");

    client.addEventListener("message", (event) => {
        // Handle incoming message
        const data = JSON.parse(event.data);
        receivedData.value = data; 
        console.log("Received data:", receivedData);
        battery1.value = receivedData.value.batteryLife;
    });
});

// --- testing with dummy reactive data --- //
//let battery1 = ref(100); 
let battery2 = ref(50);
let connection = ref(100);
let testCoordinate = ref({longitude: 40.748440, latitude: -73.984559})
let testCoordinateObject1 = {
        longitude: -177.9325790,
        latitude: 33.9325790
    }
let testCoordinateObject2 = {
    longitude: 40.748440,
    latitude: -73.984559
}

// --- functions to change reactive data randomly to test if individual vue components re-render ---- //
// var count = 89;
// var counterIncrement = -1;
// const testBattery = setInterval(function() {
//     count = count+counterIncrement;
//     if (count == 0 || count == 100 ) {
//         counterIncrement = -counterIncrement;
//     }
//     battery1.value = count;
//     battery2.value = count;
//     connection.value = count;
//  }, 100);
// const testCoords = setInterval(function() {
//     testCoordinate.value.latitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)).toFixed(6));
//     testCoordinate.value.longitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)).toFixed(6));
// }, 100);
</script>

<template>
  <div class="screen_div">
    <!-- Map component will be placed below -->
    <div class="map_div"></div>
    <!-- <p v-if="receivedData && receivedData.currentPosition">
        Latitude: {{ receivedData.currentPosition.latitude }}, Longitude: {{ receivedData.currentPosition.longitude }}
    </p> -->
    <div class="four-status-rightside">
        <!-- For final product, pass in a Vehicle Object instead that contains all of the information for the VehicleStatusComponent to display-->
        <Status :batteryPct=battery1 :latency=connection :coordinates="testCoordinate" :vehicleName="'ERU'" :vehicleStatus="'In Use'"/>
        <Status :batteryPct=67 :latency=30 :coordinates="testCoordinateObject1" :vehicleName="'MEA'" :vehicleStatus="'Standby'"/>
        <Status :batteryPct=0 :latency=100 :coordinates="testCoordinateObject2" :vehicleName="'MRA'" :vehicleStatus="'Offline'"/>
        <Status :batteryPct=10 :latency=0 :coordinates="testCoordinateObject1" :vehicleName="'FRA'" :vehicleStatus="'Offline'"/>
    </div>
  </div>

    <!-- <div class="camera-container">
       this is static screen
    </div> -->
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
}
</style>