<template>
    <button class="open-button">OPEN</button>
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

    .open-button {
        position: relative;
        display: flex;
        justify-content: center;
        width: 30%;
        height: 20%;
        background-color: #011949;
        font-size: 1em;
        transition: background-color 0.20s;
    } 

    .open-button:hover {
        background-color: #194398;
    }
</style> 