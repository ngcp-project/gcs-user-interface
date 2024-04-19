<script setup lang="ts">
import Camera from "../components/Camera.vue";
import Status from '../components/VehicleStatusComponent.vue';
import NavBar from '../components/Navbar.vue';
import { onMounted, ref } from 'vue';
import Map from '../components/Map.vue';

// for ERU widget
const receivedData = ref<any>(null);
const batteryPct = ref(0);
const testCoordinate = ref({longitude: 0, latitude: 0})
const dummyConnection = ref(0);
// for MEA widget
const receivedData2 = ref<any>(null);
const batteryPct2 = ref(78);
const testCoordinate2 = ref({longitude: 57.848923, latitude: -67.384919})
const dummyConnection2 = ref(65);

// create websocket connection once Static Screen finishes initial rendering
onMounted(() => {
    let client = new WebSocket('ws://localhost:5135/');
    console.log("Connected to 5135 server");

    client.addEventListener("message", (event) => {
        const data = JSON.parse(event.data);
        receivedData.value = data; 

        console.log("Received data from mockWebsock:", receivedData);
        batteryPct.value = receivedData.value.batteryLife;
        testCoordinate.value.latitude = receivedData.value.currentPosition.latitude;
        testCoordinate.value.longitude = receivedData.value.currentPosition.longitude;
        dummyConnection.value = receivedData.value.dummyConnection;
    });

    // -- uncomment below to use test-websocket-server.cjs for data for single vehicle widget in Static Screen -- //
    // let client2 = new WebSocket('ws://localhost:3000/');
    // console.log("Connected to port 3000 server")

    // client2.addEventListener("message", (event) => {
    //     const data = JSON.parse(event.data);
    //     receivedData2.value = data;

    //     console.log("Received data from test-websocket-server:", receivedData2);
    //     batteryPct2.value = receivedData2.value.battery;
    //     testCoordinate2.value.latitude = receivedData2.value.currentPosition.latitude;
    //     testCoordinate2.value.longitude = receivedData2.value.currentPosition.longitude;
    //     dummyConnection2.value = receivedData2.value.dummy_connection;
    // });
});

// --- testing with dummy reactive data --- //
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
  <div class="screen_div">
    <!-- Map component will be placed below -->
    <div class="map_div">
        <Map></Map>
    </div>

    <div class="four-status-rightside">
        <!-- For final product, pass in a Vehicle Object instead that contains all of the information for the VehicleStatusComponent to display-->
        <Status :batteryPct=batteryPct :latency=dummyConnection :coordinates=testCoordinate :vehicleName="'ERU'" :vehicleStatus="'In Use'"/>
        <Status :batteryPct=.40 :latency=36 :coordinates=testCoordinate2 :vehicleName="'MEA'" :vehicleStatus="'Standby'"/>
        <Status :batteryPct=0 :latency=100 :coordinates="testCoordinateObject2" :vehicleName="'MRA'" :vehicleStatus="'Offline'"/>
        <Status :batteryPct=.10 :latency=0 :coordinates="testCoordinateObject1" :vehicleName="'FRA'" :vehicleStatus="'Offline'"/>
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