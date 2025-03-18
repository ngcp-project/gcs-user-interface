<script setup lang="ts">
import { ref, computed } from 'vue'
import { Card, CardContent, CardTitle, CardFooter } from '@/components/ui/card'
import { Button } from '@/components/ui/button'

// Define Props
defineProps<{
  vehicleName: 'ERU' | 'MEA' | 'MRA'
  patientStatus: 'Secured' | 'Unsecured'
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
    'Secured': 'text-chart-2 font-semibold',
    'Unsecured': 'text-destructive font-semibold'
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
  <Card class="p-2 m-2 relative">
    <!-- Vehicle Name -->
    <CardTitle class="text-xl font-bold">{{ vehicleName }}</CardTitle>

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
        :disabled="currentStageIndex === stages.length - 1"
      >
        Next Stage
      </Button>
    </CardFooter>
  </Card>
</template>
