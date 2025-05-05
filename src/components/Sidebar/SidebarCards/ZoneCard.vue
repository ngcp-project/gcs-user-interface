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
const currentMissionId = missionStore.view.currentMissionId ? missionStore.view.currentMissionId : null;
const mission = currentMissionId !== null ? missionStore.getMissionData(currentMissionId) : null;

const zones = computed(() =>
  currentMissionId !== null ? missionStore.getZoneData(currentMissionId, props.zoneType) : []
);

// Toggle Eye Icon for specific zone
const toggleVisibility = (zoneID: number) => {
  // implement
};

const handleNewZone = () => {
  if (currentMissionId === null) return;
  missionStore.addZone(currentMissionId, props.zoneType);
  
};

const handleDeleteZone = (index: number) => {
  if (currentMissionId === null) return;
  missionStore.deleteZone(currentMissionId, props.zoneType, index);
  mapStore.removeZoneLayer(currentMissionId, props.zoneType, index);
};

const testZoneFunc = (index: number) => {
  // console.log(mapStore)
  if (currentMissionId === null) return;
  mapStore.updateZonePolygon(currentMissionId, props.zoneType, index)
};

const checkEditMode = (index: number) => {
  if (currentMissionId === null) return false;
  const zoneLayers = mapStore.getZoneLayers(currentMissionId, props.zoneType);
  const zone = zoneLayers[index];
  return zone.layer.once("pm:edit", () => {
    return true
  })
}

const editToggle = async (zone: GeoCoordinateStruct[], index: number) => {
  if (currentMissionId === null) return;
  const zoneLayers = mapStore.getZoneLayers(currentMissionId, props.zoneType);
  const zoneLayer = zoneLayers[index];

  zoneLayer.layer.on("pm:edit", () => {
    return Check
  })

  if (zone.length > 0) {
    return Pencil
  } else {
    return Plus
  }
  
}

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
          <!-- TODO: Add ui to confirm an edit -->
          <component
            :is="zone.length > 0 ? Pencil : Plus"
            class="h-5 w-5 cursor-pointer text-gray-700 hover:text-gray-500"
            @click="testZoneFunc(index)"
          />
          <Eye 
            @click="() => {}"
            class="h-5 w-5 cursor-pointer text-gray-700 hover:text-gray-500" 
          />
          <Trash2
            @click="handleDeleteZone(index)"
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
