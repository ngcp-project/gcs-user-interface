<script setup lang="ts">
import { Card, CardContent, CardTitle } from '@/components/ui/card'
import EmergencyStop from '@/components/VehicleStatus/EmergencyStop.vue';
import Battery from "@/components/VehicleStatus/VehicleBattery.vue";
import Connection from "@/components/VehicleStatus/VehicleConnection.vue";

// Define Props
defineProps<{
  vehicleName: 'ERU' | 'MEA' | 'MRA'
  battery: number
  connection: number
  latitude: number
  longitude: number
}>()
</script>

<template>
  <Card class="m-2 p-2 relative bg-primary text-secondary">
    <!-- Vehicle Name -->
    <CardTitle class="text-xl font-bold">{{ vehicleName }}</CardTitle>

    <CardContent class="flex flex-col gap-2">
      <!-- Battery & Connection Info -->
      <section class="mt-2 flex justify-center gap-6 [&>*]:flex [&>*]:items-center [&>*]:gap-1">
        <div>
          <Connection :latency="connection" :display-latency="false" />
          <span>{{ `${connection}ms` }}</span>
        </div>
        <div>
          <Battery :percentage="battery" :charging="false" />
          <span>{{ `${battery}%` }}</span>
        </div>
      </section>

      <section class="mt-2 flex flex-col font-semibold tracking-wide">
        <span>
          Latitude:
          <span class="font-medium tracking-tight">{{  latitude  }}</span>
        </span>
        <span>
          Longitude:
          <span class="font-medium tracking-tight">{{  longitude  }}</span>
        </span>
      </section>

      <EmergencyStop :vehicle-name="vehicleName"/>
    </CardContent>
  </Card>
</template>