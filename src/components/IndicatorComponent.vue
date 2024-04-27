<script lang="ts">
    import Airspeed from './FlightComponents/Airspeed.vue';
    import Altimeter from './FlightComponents/Altimeter.vue';
    import Altitude from './FlightComponents/Altitude.vue';
    import Heading from './FlightComponents/Heading.vue';

    export default {
        props: {
            vehicleName: { required: true, type: String },
            pitch: { required: true, type: Number},
            roll: { required: true, type: Number},
            altitude: {required: true, type: Number},
            airspeed: { required: true, type: Number },
            yaw: { required: true, type: Number }
        },
        components: {
            Airspeed,
            Altimeter,
            Altitude,
            Heading
        },
        computed: {
            changeSpeedSize() {
                if (this.vehicleName == 'ERU') {        // if this is ERU: make the airspeed component larger
                    return 300
                } else {
                    return 200                          // else: make the airspeed component the same size as other components
                }
            },        
        }
    };
</script>

<template>
  <div class="flight-indicators-container">
    <div v-if="vehicleName != 'ERU'" class="flight-indicator">
        <Altitude class="pitch-roll-indicator" :pitch=pitch :roll=roll></Altitude>
    </div>

    <div class="flight-indicator" :style="[vehicleName == 'ERU' ? { width: '70%' } : { width: '45%'}]"> 
        <Airspeed class="airspeed-indicator" :airspeed=airspeed :size="changeSpeedSize"></Airspeed>
    </div>

    <div v-if="vehicleName != 'ERU'" class="flight-indicator">
        <Altimeter class="altitude-indicator" :altitude=altitude></Altimeter>
    </div>

    <div v-if="vehicleName != 'ERU'" class="flight-indicator">
        <Heading class="altitude-indicator" :yaw=yaw></Heading>
    </div>
  </div>
</template>
    
<style scoped>
    .flight-indicators-container {
        display: flex;
        flex-wrap: wrap;
        gap: 2%;
        height: 56%; 
        width: 100%;
        justify-content: center;
        }

    .flight-indicator{
        position: relative;
        width: 45%;
        border: 1px solid black;
        padding-bottom: 2%;
        background-color: rgb(35, 34, 34);    
        border-radius: 3%;    
    }

</style> 