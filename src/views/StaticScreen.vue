<script setup lang="ts">
import Status from '../components/VehicleStatusComponent.vue';
import { onMounted, ref } from 'vue';
import Map from '../components/Map.vue';
import { initializeWSConnections, getAllConnections, closeConnections } from "../webSocket";

// Reactive variables for ERU widget
const batteryPct_ERU = ref<number>(0);
const testCoordinate_ERU = ref({longitude: 39.323719, latitude: -76.591978})
const connection_ERU = ref<number>(0);
// Reactive variables for MEA widget
const batteryPct_MEA = ref<number>(.78);
const testCoordinate_MEA = ref({longitude: 57.848923, latitude: -118.377468})
const connection_MEA = ref<number>(22);
// Reactive variables for MRA widget
const batteryPct_MRA = ref<number>(.2);
const testCoordinate_MRA = ref({longitude: 153.59374, latitude: -67.384919})
const connection_MRA = ref<number>(65);
// Reactive variables for FRA widget
const batteryPct_FRA = ref<number>(.4);
const testCoordinate_FRA = ref({longitude: 34.060425, latitude: -117.816546})
const connection_FRA = ref<number>(98);

function addListeners() {
    for (const [vehicleKey, webSocketConnection] of Object.entries(wsConnections)) {
        webSocketConnection.addEventListener("message", (event) => {
        const data = JSON.parse(event.data);
        // const receivedData = ref<any>(null);
        // receivedData.value = data; 
        const receivedData = {
            batteryLife: parseFloat(data.batteryLife),
            currentPosition: {
                latitude: parseFloat(data.currentPosition.latitude),
                longitude: parseFloat(data.currentPosition.longitude)
            },
            dummyConnection: data.dummyConnection
        };

        if (vehicleKey == 'eru') {
            batteryPct_ERU.value = receivedData.batteryLife;
            testCoordinate_ERU.value.latitude = receivedData.currentPosition.latitude;
            testCoordinate_ERU.value.longitude = receivedData.currentPosition.longitude;
            connection_ERU.value = receivedData.dummyConnection;
        } else if (vehicleKey == 'mea') {
            batteryPct_MEA.value = receivedData.batteryLife;
            testCoordinate_MEA.value.latitude = receivedData.currentPosition.latitude;
            testCoordinate_MEA.value.longitude = receivedData.currentPosition.longitude;
            connection_MEA.value = receivedData.dummyConnection;
        } else if (vehicleKey == 'fra') {
            batteryPct_FRA.value = receivedData.batteryLife;
            testCoordinate_FRA.value.latitude = receivedData.currentPosition.latitude;
            testCoordinate_FRA.value.longitude = receivedData.currentPosition.longitude;
            connection_FRA.value = receivedData.dummyConnection;
        } else if (vehicleKey == 'mra') {
            batteryPct_MRA.value = receivedData.batteryLife;
            testCoordinate_MRA.value.latitude = receivedData.currentPosition.latitude;
            testCoordinate_MRA.value.longitude = receivedData.currentPosition.longitude;
            connection_MRA.value = receivedData.dummyConnection;
        }});
    } // end for
}

let wsConnections: { [key: string]: WebSocket } = {};
// create websocket connection once Static Screen finishes initial rendering
onMounted(() => {
    wsConnections = getAllConnections();
    // below for loop just for testing
    for (let key in wsConnections) {
        console.log("FROM StaticScreen, got ws connection for " + key);
        console.log(typeof wsConnections[key]);
    };
    
    addListeners();
});
</script>

<template>
  <div class="screen_div">
    <!-- Map component will be placed below -->
    <div class="map_div">
        <Map></Map>
    </div>

    <div class="four-status-rightside">
        <!-- For final product, pass in a Vehicle Object instead that contains all of the information for the VehicleStatusComponent to display-->
        <Status :batteryPct=batteryPct_ERU :latency=connection_ERU :coordinates=testCoordinate_ERU :vehicleName="'ERU'" :vehicleStatus="'In Use'"/>
        <Status :batteryPct=batteryPct_MEA :latency=connection_MEA :coordinates=testCoordinate_MEA :vehicleName="'MEA'" :vehicleStatus="'Standby'"/>
        <Status :batteryPct=batteryPct_MRA :latency=connection_MRA :coordinates=testCoordinate_MRA :vehicleName="'MRA'" :vehicleStatus="'Offline'"/>
        <Status :batteryPct=batteryPct_FRA :latency=connection_FRA :coordinates=testCoordinate_FRA :vehicleName="'FRA'" :vehicleStatus="'Offline'"/>
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