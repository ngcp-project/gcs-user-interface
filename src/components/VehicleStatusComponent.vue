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
