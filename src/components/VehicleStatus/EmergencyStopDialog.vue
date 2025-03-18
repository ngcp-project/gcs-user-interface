<template>
  <Dialog>
    <DialogTrigger as-child>
      <slot>
        <Button variant="destructive">Stop All</Button>
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
          </span>
          ?
        </div>
      </slot>

      <DialogFooter>
        <DialogClose><Button variant="destructive">No</Button></DialogClose>
        <DialogClose><Button @click="sendStopCommand">Yes</Button></DialogClose>
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
import { invoke } from '@tauri-apps/api/tauri';

const props = defineProps<{
  vehicleName: string;
}>();

const vehicle_names = ["ERU", "MEA", "MRA", "FRA"];

function sendStopCommand() {
  if (props.vehicleName === "all") {
    // For "all", iterate through all vehicles and invoke the command.
    vehicle_names.forEach((name) => {
      invoke("emergency_stop", { vehicle: name })
        .then((res) => console.log("Ack for", name, res))
        .catch((err) => console.error("Error sending stop command for", name, err));
    });
  } else {
    // For a specific vehicle, invoke the emergency_stop command.
    invoke("emergency_stop", { vehicle: props.vehicleName })
      .then((res) => console.log("Ack for", props.vehicleName, res))
      .catch((err) => console.error("Error sending stop command:", err));
  }
}
</script>
