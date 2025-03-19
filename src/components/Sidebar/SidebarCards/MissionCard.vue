<script setup lang="ts">
import { Card, CardContent, CardTitle, CardFooter } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { computed, defineEmits } from "vue";
import { Trash2 } from "lucide-vue-next";
import { missionStore } from "@/lib/MissionStore";
import { MissionStruct } from "@/lib/bindings";
import { ClientMission } from "@/lib/MissionStore.types";

const props = defineProps<{
  missionId: number;
}>();

const mission = computed(() => missionStore.getMissionData(props.missionId));

// Status Styles
const statusStyles = computed(() => ({
  statusColor: {
    Inactive: "text-muted-foreground font-semibold",
    Failed: "text-destructive font-semibold",
    Active: "text-chart-4 font-semibold",
    Complete: "text-chart-2 font-semibold"
  }
}));
</script>

<template>
  <Card v-if="mission" class="relative m-2 bg-sidebar-foreground p-2 text-foreground">
    <!-- Trash Icon -->
    <div
      v-if="'isSubmitted' in mission && mission.isSubmitted === false"
      class="absolute right-2 top-2 cursor-pointer"
    >
      <Trash2
        @click.stop="missionStore.view.deleteMission(mission.mission_id)"
        class="h-5 w-5 text-foreground hover:text-destructive"
      />
    </div>

    <!-- Mission Title -->
    <CardTitle class="text-x2 font-bold">{{ mission.mission_name }}</CardTitle>

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
      <Button
        class="mr-2"
        :disabled="
          mission.mission_status === 'Active' ||
          mission.mission_status === 'Complete' ||
          mission.mission_status === 'Failed'
        "
        @click.stop="missionStore.submitMission(mission.mission_id)"
      >
        Submit
      </Button>
      <Button @click.stop> Zones </Button>
    </CardFooter>
  </Card>
</template>
