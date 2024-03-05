<template>
    <div class="outer_div">
        <div class="battery_container">
            <!-- <div :class="percentageCSS" :style="{ width: this.percentage + '%' }"></div> -->
            <div :class="percentageCSS" :style="[percentage > 15 ? { width: percentage + '%' } : { width: '15%'}]"></div>
        </div>
        <div class="battery_widget"></div> 
        <img class="lightingSymbol" :class="batteryStatus" src="..\..\assets\lightning-icon-png-5.png" >
    </div>
    
</template>
    
<script lang="ts">

    export default {
        data() {
            return {};    
        },
        props: {
            percentage: { required: true, type: Number},
            charging: { required: true, type: Boolean},
        },
        computed: {
            percentageCSS() {
                if (this.percentage <= 0) {
                    return 'zeroPercent'
                } else if ((this.percentage > 0) && (this.percentage <= 15)) {
                    return 'tenPercent'
                } else if ((this.percentage > 15) && (this.percentage <= 30)) {
                    return 'twentyFivePercent'
                } else if ((this.percentage > 30) && (this.percentage <= 50)) {
                    return 'fiftyPercent'
                } else {
                    return 'normalPercent'
                }
            },
            batteryStatus() {
                if (this.charging == true) {
                    return 'charging'
                } else if (this.percentage <= 0) {
                    return 'dead'
                }
            }
        },
    };
</script>
    
    
<style scoped>
    .outer_div {
        display: flex;
        position: relative;
        /* height: 110px;
        width: 225px;     */
        height: 12%;
        width: 12%;  
    }

    .battery_widget {
        position: relative;
        height: 50%;
        width: 10%;  
        background-color:black; 
        top: 30%;
        left: 1%;
        /* top: 30%; */
        /* border-radius: 0 12px 12px 0; */
        border-radius: 0 25% 25% 0;
    }

    /* .battery_icon {
        position: absolute;
        width: 40%;
        left: 30%;
        top: 15%;
        animation: blinker 1s linear infinite;
        visibility: hidden;
    } */
    .lightingSymbol {
        position: absolute;
        width: 40%;
        left: 30%;
        top: 15%;
        visibility: hidden;
    }
    .dead {
        visibility: visible;
        animation: blinker 1s linear infinite;
    }
    .charging {
        visibility: visible;
    }

    @keyframes blinker {
        50% {
            opacity: 0;
        }
    }   

    .battery_container {
        position: relative;
        display: flex;
        border: 0.1em solid black;
        height: 100%;
        width: 100%;
        border-radius: 12%;
        background-color: white;
    }

    #battery_progress {
        background-color: rgb(116, 194, 92); 
        border-radius: 12%;
        height:100%;
        width: 100%; 
    }

    .zeroPercent {
        width: 0%;

    }
    .tenPercent {
        background-color: red; 
        border-radius: 12%;
        height:100%;    
    }
    .twentyFivePercent {
        background-color: rgb(116, 115, 109);
        border-radius: 12%;
        height:100%;
    }
    .fiftyPercent {
        background-color: rgb(245, 225, 44); 
        border-radius: 12%;
        height:100%;
    }
    .normalPercent {
        background-color: rgb(116, 194, 92); 
        border-radius: 12%;
    }

</style> 