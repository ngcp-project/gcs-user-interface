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
const batteryPct2 = ref(.78);
const testCoordinate2 = ref({longitude: 57.848923, latitude: -67.384919})
const dummyConnection2 = ref(65);
// for MRA widget
const receivedData3 = ref<any>(null);
const batteryPct3 = ref(.2);
const testCoordinate3 = ref({longitude: 57.848923, latitude: -67.384919})
const dummyConnection3 = ref(65);
// for FRA widget
const receivedData4 = ref<any>(null);
const batteryPct4 = ref(.4);
const testCoordinate4 = ref({longitude: 57.848923, latitude: -67.384919})
const dummyConnection4 = ref(65);

const connections = [
    'ws://localhost:5135/ws/eru',
    'ws://localhost:5136/ws/mea',
    'ws://localhost:5137/ws/fra',
    'ws://localhost:5138/ws/mra',

]

let wsClientsList: { [key: string]: WebSocket } = {};
let test: { [key: string]: string } = {};

let names = ['eru', 'mea', 'fra', 'mra']
const telemetryData = [];

function getWebsocketConnection(): string {
    for (let i = 0; i < connections.length; i++) {
        let newWebSocket = new WebSocket(connections[i]);
        let vehicleName = names[i];
        wsClientsList[names[i]] = newWebSocket;
        console.log(connections[i]);
    }

    for (const [vehicleKey, webSocketConnection] of Object.entries(wsClientsList)) {
        console.log(vehicleKey);
        webSocketConnection.addEventListener("message", (event) => {
        const data = JSON.parse(event.data);
        receivedData.value = data; 

        console.log("Received data from eru server:", receivedData);
        if (vehicleKey == 'eru') {
            batteryPct.value = receivedData.value.batteryLife;
            testCoordinate.value.latitude = receivedData.value.currentPosition.latitude;
            testCoordinate.value.longitude = receivedData.value.currentPosition.longitude;
            dummyConnection.value = receivedData.value.dummyConnection;
        } else if (vehicleKey == 'mea') {
            batteryPct2.value = receivedData.value.batteryLife;
            testCoordinate2.value.latitude = receivedData.value.currentPosition.latitude;
            testCoordinate2.value.longitude = receivedData.value.currentPosition.longitude;
            dummyConnection2.value = receivedData.value.dummyConnection;
        } else if (vehicleKey == 'fra') {
            batteryPct3.value = receivedData.value.batteryLife;
            testCoordinate3.value.latitude = receivedData.value.currentPosition.latitude;
            testCoordinate3.value.longitude = receivedData.value.currentPosition.longitude;
            dummyConnection3.value = receivedData.value.dummyConnection;
        } else if (vehicleKey == 'mra') {
            batteryPct4.value = receivedData.value.batteryLife;
            testCoordinate4.value.latitude = receivedData.value.currentPosition.latitude;
            testCoordinate4.value.longitude = receivedData.value.currentPosition.longitude;
            dummyConnection4.value = receivedData.value.dummyConnection;
        }
        });
    }


    return "hi";
}

// create websocket connection once Static Screen finishes initial rendering
onMounted(() => {
    getWebsocketConnection();
    // let receivedData = getWebsocketConnection();
    //     batteryPct.value = receivedData.value.batteryLife;
    //     testCoordinate.value.latitude = receivedData.value.currentPosition.latitude;
    //     testCoordinate.value.longitude = receivedData.value.currentPosition.longitude;
    //     dummyConnection.value = receivedData.value.dummyConnection;

    let client = new WebSocket('ws://localhost:5135/ws/eru');
    console.log("Connected to eru server");

    client.addEventListener("message", (event) => {
        const data = JSON.parse(event.data);
        receivedData.value = data; 

        console.log("Received data from eru server:", receivedData);
        batteryPct.value = receivedData.value.batteryLife;
        testCoordinate.value.latitude = receivedData.value.currentPosition.latitude;
        testCoordinate.value.longitude = receivedData.value.currentPosition.longitude;
        dummyConnection.value = receivedData.value.dummyConnection;
    });

    // let client3 = new WebSocket('ws://localhost:5136/ws/mea');
    // console.log("Connected to mea server");

    // client3.addEventListener("message", (event) => {
    //     const data = JSON.parse(event.data);
    //     receivedData3.value = data; 

    //     console.log("Received data from mea server:", receivedData3);
    //     batteryPct3.value = receivedData3.value.batteryLife;
    //     testCoordinate3.value.latitude = receivedData3.value.currentPosition.latitude;
    //     testCoordinate3.value.longitude = receivedData3.value.currentPosition.longitude;
    //     dummyConnection3.value = receivedData3.value.dummyConnection;
    // });

    // let client2 = new WebSocket('ws://localhost:5137/ws/fra');
    // console.log("Connected to fra server");

    // client2.addEventListener("message", (event) => {
    //     const data = JSON.parse(event.data);
    //     receivedData2.value = data; 

    //     console.log("Received data from fra server:", receivedData2);
    //     batteryPct2.value = receivedData2.value.batteryLife;
    //     testCoordinate2.value.latitude = receivedData2.value.currentPosition.latitude;
    //     testCoordinate2.value.longitude = receivedData2.value.currentPosition.longitude;
    //     dummyConnection2.value = receivedData2.value.dummyConnection;
    // });


    // let client4 = new WebSocket('ws://localhost:5138/ws/mra');
    // console.log("Connected to mra server");

    // client4.addEventListener("message", (event) => {
    //     const data = JSON.parse(event.data);
    //     receivedData4.value = data; 

    //     console.log("Received data from mra server:", receivedData4);
    //     batteryPct4.value = receivedData4.value.batteryLife;
    //     testCoordinate4.value.latitude = receivedData4.value.currentPosition.latitude;
    //     testCoordinate4.value.longitude = receivedData4.value.currentPosition.longitude;
    //     dummyConnection4.value = receivedData4.value.dummyConnection;
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
        <Status :batteryPct=batteryPct2 :latency=dummyConnection2 :coordinates=testCoordinate2 :vehicleName="'MEA'" :vehicleStatus="'Standby'"/>
        <Status :batteryPct=batteryPct3 :latency=dummyConnection3 :coordinates="testCoordinate3" :vehicleName="'MRA'" :vehicleStatus="'Offline'"/>
        <Status :batteryPct=batteryPct4 :latency=dummyConnection4 :coordinates="testCoordinate4" :vehicleName="'FRA'" :vehicleStatus="'Offline'"/>
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