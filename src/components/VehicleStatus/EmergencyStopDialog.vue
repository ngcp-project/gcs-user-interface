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
          Send STOP command to
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

const props = defineProps<{
  vehicleName: string;
}>();

const vehicle_names = ["ERU", "MEA", "MRA", "FRA"];

function sendStopCommand() {
  const promises: any[] = [];
  if (props.vehicleName == "all") {
    // send Emergency Stop command for all vehicles
    vehicle_names.forEach((name) => {
      const promise = fetch("http://localhost:5135/EmergencyStop", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({ Key: name })
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error("Network response was not ok");
          }
          return response.json();
        })
        .then((data) => console.log(data))
        .catch((error) => console.error("Error sending stop command:", error));
      promises.push(promise);
    });

    Promise.all(promises)
      .then((data) => {
        console.log("Sent stop commands to all four vehicles!");
      })
      .catch((error) => {
        console.error("Error sending stop command:", error);
      });
  } else {
    // send Emergency Stop command for specific vehicle
    fetch("http://localhost:5135/EmergencyStop", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({ Key: props.vehicleName })
    })
      .then((response) => {
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
        return response.json();
      })
      .then((data) => console.log(data))
      .catch((error) => console.error("Error sending stop command:", error));
  } //end else
}
</script>
