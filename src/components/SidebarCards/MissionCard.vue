<script setup lang="ts">
import { Card, CardContent, CardTitle, CardFooter } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { computed } from 'vue'
import { Trash2 } from 'lucide-vue-next'

defineProps<{
  missionNumber: number
  status: 'Not Started' | 'Initialized' | 'In Progress' | 'Complete'
}>()

// Status Styles
const statusStyles = computed(() => ({
  statusColor: {
    'Not Started': 'text-gray-700 font-semibold',
    'Initialized': 'text-red-700 font-semibold',
    'In Progress': 'text-yellow-700 font-semibold',
    'Complete': 'text-green-700 font-semibold'
  }
}))

// Mission button click handler
function startMission(missionNumber: number) {
  console.log(`Start button clicked for Mission ${missionNumber}`)
}

function deleteMission(missionNumber: number) {
  console.log(`Delete button clicked for Mission ${missionNumber}`)
}
</script>

<template>
  <Card class="w-64 p-2 relative ">
    <!-- Trash Icon -->
    <div class="absolute top-2 right-2 cursor-pointer">
      <Trash2 
      @click = "deleteMission(missionNumber)"
      class="w-5 h-5 text-gray-700 hover:text-red-500" />
    </div>

    <!-- Mission Title -->
    <CardTitle class="text-x2 font-bold text-black">Mission {{ missionNumber }}</CardTitle>

    <!-- Status Section -->
    <CardContent class="mt-2 flex flex-col items-start">
      <span class="font-semibold">
        Status: <span :class="statusStyles.statusColor[status]">{{ status }}</span>
      </span>
    </CardContent>

    <!-- Start Button -->
    <CardFooter class="mt-4 justify-start">
      <Button 
        @click="startMission(missionNumber)" 
        class="bg-gray-200 text-gray-700 hover:bg-gray-300"
        :disabled="status === 'In Progress'"
      >
        Start
      </Button>
    </CardFooter>

  </Card>
</template>
