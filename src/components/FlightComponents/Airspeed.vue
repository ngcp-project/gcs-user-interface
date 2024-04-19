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
		<Airspeed :size="200" :airspeed="airspeed"/>
        <span style="font-size: 1.2em;"> Speed: {{ airspeed }} ft/s</span>
	</div>
</template>

<style scoped>
    .airspeed-div {
        position: relative;
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 2%;
    }

</style>