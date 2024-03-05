<template>
    <div class="outer_div">
        <div class="container">
            <div v-if="latency == 0" class="grayed_bar" style='height: 20%'></div>
            <div v-else class="bar" style='height: 20%'></div>

            <div v-if="latency <= 30" class="grayed_bar" style='height: 40%'></div>
            <div v-else class="bar" style='height: 40%'></div>

            <div v-if="latency <= 50" class="grayed_bar" style='height: 60%'></div>
            <div v-else class="bar" style='height: 60%'></div>

            <div v-if="latency <= 70" class="grayed_bar" style='height: 80%'></div>
            <div v-else class="bar" style='height: 80%'></div>
        </div>
        <span class="connection_number">{{ latency }} ms</span>
    </div>
    
    </template>
    
<script lang="ts">

    export default {
        data() {
            return {};    
        },
        props: {
            latency: { required: true, type: Number},
        },
        computed: {
        }
    };
</script>
    
    
<style scoped>
    .outer_div {
        display: flex;
        position: relative;
        height: 25%;
        width: 10%;  
    }
    .container {
        position: relative;
        justify-content: center;
        display: flex;
        gap: 0.1em;
        height: 100%;
        width: 100%;
        border-radius: 12%;
        background-color: white;
    }
    .bar {
        width:100%;
        background-color: white;
        border: 0.1em solid black;
        margin-top: auto;
        border-radius: 20%;
    }
    .grayed_bar {
        width:100%;
        background-color: rgb(136, 135, 135);
        border: 0.1em solid black;
        margin-top: auto;
        opacity: 0.2;
        border-radius: 20%;
    }
    .connection_number {
        position: absolute;
        left: 110%;
        bottom: 0%;
        width: 120%;
    }
    
</style> 