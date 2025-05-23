<template>
  <Dialog>
    <DialogTrigger as-child>
      <slot>
        <Button variant="destructive">EMERGENCY STOP ALL</Button>
      </slot>
    </DialogTrigger>
    <DialogContent>
      <DialogHeader>
        <DialogTitle class="flex items-center gap-2">
          <Icon
            icon="material-symbols-light:emergency-home-rounded"
            class="h-[30px] w-[30px] text-red-500"
          />
          <slot name="header">Emergency Stop</slot>
        </DialogTitle>
      </DialogHeader>
      <slot name="body">
        <div class="text-xl font-medium">
          Send Stop Command to
          <span class="font-bold text-red-500">
            <span v-if="vehicleName == 'all'"><span class="">ALL</span> vehicles</span>
            <span v-else>
              vehicle <span>{{ vehicleName }}</span>
            </span>
          </span>?
        </div>
      </slot>

      <DialogFooter>
        <DialogClose><Button>No</Button></DialogClose>
        <DialogClose><Button variant="destructive" @click="sendStopCommand">Yes</Button></DialogClose>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from "@/components/ui/dialog";
import { Icon } from "@iconify/vue";
import { createTauRPCProxy } from "@/lib/bindings";

const { vehicleName } = defineProps<{ vehicleName: string }>();

const vehicle_names = ["ERU", "MEA", "MRA", "FRA"];
const commands = createTauRPCProxy().commands;

async function sendStopCommand() {
  try {
    if (vehicleName == "all") {
      // Send Emergency Stop command for all vehicles
      const promises = vehicle_names.map(name => 
        commands.send_emergency_stop(name)
      );
      await Promise.all(promises);
      console.log("Sent stop commands to all vehicles!");
    } else {
      // Send Emergency Stop command for specific vehicle
      await commands.send_emergency_stop(vehicleName);
      console.log(`Sent stop command to vehicle ${vehicleName}`);
    }
  } catch (error) {
    console.error("Error sending stop command:", error);
  }
}
</script>