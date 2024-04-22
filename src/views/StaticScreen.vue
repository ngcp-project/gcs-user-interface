<script setup lang="ts">
import Status from '../components/VehicleStatusComponent.vue';
import { onMounted, onBeforeUnmount, ref, reactive } from 'vue';
import Map from '../components/Map.vue';
import { initializeWSConnections, getAllConnections, closeConnections } from "../webSocket";

// initialize reactive variables for each vehicle's telemetry data
const ERU_data = reactive({batteryPct: 0, connection: 0, coordinates: {longitude: 39.323719, latitude: -76.591978}, status: 'Standby'});
const MEA_data = reactive({batteryPct: .78, connection: 22, coordinates: {longitude: 57.848923, latitude: -118.377468}, status: 'Standby'});
const MRA_data = reactive({batteryPct: .2, connection: 65, coordinates: {longitude: 153.59374, latitude: -67.384919}, status: 'Standby'});
const FRA_data = reactive({batteryPct: .4, connection: 98, coordinates: {longitude: 34.060425, latitude: -117.816546}, status: 'Standby'});

// add event listeners for each vehicle WS connection that updates the reactive variables whenever new data is received
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
            dummyConnection: data.dummyConnection,
            status: data.vehicleStatus
        };

        switch (vehicleKey) {
            case 'eru':
                ERU_data.batteryPct = receivedData.batteryLife;
                ERU_data.coordinates.latitude = receivedData.currentPosition.latitude;
                ERU_data.coordinates.longitude = receivedData.currentPosition.longitude;
                ERU_data.connection = receivedData.dummyConnection;   
                ERU_data.status = receivedData.status;   
            case 'mea':
                MEA_data.batteryPct = receivedData.batteryLife;
                MEA_data.coordinates.latitude = receivedData.currentPosition.latitude;
                MEA_data.coordinates.longitude = receivedData.currentPosition.longitude;
                MEA_data.connection = receivedData.dummyConnection; 
                MEA_data.status = receivedData.status;     
            case 'fra':
                FRA_data.batteryPct = receivedData.batteryLife;
                FRA_data.coordinates.latitude = receivedData.currentPosition.latitude;
                FRA_data.coordinates.longitude = receivedData.currentPosition.longitude;
                FRA_data.connection = receivedData.dummyConnection;
                FRA_data.status = receivedData.status;   
            case 'mra':
                MRA_data.batteryPct = receivedData.batteryLife;
                MRA_data.coordinates.latitude = receivedData.currentPosition.latitude;
                MRA_data.coordinates.longitude = receivedData.currentPosition.longitude;
                MRA_data.connection = receivedData.dummyConnection;
                MRA_data.status = receivedData.status;   
            }
        });
    } // end for
}

let wsConnections: { [key: string]: WebSocket } = {};
// create websocket connection once Static Screen finishes initial rendering
onMounted(() => {
    wsConnections = getAllConnections();
    // below for loop just for testing. check console for outputs
    // for (let key in wsConnections) {
    //     console.log("FROM StaticScreen, got ws connection for " + key);
    // };
    addListeners();
});

// close connections on component unmount
onBeforeUnmount(() => {
    closeConnections();
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