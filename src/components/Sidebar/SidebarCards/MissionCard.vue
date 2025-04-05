<script setup lang="ts">
import { Card, CardContent, CardTitle, CardFooter } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { computed } from "vue";
import { Trash2 } from "lucide-vue-next";
import { missionStore } from "@/lib/MissionStore";

const props = defineProps<{
  missionId: number;
}>()

// Status Styles
const statusStyles = {
  statusColor: {
    Inactive: "text-muted-foreground font-semibold",
    Failed: "text-destructive font-semibold",
    Active: "text-chart-4 font-semibold",
    Complete: "text-chart-2 font-semibold"
  }
};

const mission = computed(() => missionStore.getMissionData(props.missionId));

const handleZoneClick = () => {
  missionStore.setCurrentMissionID(props.missionId)
  missionStore.setCurrentView("zone")
}

</script>

<template>
  <Card v-if="mission" class="relative m-2 bg-sidebar-foreground p-2 text-foreground">

    <!-- Mission Title -->
    <CardTitle class="flex items-center gap-2">
      <Input class="flex-1" v-model="mission.mission_name" />
      <!-- Trash Icon -->
      <div v-if="mission.mission_status == 'Inactive'" class="cursor-pointer">
        <Trash2 @click.stop="" class="h-5 w-5 text-foreground hover:text-destructive" />
      </div>
    </CardTitle>

    <!-- Status Section -->
    <CardContent class="mt-2 flex flex-col items-start">
      <span class="font-semibold">
        Status:
        <span :class="statusStyles.statusColor[mission.mission_status]">{{
          mission.mission_status
        }}</span>
      </span>
    </CardContent>

    <!-- Submit Button -->
    <CardFooter class="mt-4 justify-start">
      <Button class="mr-2" :disabled="mission.mission_status !== 'Inactive'
        " @click.stop="">
        Start
      </Button>
      <Button @click.stop="handleZoneClick"> Zones </Button>
    </CardFooter>
  </Card>
</template>
