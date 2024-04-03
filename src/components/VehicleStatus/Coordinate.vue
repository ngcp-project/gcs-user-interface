<template>
    <div class="coordinate-container">
        <div class="coordinate-div">Longitude: {{ coordinates.longitude }}</div>
        <div class="coordinate-div">Latitude: {{ coordinates.latitude }}</div>
    </div>
</template>
    
<script lang="ts">
    export default {
        props: {
            coordinates: { required: true, type: Object }
        },
        computed: {

        }
    };
</script>
    
    
<style scoped>
    .coordinate-container {
        position: relative;
        display: flex;
        flex-direction: column;
        width: 75%;
        /* border: 0.05em solid black; */
    }

    .coordinate-div {
        position: relative;
        padding: 1%;
        font-size: 1.2em;
        color: black;
        width: 100%;
    }


</style> 