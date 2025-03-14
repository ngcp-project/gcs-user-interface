<script setup lang="ts">
import { Card, CardContent, CardTitle, CardFooter } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { computed, defineEmits } from 'vue'
import { Trash2 } from 'lucide-vue-next'

defineProps<{
  missionNumber: number
  status: 'Active' | 'Inactive' | 'Failed' | 'Complete'
}>()

const emit = defineEmits(['deleteMission'])

// Status Styles
const statusStyles = computed(() => ({
  statusColor: {
    'Inactive': 'text-muted-foreground font-semibold',
    'Failed': 'text-destructive font-semibold',
    'Active': 'text-chart-4 font-semibold',
    'Complete': 'text-chart-2 font-semibold'
  }
}))

// Emit delete event to parent
function deleteMission(missionNumber: number) {
  emit('deleteMission', missionNumber)
}
</script>

<template>
  <Card class="p-2 m-2 relative bg-sidebar-foreground text-foreground">
    <!-- Trash Icon -->
    <div class="absolute top-2 right-2 cursor-pointer">
      <Trash2 
        @click="deleteMission(missionNumber)"
        class="w-5 h-5 text-secondary hover:text-destructive" 
      />
    </div>

    <!-- Mission Title -->
    <CardTitle class="text-x2 font-bold">Mission {{ missionNumber }}</CardTitle>

    <!-- Status Section -->
    <CardContent class="mt-2 flex flex-col items-start">
      <span class="font-semibold">
        Status: <span :class="statusStyles.statusColor[status]">{{ status }}</span>
      </span>
    </CardContent>

    <!-- Submit Button -->
    <CardFooter class="mt-4 justify-start">
      <Button 
        class="mr-2"
        :disabled="status === 'Active'||status === 'Complete'||status === 'Failed'"
      >
        Submit
      </Button>
      <Button
      >
        Zones
      </Button>
    </CardFooter>
  </Card>
</template>
