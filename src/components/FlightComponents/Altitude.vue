<script>
import {Attitude} from  'vue-flight-indicators'

export  default {
	name:  'altitude',
    props: {
        pitch: { required: true, type: Number},
        roll: {required: true, type: Number},
    },  
	components: {
		Attitude
	}
}
</script>

<template>
	<div class="altitude-div">
		<Attitude :size="200" :pitch="pitch" :roll="roll" />
        <span style="font-size: 1.2em;"> Pitch: {{ pitch }} </span>
        <span style="font-size: 1.2em;"> Roll: {{ roll }} </span>
	</div>
</template>

<style scoped>
    .altitude-div {
        position: relative;
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        border: 1px solid black;
    }
</style>