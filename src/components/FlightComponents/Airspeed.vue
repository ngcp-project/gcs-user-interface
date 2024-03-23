<script>
import {Airspeed} from  'vue-flight-indicators'

export  default {
	name:  'airspeed',
    props: {
        airspeed: { required: true, type: Number},
    },  
	components: {
		Airspeed
	}
}
</script>

<template>
	<div class="airspeed-div">
		<Airspeed :size="220" :airspeed="airspeed"/>
	</div>
</template>

<style scoped>
    .airspeed-div {
        height: 100%;
        width:100%;
        border: 1px solid black;
    }

</style>