<script>
import {Altimeter} from  'vue-flight-indicators'

export  default {
	name:  'altimeter',
    props: {
        altitude: { required: true, type: Number},
    },  
	components: {
		Altimeter
	}
}
</script>

<template>
	<div class="altimeter-div">
		<Altimeter :size="200" :altitude="altitude"/>
        <span style="font-size: 1.2em;"> Altitude: {{ altitude }} ft/s</span>
	</div>
</template>

<style scoped>
    .altimeter-div {
        position: relative;
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 2%;
    }

</style>