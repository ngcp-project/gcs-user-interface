<template>
    <div class="vehicle-title-container">
        <div class="vehicle-name-div"> 
                {{ vehicleName }} 
               <img :src="getIconPath" class="vehicle-icon"/>
               <img v-if="!isInKeepInZone || isInKeepOutZone" src="../../assets/warning.png" class="warning-icon"/>
        </div>
        <div class="vehicle-status-div">Status: {{ vehicleStatus }}</div>
    </div>
   
    </template>
    
<script lang="ts">

// import {isInKeepInZone, isInKeepInZone2, isInKeepOutZone2} from "../../Functions/geofence";

    export default {
        props: {
            vehicleName: { required: true, type: String },
            vehicleStatus: { required: true, type: String },
            // vehicleIcon: { type: Object },
            isInKeepInZone: { required: true, type: Boolean},
            isInKeepOutZone: { required: true, type: Boolean}
        },
        computed: {
            getIconPath() {
                return new URL('../../assets/' + this.vehicleName + '.png', import.meta.url).href;
            }
        }
    };
</script>
    
    
<style scoped>

    .vehicle-title-container {
        position: relative;
        width: 100%;
        padding-top: 6%;
        padding-left: 6%;
    }

    .vehicle-name-div {
        position: relative;
        display: flex;
        font-size: 2.3em;
        align-items: center;
        gap: 2%;
    }

    .vehicle-status-div {
        position: relative;
        font-size: 1.4em;  
        margin-top: 3%;
    }

    .vehicle-icon {
        position: relative;
        width: 16%;
    }

    .warning-icon {
        position: relative;
        width: 16%;
    }

</style> 