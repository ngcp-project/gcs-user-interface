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
		<Altimeter :size="220" :altitude="altitude"/>
	</div>
</template>

<style scoped>
    .altimeter-div {
        height: 100%;
        width:100%;
        border: 1px solid black;
    }

</style>