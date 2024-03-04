<script setup lang="ts">

import Camera from "../components/Camera.vue";
</script>

<template>
    <div class="camera-container">
        <Camera :cameraNumber="1" />
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
</style>