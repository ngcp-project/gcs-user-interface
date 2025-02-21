<script setup lang="ts">
import { Card, CardContent, CardTitle } from '@/components/ui/card'
import { computed, ref } from 'vue'
import { Trash2, Eye, EyeOff, Pencil } from 'lucide-vue-next'

defineProps<{
  stageName: string
  status: 'Incomplete' | 'Completed' | 'In Progress'
}>()

// Status Styles
const statusStyles = computed(() => ({
  statusColor: {
    'Completed': 'text-green-700 font-semibold',
    'In Progress': 'text-yellow-700 font-semibold',
    'Incomplete': 'text-red-700 font-semibold'
  }
}))

// Toggle Eye Icon
const isVisible = ref(true) // Track visibility state

const toggleVisibility = () => {
  isVisible.value = !isVisible.value
}

// Stage click handlers
function deleteStage(stageName: string) {  
  console.log(`Delete button clicked for Mission ${stageName}`)
}
</script>

<template>
  <Card class="w-64 p-2 relative">
    <!-- Trash Icon -->
    <div class="absolute top-2 right-2 cursor-pointer">
      <Trash2 
        @click="deleteStage(stageName)"
        class="w-5 h-5 text-gray-700 hover:text-red-500" />
    </div>

    <!-- Mission Title -->
    <CardTitle class="text-x2 font-bold text-black">{{ stageName }}</CardTitle>

    <!-- Status Section -->
    <CardContent class="mt-2 flex flex-col items-start">
      <span class="font-semibold">
        Status: <span :class="statusStyles.statusColor[status]">{{ status }}</span>
      </span>
      <div class="flex items-center justify-between w-full">
        <span class="font-semibold">Search Area</span>
        <div class="flex gap-x-2">
          <Pencil class="w-5 h-5 text-gray-700 hover:text-gray-500 cursor-pointer" />
          <component 
            :is="isVisible ? Eye : EyeOff" 
            class="w-5 h-5 text-gray-700 hover:text-gray-500 cursor-pointer"
            @click="toggleVisibility"
          />
        </div>
      </div>
    </CardContent>
  </Card>
</template>
