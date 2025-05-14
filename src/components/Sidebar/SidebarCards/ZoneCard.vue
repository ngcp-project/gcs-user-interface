<script setup lang="ts">
import { Card, CardContent, CardTitle, CardFooter } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { computed } from "vue";
import { Trash2, Eye, EyeOff, Pencil, Square, Plus, Check } from "lucide-vue-next";
import { missionStore } from "@/lib/MissionStore";
import { GeoCoordinateStruct, ZoneType } from "@/lib/bindings";
import mapStore from "@/lib/MapStore";

const props = defineProps<{
  zoneType: ZoneType;
}>();

// Title Color Styles
const titleStyles = {
  titleColor: {
    In: "bg-chart-2",
    Out: "bg-destructive"
  }
}

const zoneType = {
  KeepIn: "In",
  KeepOut: "Out"
}[String(props.zoneType) || "KeepIn"] as "In" | "Out";

// Track zones
const currentMissionId = missionStore.view.currentMissionId
  ? missionStore.view.currentMissionId
  : null;
const mission = currentMissionId !== null ? missionStore.getMissionData(currentMissionId) : null;

const zones = computed(() =>
  currentMissionId !== null ? missionStore.getZoneData(currentMissionId, props.zoneType) : []
);

const zoneLayer = computed(() => {
  console.log("update");
  if (currentMissionId === null) return null;
  return mapStore.getZoneLayers(currentMissionId, props.zoneType);
});

// Toggle Eye Icon for specific zone
const toggleVisibility = (zoneID: number) => {
  if (currentMissionId === null) return;
  mapStore.setLayerVisibility(currentMissionId, props.zoneType);
};

const handleNewZone = () => {
  if (currentMissionId === null) return;
  missionStore.addZone(currentMissionId, props.zoneType);
};

const handleDeleteZone = (index: number) => {
  if (currentMissionId === null) return;
  missionStore.deleteZone(currentMissionId, props.zoneType, index);
};

const handleCreateZone = (index: number) => {
  if (currentMissionId === null) return;
  mapStore.updateZonePolygon(currentMissionId, props.zoneType, index);
};

const zoneVisibility = (index: number) =>
  computed(() => {
    const test = mapStore;
    console.log("zoneVisibility", zoneLayer.value, index);
    return zoneLayer.value && zoneLayer.value[index]?.properties?.visibility;
  });
</script>

<template>
  <Card class="relative m-2 p-2">
    <!-- Zone Card Title -->
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
        <div class="flex gap-x-2">
          <!-- TODO: Add ui to confirm an edit -->
          <component
            :is="zone.length > 0 ? Pencil : Plus"
            v-if="mission?.mission_status !== 'Complete'"
            class="h-5 w-5 cursor-pointer text-gray-700 hover:text-gray-500"
            @click="handleCreateZone(index)"
          
          />
          <component
            :is="zoneVisibility(index) ? Eye : EyeOff"
            @click="() => toggleVisibility(index)"
            class="h-5 w-5 cursor-pointer text-gray-700 hover:text-gray-500"
          />
          <Trash2
           
            v-if="mission?.mission_status !== 'Complete' && currentMissionId !== null"
            @click="handleDeleteZone(index)"
            class="h-5 w-5 cursor-pointer text-gray-700 hover:text-gray-500"
          
          />
        </div>
      </div>
    </CardContent>

    <!-- Add Zone Button -->
    <CardFooter
      v-if="mission?.mission_status !== 'Complete'"
      class="mt-4 items-center justify-center"
    >
      <Button
        
        v-if="currentMissionId !== null"
        @click="handleNewZone()"
        class="text-fg flex flex-col items-center bg-transparent shadow-none hover:bg-transparent"
      >
        <Plus class="h-5 w-5" />
        Add Zone
      </Button>
    </CardFooter>
  </Card>
</template>
