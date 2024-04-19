<script setup lang="ts">
import Status from '../components/VehicleStatusComponent.vue';
import NavBar from '../components/Navbar.vue';
import Camera from "../components/Camera.vue";
import { useRoute } from 'vue-router';
import { onMounted, ref } from 'vue';
const route = useRoute();
const cameraID = Number(route.params.id); // Assuming we're using camera Number 

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
</script>

<template>
    <div class="camera-container">
        <div class="camera-wrapper">
            <router-link to="/" class="back">Back</router-link>
            <Camera :cameraID="cameraID" />
            <!-- Back button -->
        </div>
        <div class="VehicleComponents">
            <Status :batteryPct=battery1 :latency=connection :coordinates="testCoordinate" :vehicleName="'ERU'" :vehicleStatus="'In Use'"/>
            <Status :batteryPct=battery1 :latency=connection :coordinates="testCoordinate" :vehicleName="'ERU'" :vehicleStatus="'In Use'"/>
            <Status :batteryPct=battery1 :latency=connection :coordinates="testCoordinate" :vehicleName="'ERU'" :vehicleStatus="'In Use'"/>
            <Status :batteryPct=battery1 :latency=connection :coordinates="testCoordinate" :vehicleName="'ERU'" :vehicleStatus="'In Use'"/>
        </div>
    </div>
    
</template>
  
<style scoped>
.camera-container {
    height: 100vh; /* Set the height of the container to 100% of the viewport height */
    width: 100%;
    display: flex; /* Use flexbox to align items vertically */
    justify-content: center; /* Center the child element horizontally */
    align-items: center; /* Center the child element vertically */
}
.camera-wrapper {
    position: relative; /* For positioning the button relative to the camera */
    height: 100%; 
    width: 75%;
}
.back {
    position: absolute; /* Position the button relative to the container */
    top: 10px;
    left: 10px; 
    padding: 5px 10px;
    border: none;
    background-color: lightgray;
    color: black;
    cursor: pointer;
  }
  
.VehicleComponents {
    display: flex;
    flex-direction: column;
    height: 100vh;
}
</style>