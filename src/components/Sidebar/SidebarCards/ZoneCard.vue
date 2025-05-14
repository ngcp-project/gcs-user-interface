<script setup lang="ts">
import { Card, CardContent, CardTitle, CardFooter } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { computed } from 'vue'
import { Trash2, Eye, EyeOff, Pencil, Square, Plus } from 'lucide-vue-next'
import { missionStore } from "@/lib/MissionStore";
import { ZoneType } from "@/lib/bindings";
import { title } from 'process';

const props = defineProps<{
  zoneType: ZoneType
}>()

// Title Color Styles
const titleStyles = {
  titleColor: {
    'In': 'bg-chart-2',
    'Out': 'bg-destructive'
  }
}

const zoneType = {
  KeepIn: "In",
  KeepOut: "Out"
}[String(props.zoneType)]

// Track zones
const currentMissionId = missionStore.view.currentMissionId;
const mission = currentMissionId !== null ? missionStore.getMissionData(currentMissionId) : null;

const zones = computed(() => currentMissionId !== null ? missionStore.getZoneData(currentMissionId, props.zoneType) : [])


// Toggle Eye Icon for specific zone
const toggleVisibility = (zoneID: number) => {
  // const zone = zones.value.find(zone => zone.id === zoneID)
  // if (zone) zone.isVisible = !zone.isVisible
}
console.log(zones)
</script>

<template>
  <Card class="p-2 m-2 relative">
    <!-- Zone Card Title -->
    <CardTitle class="text-x2 font-bold flex items-center">
      Keep {{ zoneType }}
      <Square class="w-5 h-5 ml-3 rounded-sm text-transparent" :class="titleStyles.titleColor[zoneType]" />
    </CardTitle>

    <!-- Zones List -->
    <CardContent class="mt-2 flex flex-col items-start space-y-3">
      <div v-for="(zone, index) in zones" :key="props.zoneType"
        class="flex items-center justify-between w-full pb-1 pt-1">
        <span class="font-semibold">Zone {{ index }}</span>
        <div class="flex gap-x-2">
          <Pencil 
            v-if="mission?.mission_status !== 'Complete'"
            class="w-5 h-5 text-gray-700 hover:text-gray-500 cursor-pointer" 
          />
          <component :is="zone.isVisible ? Eye : EyeOff"
            class="w-5 h-5 text-gray-700 hover:text-gray-500 cursor-pointer" @click="toggleVisibility(zone.id)" />
          <Trash2
            v-if="mission?.mission_status !== 'Complete' && currentMissionId !== null"
            @click="missionStore.deleteZone(currentMissionId, props.zoneType, index)"
            class="w-5 h-5 text-gray-700 hover:text-gray-500 cursor-pointer" 
          />
        </div>
      </div>
    </CardContent>

    <!-- Add Zone Button -->
    <CardFooter v-if="mission?.mission_status !== 'Complete'" class="mt-4 justify-center items-center">
      <Button 
        v-if="currentMissionId !== null"
        @click="missionStore.addZone(currentMissionId, props.zoneType)"
        class="text-fg bg-transparent shadow-none flex flex-col items-center hover:bg-transparent">
        <Plus class="w-5 h-5" />
        Add Zone
      </Button>
    </CardFooter>
  </Card>
</template>
