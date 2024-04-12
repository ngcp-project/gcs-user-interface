<script lang="ts">
    import Battery from './VehicleStatus/Battery.vue';
    import Connection from './VehicleStatus/Connection.vue';
    import VehicleTitle from './VehicleStatus/VehicleTitle.vue';
    import EmergencyStop from './VehicleStatus/EmergencyStop.vue';
    import Open from './VehicleStatus/Open.vue';
    import Coordinate from './VehicleStatus/Coordinate.vue';

    export default {
        props: {
            vehicleName: { required: true, type: String},
            vehicleStatus: { required: true, type: String},
            batteryPct: {required: true, type: Number},
            latency: { required: true, type: Number },
            coordinates: { required: true, type: Object }
        },
        components: {
            VehicleTitle,
            Battery,
            Connection,
            EmergencyStop,
            Open,
            Coordinate
        },
    };
</script>

<template>
    <div class="status_container">

        <!-- Left side of container (Name, Status, Battery, Connection)-->
        <div class="left-container">
            <VehicleTitle :vehicleName="vehicleName" :vehicleStatus="vehicleStatus"/>
            <div class="battery-status-container">
                <Battery :percentage = "batteryPct" :charging="false" class="adjust-battery"/>
                <span style="margin-top: 4%; font-size: 1.4em;">{{ batteryPct }}%</span>
            </div>
            <div class="connection-status-container">
                <Connection :latency="latency" class="adjust-connection"/> 
                <div class="connection-status-specifics">
                    <span style="font-size: 0.9em;">Connection:</span>
                    <span style="font-size: 0.9em;">Last Packet: {{ latency }} </span>
                </div>
            </div>
        </div>

        <!-- Right side of container (Open button, Coordinates, Emergency Stop)-->
        <div class="right-container">
            <Open class="adjust-open-button"></Open>
            <Coordinate :coordinates="coordinates" class="adjust-coordinates"></Coordinate>
            <EmergencyStop :vehicleName="vehicleName" class="adjust-emergency-button"/>
        </div>
    </div>
</template>
    
    
<style scoped>
    .status_container {
        display: flex;
        position: relative;
        height: 100%;
        /* width: 25%;  */
        width: 100%; 
        border: 0.1em solid black;
        background-color: white;
        color: black;
    }

    .left-container {
        position: relative;
        display: flex;
        flex-direction: column;
        height: 100%;
        width: 46%;
    }

    .right-container {
        position: relative;
        display: flex;
        flex-direction: column;
        height: 100%;   
        width: 54%; 
    }

    .battery-status-container {
        display: flex;
        position: relative;
        width: 100%; 
        height: 20%;
        gap: 6%;
        margin-top: auto;
    }

    .connection-status-container {
        display: flex;
        position: relative;
        width: 100%; 
        height: 30%;
        margin-bottom: 2%;  
      /*  margin-left: 10%;  */
    }

    .connection-status-specifics {
        display: flex;
        flex-direction: column;
        padding-left: 3%;
        padding-top: 6%;
    }

    .adjust-battery {
        height: 84%;
        width: 34%;
        margin-left: 8%;
    }

    .adjust-connection {
        height: 92%;
        width: 26%; 
        padding-left: 10%;
    }

    .adjust-emergency-button {
        margin-top: auto;
        margin-bottom: 5%;  
        margin-left: 10%;
    }

    .adjust-open-button {
        margin-top: 4%;
        margin-left: 58%;
    }

    .adjust-coordinates {
        margin-top: 18%;
        margin-left: 8%;   
    }
 
</style> 