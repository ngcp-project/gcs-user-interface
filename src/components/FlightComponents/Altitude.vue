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
		<Attitude :size="220" :pitch="pitch" :roll="roll" />
	</div>
</template>

<style scoped>
    .altitude-div {
        height: 100%;
        width:100%;
        border: 1px solid black;
    }

</style>