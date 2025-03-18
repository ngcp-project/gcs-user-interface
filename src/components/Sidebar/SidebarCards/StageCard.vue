<script setup lang="ts">
import { Card, CardContent, CardTitle } from '@/components/ui/card'
import { computed, ref } from 'vue'
import { Trash2, Eye, EyeOff, Pencil } from 'lucide-vue-next'

defineProps<{
  stageName: string
  status: 'Incomplete' | 'Complete' | 'In Progress'
}>()

const emit = defineEmits(['deleteStage'])

// Status Styles
const statusStyles = computed(() => ({
  statusColor: {
    'Complete': 'text-chart-2 font-semibold',
    'In Progress': 'text-chart-4 font-semibold',
    'Incomplete': 'text-destructive font-semibold'
  }
}))

// Toggle Eye Icon
const isVisible = ref(true) // Track visibility state

const toggleVisibility = () => {
  isVisible.value = !isVisible.value
}

// Stage click handlers
function deleteStage(stageNumber: string) {
  emit('deleteStage', stageNumber)
}
</script>

<template>
  <Card class="p-2 m-2 relative">
    <!-- Trash Icon -->
    <div class="absolute top-2 right-2 cursor-pointer">
      <Trash2 
        @click="deleteStage(stageName)"
        class="w-5 h-5 text-foreground hover:text-destructive" />
    </div>

    <!-- Mission Title -->
    <CardTitle class="text-x2 font-bold">{{ stageName }}</CardTitle>

    <!-- Status Section -->
    <CardContent class="mt-2 flex flex-col items-start">
      <span class="font-semibold">
        Status: <span :class="statusStyles.statusColor[status]">{{ status }}</span>
      </span>
      <div class="flex items-center justify-between w-full">
        <span class="font-semibold">Search Area</span>
        <div class="flex gap-x-2">
          <Pencil class="w-5 h-5 text-secondary-foreground hover:text-secondary-foreground/80 cursor-pointer" />
          <component 
            :is="isVisible ? Eye : EyeOff" 
            class="w-5 h-5 text-secondary-foreground hover:text-secondary-foreground/80 cursor-pointer"
            @click="toggleVisibility"
          />
        </div>
      </div>
    </CardContent>
  </Card>
</template>
