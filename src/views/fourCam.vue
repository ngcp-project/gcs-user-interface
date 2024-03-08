<script setup lang="ts">

import Camera from "../components/Camera.vue";
</script>

<template>
    <div class="grid">
        <div>
            <Camera :cameraNumber="1"/>
        </div>
        <div>
            <Camera :cameraNumber="2"/>
        </div>
        <div>
            <Camera :cameraNumber="2"/>
        </div>
        <div>
            <Camera :cameraNumber="1"/>
        </div>
    </div>
</template>