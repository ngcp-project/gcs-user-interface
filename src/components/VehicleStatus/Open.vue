<template>
    <!-- <div class="open-button-div">
        <button class="open-button">OPEN</button>
    </div> -->
    <button class="test-open-button">OPEN</button>
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
    .open-button-div {
        position: relative;
        height: 25%;
        width: 35%;  
    }

    .open-button {
        position: relative;
        display: inline-block;
        width: 100%;
        height: 100%;
        background-color: #011949;
        font-size: 1.3em;
    } 

    .test-open-button {
        position: relative;
        display: inline-block;
        width: 30%;
        height: 20%;
        background-color: #011949;
        font-size: 1em;
    } 

    .test-open-button:hover {
        background-color: #03369b;
    }
</style> 