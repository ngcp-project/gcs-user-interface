<template>
    <div class="vehicle-title-container">
        <div class="vehicle-name-div"> {{ vehicleName }}</div>
        <div class="vehicle-status-div">Status: {{ vehicleStatus }}</div>
    </div>
    
    </template>
    
<script lang="ts">

    export default {
        props: {
            vehicleName: { required: true, type: String },
            vehicleStatus: { required: true, type: String }
        }
    };
</script>
    
    
<style scoped>

    .vehicle-title-container {
        position: relative;
        width: 100%;
        padding-top: 8%;
        padding-left: 6%;
    }

    .vehicle-name-div {
        position: relative;
        font-size: 2.5em;
    }

    .vehicle-status-div {
        position: relative;
        font-size: 1.5em;  
        margin-top: 5%;;
    }

</style> 