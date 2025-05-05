<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { emit } from "@tauri-apps/api/event";

import { Button } from "@/components/ui/button";
import { Toaster } from "@/components/ui/sonner";
import { toast } from 'vue-sonner'

// --------- Sync toasts across screens ------ //
const toastMap = new Map<string, string>()
var id: String;

function generateId() {
  return `toast-${Date.now()}-${Math.random()}`;
}

listen('create-toast', (event) => {
  const { id, type, title, description } = event.payload as {
    id: string
    type: 'error' | 'warning' | 'info'
    title: string
    description: string
  }

  const toastId = toast[type](title, {
    id,
    description,
    duration: Infinity,
    action: {
      label: 'Dismiss',
      onClick: () => emit('dismiss-toast', { id }),
    }
  })

  toastMap.set(id, String(toastId))
})

listen('dismiss-toast', (event) => {
  const { id } = event.payload as { id: string }
  toast.dismiss(id)
})

listen('dismiss-all-toasts', (event) => {
  toast.dismiss();
})
</script>

<template>
  <Toaster richColors />

  <!-- --------- Errors --------- -->
  <Button
    variant="outline"
    @click="
      () => {
        id = generateId();
        emit('create-toast', {
          id,
          type: 'error',
          title: 'Error: Connection Failure',
          description: `Unable to connect to \${vehicle}`,
        })
      }
    "
  >
    Connection Error
  </Button>
  <Button
    variant="outline"
    @click="
      () => {
        id = generateId();
        emit('create-toast', {
          id,
          type: 'error',
          title: 'Error: Abnormal Status',
          description: `Abnormal \${vehicle} status (low battery, system error/failure)!`,
        })
      }
    "
  >
    Abnormal Status Error
  </Button>

  <!-- --------- Warnings --------- -->
  <Button
    variant="outline"
    @click="
      () => {
        id = generateId();
        emit('create-toast', {
          id,
          type: 'warning',
          title: 'Warning: Signal Integrity',
          description: `Weak signal integrity/connection lost to \${vehicle}!`,
        })
      }
    "
  >
    Signal Integrity Warning
  </Button>
  <Button
    variant="outline"
    @click="
      () => {
        id = generateId();
        emit('create-toast', {
          id,
          type: 'warning',
          title: 'Warning: Keep-Out',
          description: `\${vehicle} within \${distance} ft of keep-out zone!`,
        })
      }
    "
  >
    Keep-Out Warning
  </Button>
  <Button
    variant="outline"
    @click="
      () => {
        id = generateId();
        emit('create-toast', {
          id,
          type: 'warning',
          title: 'Warning: Vehicle Proximity',
          description: `\${vehicle1} and \${vehicle2} are within \${distance} ft of each other!`,
        })
      }
    "
  >
    Proximity Warning
  </Button>

  <!-- --------- Dismiss All --------- -->
  <Button
    variant="outline"
    @click="
      () => {
        emit('dismiss-all-toasts')
      }
    "
    >Dismiss All
  </Button>
</template>
