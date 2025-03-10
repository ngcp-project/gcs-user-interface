<script setup lang="ts">
import { Card, CardContent, CardTitle } from '@/components/ui/card'
import CardFooter from '@/components/ui/card/CardFooter.vue';
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

    <CardContent class="mt-2 flex flex-col items-start space-y-3">
      <!-- Battery & Connection Info -->
      <section class="flex items-center justify-between py-1 gap-x-2 ">
        <Connection :latency="connection" :display-latency="false" />
        <span class="font-semibold">
          {{ `${connection}ms` }}
        </span>
        <Battery :percentage="battery" :charging="false" />
        <span class="font-semibold">
          {{ `${battery}%` }}
        </span>
      </section>

      <!-- Coordinates -->
      <section class="mt-2 flex flex-col">
        <span class="font-semibold">
          Latitude:
          {{  latitude  }}
        </span>
        <span class="font-semibold">
          Longitude:
          {{  longitude  }}
        </span>
      </section>

      <CardFooter class="mt-4 flex w-full justify-center">
        <EmergencyStop :vehicle-name="vehicleName"/>
      </CardFooter>
    </CardContent>
  </Card>
</template>