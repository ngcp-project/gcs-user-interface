<script setup lang="ts">
import { Card, CardContent, CardTitle, CardFooter } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { computed } from "vue";
import { Trash2, Eye, EyeOff, Pencil, Square, Plus } from "lucide-vue-next";
import { missionStore } from "@/lib/MissionStore";
import { ZoneType } from "@/lib/bindings";
import mapStore from "@/lib/MapStore";

const props = defineProps<{
  zoneType: ZoneType;
}>();

// Title Styles
const titleStyles = computed(() => ({
  titleColor: {
    In: "bg-chart-2",
    Out: "bg-destructive"
  }
}));

const zoneType = {
  KeepIn: "In",
  KeepOut: "Out"
}[String(props.zoneType) || "KeepIn"] as "In" | "Out";

// Track zones
const currentMissionId = missionStore.view.currentMissionId;
const mission = currentMissionId !== null ? missionStore.getMissionData(currentMissionId) : null;

const zones = computed(() =>
  currentMissionId !== null ? missionStore.getZoneData(currentMissionId, props.zoneType) : []
);

// Toggle Eye Icon for specific zone
const toggleVisibility = (zoneID: number) => {
  // const zone = zones.value.find(zone => zone.id === zoneID)
  // if (zone) zone.isVisible = !zone.isVisible
};

const handleNewZone = () => {
  if (currentMissionId === null) return;
  missionStore.addZone(currentMissionId, props.zoneType);
  mapStore.addZonePolygon(props.zoneType);
};
</script>

<template>
  <Card class="relative m-2 p-2">
    <!-- Mission Title -->
    <CardTitle class="text-x2 flex items-center font-bold">
      Keep {{ zoneType }}
      <Square
        class="ml-3 h-5 w-5 rounded-sm text-transparent"
        :class="titleStyles.titleColor[zoneType || 'In']"
      />
    </CardTitle>

    <!-- Zones List -->
    <CardContent class="mt-2 flex flex-col items-start space-y-3">
      <div
        v-for="(zone, index) in zones"
        :key="props.zoneType"
        class="flex w-full items-center justify-between pb-1 pt-1"
      >
        <span class="font-semibold">Zone {{ index }}</span>
        <div v-if="mission?.mission_status === 'Inactive'" class="flex gap-x-2">
          <Pencil class="h-5 w-5 cursor-pointer text-gray-700 hover:text-gray-500" />
          <component
            :is="zone.isVisible ? Eye : EyeOff"
            class="h-5 w-5 cursor-pointer text-gray-700 hover:text-gray-500"
            @click="toggleVisibility(zone.id)"
          />
          <Trash2
            @click="missionStore.deleteZone(currentMissionId, props.zoneType, index)"
            class="h-5 w-5 cursor-pointer text-gray-700 hover:text-gray-500"
          />
        </div>
      </div>
    </CardContent>

    <!-- Add Zone Button -->
    <CardFooter
      v-if="mission?.mission_status === 'Inactive'"
      class="mt-4 items-center justify-center"
    >
      <Button
        @click="handleNewZone()"
        class="text-fg flex flex-col items-center bg-transparent shadow-none hover:bg-transparent"
      >
        <Plus class="h-5 w-5" />
        Add Zone
      </Button>
    </CardFooter>
  </Card>
</template>
