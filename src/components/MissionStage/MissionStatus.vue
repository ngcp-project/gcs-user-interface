<template>
  <div
    class="flex flex-col items-start justify-center rounded-sm border bg-secondary p-2 font-semibold text-secondary-foreground"
  >
    <div class="flex w-full items-center justify-between gap-2">
      <div>
        <span class="opacity-80">Mission: </span>
        {{ MISSION_INFO.missionName === "" ? "null" : MISSION_INFO.missionName }}
      </div>
      <Dialog>
        <DialogTrigger asChild>
          <NgButton size="xs">OPEN</NgButton>
        </DialogTrigger>
        <DialogContent>
          <MissionDropdown :missionNumber="missionNumber" @close="closePopup()" />
        </DialogContent>
      </Dialog>
    </div>
    <div class="flex w-full items-center gap-2">
      <span class="opacity-80">Current Stage: </span><span>{{ getLastStage() ?? "null" }}</span>
    </div>
  </div>
</template>

<script lang="ts">
import { inject } from "vue";
import MissionDropdown from "./MissionDropdown.vue";

import { NgButton } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { SearchCoordsProvider, defaultSearchCoords } from "@/types/search-coords-provider";
import { TargetCoordsProvider, defaultTargetCoords } from "@/types/target-coords.provider";
import { MissionInfoProvider } from "@/types/mission-info-provider";

import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";

export default {
  setup() {
    const { selectingTarget } =
      inject<TargetCoordsProvider>("target-coords-provider") ?? defaultTargetCoords;
    const { selectingSearch } =
      inject<SearchCoordsProvider>("search-coords-provider") ?? defaultSearchCoords;
    const { MISSION_INFO, getStageNames } = inject<MissionInfoProvider>("mission-info-provider")!;

    return { selectingTarget, selectingSearch, MISSION_INFO, getStageNames };
  },

  name: "MissionStatus",
  data() {
    return {
      showPopup: false
    };
  },
  components: {
    MissionDropdown,
    // eslint-disable-next-line vue/no-reserved-component-names
    Dialog,
    DialogContent,
    DialogTrigger,
    NgButton
  },
  props: {
    missionNumber: {
      type: Number,
      required: true
    },
    status: {
      type: String,
      required: true
    }
  },
  methods: {
    // make sure to set selectingTarget back to false
    closePopup() {
      this.selectingTarget = false;
      this.selectingSearch = false;
      this.showPopup = false;
    },
    getLastStage() {
      // console.log(this.MISSION_INFO['stages'][this.MISSION_INFO['stages'].length - 1]);
      const stage_names = this.getStageNames();

      return stage_names[stage_names.length - 1];
    },
    cn
  }
};
</script>
<style scoped>
.popup {
  position: fixed;
  top: 70%;
  left: 68%;
  transform: translate(-50%, -50%);
  background: rgba(0, 0, 0, 0.5);
  padding: 20px;
  z-index: 1000;
}
</style>
