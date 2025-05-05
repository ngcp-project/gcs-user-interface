<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { toast } from 'vue-sonner'
</script>

<template>
  <!-- Errors -->
  <Button
    variant="outline" @click="() => {
      toast.error('Error: Connection Failure', {
        description: `Unable to connect to \${vehicle}!`,
        action: {
          label: 'Dismiss',
          onClick: () => console.log('Undo'),
        },
        duration: Infinity,
      })
    }"
  >
  Connection Error
  </Button>
  <Button
    variant="outline" @click="() => {
      toast.error('Error: Abnormal Status', {
        description: `Abnormal \${vehicle} status (low battery, system error/failure)!`,
        action: {
          label: 'Dismiss',
          onClick: () => console.log('Undo'),
        },
        duration: Infinity
      })
    }"
  >
  Abnormal Status Error
  </Button>

  <!-- Warnings -->
  <Button
    variant="outline" @click="() => {
      toast.warning('Warning: Signal Integrity', {
        description: `Weak signal integrity/connection lost to \${vehicle}!`,
        action: {
          label: 'Dismiss',
          onClick: () => console.log('Undo'),
        },
        duration: Infinity
      })
    }"
  >
  Signal Integrity Warning
  </Button>
  <Button
    variant="outline" @click="() => {
      toast.warning('Warning: Keep-Out', {
        description: `\${vehicle} within \${distance} ft of keep-out zone!`,
        action: {
          label: 'Dismiss',
          onClick: () => console.log('Undo'),
        },
        duration: Infinity
      })
    }"
  >
  Keep-Out Warning
  </Button>
  <Button
    variant="outline" @click="() => {
      toast.warning('Warning: Vehicle Proximity', {
        description: `\${vehicle1} and \${vehicle2} are within \${distance} ft of each other!`,
        action: {
          label: 'Dismiss',
          onClick: () => console.log('Undo'),
        },
        duration: Infinity
      })
    }"
  >
  Proximity Warning
  </Button>
  <Button
    variant="outline" @click="() => {
      toast.dismiss()
    }"
  >Dismiss All
  </Button>
</template>