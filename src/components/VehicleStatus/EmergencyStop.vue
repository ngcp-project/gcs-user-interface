<template>
    <div class="emergency-stop-div">
        <button class="emergency-button"> EMERGENCY STOP </button>
    </div>
    
</template>
    
<script lang="ts">

    export default {
        data() {
            return {};    
        },
        // props: {
        // },
        // computed: {
        // }
    };
</script>
    
    
<style scoped>
    .emergency-stop-div {
        position: relative;
        display: flex;
        height: 20%;
        width: 70%;  
        text-align: center;
    }

    .emergency-button {
        position: relative;
        display: inline-block;
        width: 100%;
        height: 100%;
        background-color: rgb(255, 57, 57);
        font-size: 1em;
    } 

    .emergency-button:hover {
        background-color: rgb(192, 40, 40);
    }
</style> 