<script setup lang="ts">
import { Card, CardContent, CardTitle, CardFooter } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { computed, ref } from 'vue'
import { Trash2, Eye, EyeOff, Pencil, Square, Plus } from 'lucide-vue-next'

defineProps<{
  zoneType: 'In' | 'Out'
}>()

// Title Styles
const titleStyles = computed(() => ({
  titleColor: {
    'In': 'bg-chart-2',
    'Out': 'bg-destructive'
  }
}))

// Track zones
const zones = ref<{ id: number; isVisible: boolean }[]>([]) // Store zone items with visibility

// Function to add a new zone
const addZone = () => {
  const newZone = { id: Date.now(), isVisible: true } // Unique ID with visibility state
  zones.value.push(newZone)
}

// Function to delete a zone by ID
const deleteZone = (zoneID: number) => {
  zones.value = zones.value.filter(zone => zone.id !== zoneID)
}

// Toggle Eye Icon for specific zone
const toggleVisibility = (zoneID: number) => {
  const zone = zones.value.find(zone => zone.id === zoneID)
  if (zone) zone.isVisible = !zone.isVisible
}
</script>

<template>
  <Card class="p-2 m-2 relative">
    <!-- Mission Title -->
    <CardTitle class="text-x2 font-bold flex items-center">
      Keep {{ zoneType }}
      <Square class="w-5 h-5 ml-3 rounded-sm text-transparent" :class="titleStyles.titleColor[zoneType]" />
    </CardTitle>

    <!-- Zones List -->
    <CardContent class="mt-2 flex flex-col items-start space-y-3">
      <div v-for="zone in zones" :key="zone.id" class="flex items-center justify-between w-full pb-1 pt-1">
        <span class="font-semibold">Zone {{ zone.id }}</span>
        <div class="flex gap-x-2">
          <Pencil class="w-5 h-5 text-gray-700 hover:text-gray-500 cursor-pointer" />
          <component 
            :is="zone.isVisible ? Eye : EyeOff" 
            class="w-5 h-5 text-gray-700 hover:text-gray-500 cursor-pointer"
            @click="toggleVisibility(zone.id)"
          />
          <Trash2 
            @click="deleteZone(zone.id)" 
            class="w-5 h-5 text-gray-700 hover:text-destructive cursor-pointer"
          />
        </div>
      </div>
    </CardContent>

    <!-- Add Zone Button -->
    <CardFooter class="mt-4 justify-center items-center">
      <Button 
        @click="addZone"
        class="bg-transparent shadow-none flex flex-col items-center hover:bg-transparent"
      >
        <Plus class="w-5 h-5" />
        Add Zone
      </Button>
    </CardFooter>
  </Card>
</template>
