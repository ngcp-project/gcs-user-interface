<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Toaster } from "@/components/ui/toast";
import { useToast } from "@/components/ui/toast/use-toast";

const { toast } = useToast();
</script>

<template>
  <!-- Duration 0 prevents toast from automatically disappearing -->
  <Toaster class="z-50" :duration="0" />
  <Button
    @click="
      () => {
        toast({
          title: 'Error: Connection Failure',
          description: `Unable to connect to \${vehicle}!`
        });
      }
    "
  >
    Connection Error
  </Button>
  <Button
    @click="
      () => {
        toast({
          title: 'Error: Abnormal Status',
          description: `Abnormal \${vehicle} status (low battery, system error/failure)!`
        });
      }
    "
  >
    Abnormal Status Error
  </Button>
  <Button
    @click="
      () => {
        toast({
          title: 'Warning: Signal Integrity',
          description: `Weak signal integrity/connection lost to \${vehicle}!`
        });
      }
    "
  >
    Signal Integrity Warning
  </Button>
  <Button
    @click="
      () => {
        toast({
          title: 'Warning: Keep-Out',
          description: `\${vehicle} within \${distance} ft of keep-out zone!`
        });
      }
    "
  >
    Keep-Out Warning
  </Button>
  <Button
    @click="
      () => {
        toast({
          title: 'Warning: Vehicle Proximity',
          description: `\${vehicle1} and \${vehicle2} are within \${distance} ft of each other!`
        });
      }
    "
  >
    Proximity Warning
  </Button>
</template>
