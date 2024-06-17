<script lang="ts">
import Battery from "./VehicleStatus/VehicleBattery.vue";
import Connection from "./VehicleStatus/VehicleConnection.vue";
import VehicleTitle from "./VehicleStatus/VehicleTitle.vue";
import EmergencyStop from "./VehicleStatus/EmergencyStop.vue";
import Open from "./VehicleStatus/OpenVehicleStatus.vue";
import Coordinate from "./VehicleStatus/VehicleCoordinate.vue";

export default {
  props: {
    vehicleName: { required: true, type: String },
    vehicleStatus: { required: true, type: String },
    batteryPct: { required: true, type: Number },
    latency: { required: true, type: Number },
    coordinates: { required: true, type: Object },
    isInKeepInZone: { required: true, type: Boolean },
    isInKeepOutZone: { required: true, type: Boolean }
  },
  components: {
    VehicleTitle,
    Battery,
    Connection,
    EmergencyStop,
    Open,
    Coordinate
  }
};
</script>

<template>
  <div
    class="flex h-fit w-fit flex-col gap-1 rounded-sm border bg-secondary p-2 hover:bg-secondary/80"
  >
    <!-- Left side of container (Name, Status, Battery, Connection)-->
    <div class="flex h-full w-full items-center justify-between gap-1">
      <VehicleTitle
        class="flex-grow"
        :vehicleName="vehicleName"
        :vehicleStatus="vehicleStatus"
        :isInKeepInZone="isInKeepInZone"
        :isInKeepOutZone="isInKeepOutZone"
      />
    </div>
    <div class="flex w-full items-center gap-2">
      <div class="flex h-full w-fit flex-none items-center gap-1">
        <Battery :percentage="batteryPct" :charging="false" />
        <span style="">{{ Math.round(batteryPct * 100) }}%</span>
        <!-- Multiply batteryPct by 100 because it is in between 0 and 1 -->
      </div>
      <div class="flex h-[30px] w-[70px] flex-none">
        <Connection :latency="latency" :displayLatency="true" class="h-full w-[100px]" />
        <!-- <div class="connection-status-specifics">                  <-- moved the text showing latency into actual Connection component 
                    <span style="font-size: 0.9em;">Connection:</span>              bcuz we calculate latency in Connection.vue
                    <span style="font-size: 0.9em;">Last Packet: {{ latency }} </span>
                </div> -->
      </div>
    </div>

    <!-- Right side of container (Open button, Coordinates, Emergency Stop)-->
    <div class="flex h-full w-full flex-col gap-1">
      <Coordinate :coordinates="coordinates"></Coordinate>
      <div class="flex w-full items-center justify-between gap-2">
        <Open />
        <EmergencyStop :vehicleName="vehicleName" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.status_container {
  /* display: flex;
  position: relative;
  height: 100%;
  width: 100%;
  border: 0.1em solid black;
  background-color: white;
  color: black; */
  @apply flex h-full w-full bg-secondary;
}

.left-container {
  /* position: relative;
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 46%; */
  @apply flex h-full w-1/2 flex-col;
}

.right-container {
  /* position: relative;
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 54%; */
  @apply flex h-full w-1/2 flex-col;
}

.battery-status-container {
  /* display: flex;
  position: relative;
  width: 100%;
  height: 20%;
  gap: 6%;
  margin-top: auto; */
  @apply mt-auto flex h-1/5 w-full;
}

.connection-status-container {
  /* display: flex;
  position: relative;
  width: 100%;
  height: 30%;
  margin-bottom: 2%; */
  /*  margin-left: 10%;  */
  @apply flex h-1/3 w-full;
}

.connection-status-specifics {
  /* display: flex;
  flex-direction: column;
  padding-left: 3%;
  padding-top: 6%; */
  @apply flex flex-col;
}

.adjust-battery {
  /* height: 84%;
  width: 34%;
  margin-left: 8%; */
  @apply ml-1 h-full w-1/3;
}

.adjust-connection {
  /* height: 92%;
  width: 26%;
  padding-left: 10%; */
  @apply h-full w-1/5;
}

.adjust-emergency-button {
  /* margin-top: auto;
  margin-bottom: 5%;
  margin-left: 10%; */
}

.adjust-open-button {
  /* margin-top: 4%;
  margin-left: 58%; */
}

.adjust-coordinates {
  /* margin-top: 18%;
  margin-left: 8%; */
}
</style>
