<script setup lang="ts">
import { ref, computed } from 'vue'
import { Card, CardContent, CardTitle, CardFooter } from '@/components/ui/card'
import { Button } from '@/components/ui/button'

// Define Props
defineProps<{
  vehicleName: 'ERU' | 'MEA' | 'MRA'
  patientStatus: 'Secured' | 'Not Secured'
}>()

// Temp stages for the vehicle
const stages = ['Search', 'Rescue', 'Transport', 'Completed']

// Stage index
const currentStageIndex = ref(0)

// Computed property for current stage name
const stageName = computed(() => stages[currentStageIndex.value])

// Status Styles
const patientStatusStyles = computed(() => ({
  statusColor: {
    'Secured': 'text-green-700 font-semibold',
    'Not Secured': 'text-red-700 font-semibold'
  }
}))

// Move to the next stage
function nextStage() {
  if (currentStageIndex.value < stages.length - 1) {
    currentStageIndex.value++
  }
}
</script>

<template>
  <Card class="w-64 p-2 relative">
    <!-- Vehicle Name -->
    <CardTitle class="text-xl font-bold text-black">{{ vehicleName }}</CardTitle>

    <!-- Vehicle Stage & Patient Status -->
    <CardContent class="mt-2 flex flex-col items-start">
      <span class="font-semibold">
        Stage: {{ stageName }}
      </span>
      <span class="font-semibold">
        Patient Status: 
        <span :class="patientStatusStyles.statusColor[patientStatus]">
          {{ patientStatus }}
        </span>
      </span>
    </CardContent>

    <!-- Next Stage Button -->
    <CardFooter class="mt-4 justify-start">
      <Button 
        @click="nextStage"
        class="bg-gray-200 text-gray-700 hover:bg-gray-300"
        :disabled="currentStageIndex === stages.length - 1"
      >
        Next Stage
      </Button>
    </CardFooter>
  </Card>
</template>
