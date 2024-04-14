<script setup lang="ts">

import Camera from "../components/Camera.vue";
import { useRoute } from 'vue-router';
const route = useRoute();
const cameraID = Number(route.params.id); // Assuming we're using camera Number 

</script>

<template>
  <div class="cam-focus-screen-div">
    <div class="camera-container">
        <div class="camera-wrapper">
            <router-link to="/" class="back">Back</router-link>
            <Camera :cameraID="cameraID" />
            <!-- Back button -->
        </div>
    </div>

    <div class="vehicle-info-container"></div>
</div>
</template>
  
<style scoped>
.cam-focus-screen-div {
    display:flex;
    height: 90vh; 
    width: 100%;
}

.camera-container {
    /* height: 90vh;  */
    height: 100%; 
    width: 75%;
    display: flex; /* Use flexbox to align items vertically */
    justify-content: center; /* Center the child element horizontally */
    align-items: center; /* Center the child element vertically */
}
.camera-wrapper {
    position: relative; /* For positioning the button relative to the camera */
    height: 100%; 
    width: 100%;
}
.back {
    position: absolute; /* Position the button relative to the container */
    top: 10px;
    left: 10px; 
    padding: 5px 10px;
    border: none;
    background-color: lightgray;
    color: black;
    cursor: pointer;
  }
  .vehicle-info-container {
    height: 100%; 
    width: 25%;
    /* background-color: pink; */
}
</style>