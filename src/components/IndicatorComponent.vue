<script lang="ts">
    import Airspeed from './FlightComponents/Airspeed.vue';
    import Altimeter from './FlightComponents/Altimeter.vue';
    import Altitude from './FlightComponents/Altitude.vue';
    import Heading from './FlightComponents/Heading.vue';

    export default {
        props: {
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
    };
</script>

<template>
  <div class="flight-indicators-container">
    <div class="flight-indicator">
        <Altitude class="pitch-roll-indicator" :pitch=pitch :roll=roll></Altitude>
    </div>

    <div class="flight-indicator">
        <Airspeed class="airspeed-indicator" :airspeed=airspeed></Airspeed>
    </div>

    <div class="flight-indicator">
        <Altimeter class="altitude-indicator" :altitude=altitude></Altimeter>
    </div>

    <div class="flight-indicator">
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