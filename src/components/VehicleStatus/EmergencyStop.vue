<template>
    <div class="outer_div">
   
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
    .connection-container {
        position: relative;
        justify-content: center;
        display: flex;
        gap: 0.05em;
        height: 100%;
        width: 100%;
        border-radius: 12%;
        /* background-color: white; */
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
        width: 180%;
        font-size:0.8em;
    }
    
</style> 