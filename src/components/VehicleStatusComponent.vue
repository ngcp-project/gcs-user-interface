<script lang="ts">
    import Battery from './VehicleStatus/Battery.vue';
    import Connection from './VehicleStatus/Connection.vue';
    import VehicleTitle from './VehicleStatus/VehicleTitle.vue';
    import EmergencyStop from './VehicleStatus/EmergencyStop.vue';
    import Open from './VehicleStatus/Open.vue';

    export default {
        props: {
            vehicleName: { required: true, type: String},
            vehicleStatus: { required: true, type: String},
            batteryPct: {required: true, type: Number},
        },
        components: {
            VehicleTitle,
            Battery,
            Connection,
            EmergencyStop,
            Open
        },
        // computed: {
        //     batteryPercentage(): number {
        //         return this.batteryPct;
        //     }
        // },
    };
</script>

<template>
    <div class="status_container">

        <!-- Left side of container (Name, Status, Battery, Connection)-->
        <div class="left-container">
            <VehicleTitle :vehicleName="vehicleName" :vehicleStatus="vehicleStatus"/>
            <Battery :percentage = "batteryPct" :charging="false" class="adjust-battery"/>
            <Connection :latency=65 class="adjust-connection"/> 
        </div>

        <div class="right-container">
            <Open class="adjust-open-button"></Open>
            <EmergencyStop class="adjust-emergency-button"/>
        </div>
    </div>
</template>
    
    
<style scoped>
    .status_container {
        display: flex;
        position: relative;
        height: 200px;

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
        width: 40%;
    }

    .right-container {
        position: relative;
        display: flex;
        flex-direction: column;
        height: 100%;   
        width: 60%; 
    }

    .adjust-battery {
        height: 17%;
        width: 34.5%;
        margin-top: auto;
        margin-left: 8%;
    }

    .adjust-connection {
        height: 28%;
        width: 26%; 
        margin-top: 2%;
        margin-bottom: 4%;  
        margin-left: 10%; 
    }

    .adjust-emergency-button {
        margin-top: auto;
        margin-bottom: 4%;  
        margin-left: 20%;
    }

    .adjust-open-button {
        margin-top: 4%;
        margin-left: 60%;
    }
 
</style> 