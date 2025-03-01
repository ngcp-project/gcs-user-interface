<template>
  <div class="coordinate-form">
    <h2>{{ MISSION_INFO?.missionName }}</h2>
    <form @submit.prevent="submitCoordinates()">
      <!-- Dropdown to show all stages -->
      <label for="stage">Stage:</label>
      <Select id="stage" v-model="selectedStage">
        <SelectTrigger>
          <SelectValue placeholder="Select a stage" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem v-for="(stage, index) in getStageNames()" :key="index" :value="stage">
              {{ stage }}
            </SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>

      <!-- Dropdown to show all vehicles -->
      <label for="vehicle">Vehicle:</label>
      <Select id="vehicle" v-model="selectedVehicle">
        <SelectTrigger>
          <SelectValue placeholder="Select a stage" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem v-for="vehicle in vehicles" :key="vehicle" :value="vehicle">
              {{ vehicle }}
            </SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
      <div style="display: grid; gap: 10px">
        <label for="targetCoordinate">Target Coordinate: </label>
        <NgInput id="targetCoordinate" v-model="populateTarget" type="text" />
        <Button @click.prevent="selectTargetCoordinate" size="sm" variant="secondary">
          {{ target_button_text }}
        </Button>

        <label for="searchArea">Search Area:</label>
        <NgInput id="searchArea" v-model="populateSearch" type="text" />
        <Button @click.prevent="selectSearchArea" size="sm" variant="secondary">
          {{ search_button_text }}
        </Button>
      </div>
      <Button type="submit" @click="printMISSION_INFO()">Submit</Button>
    </form>
  </div>
</template>

<script lang="ts">
import { inject } from "vue";
import { SearchCoordsProvider } from "@/types/search-coords-provider";
import { TargetCoordsProvider } from "@/types/target-coords.provider";
import { MissionInfoProvider } from "@/types/mission-info-provider";

import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue
} from "@/components/ui/select";

import { Button } from "../ui/button";
import { NgInput } from "../ui/input";

export default {
  components: {
    Button,
    NgInput,
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectTrigger,
    SelectValue
  },
  setup() {
    const { searchCoords, selectingSearch } =
      inject<SearchCoordsProvider>("search-coords-provider")!;
    const { targetCoord, selectingTarget } =
      inject<TargetCoordsProvider>("target-coords-provider")!;
    const { MISSION_INFO, addStage, updateSearchArea, updateTarget, getStageNames, getStageInfo } =
      inject<MissionInfoProvider>("mission-info-provider")!;

    return {
      searchCoords,
      selectingSearch,
      selectingTarget,
      targetCoord,
      MISSION_INFO,
      addStage,
      updateSearchArea,
      updateTarget,
      getStageNames,
      getStageInfo
    };
  },
  data() {
    return {
      selectedStage: undefined as string | undefined,
      selectedVehicle: "ERU",
      vehicles: ["ERU", "MEA", "MRA", "FRA"],
      latitude: null,
      longitude: null,
      target_button_text: "Select",
      search_button_text: "Select",
      vehicle_data: {
        ERU: { target: "", search: "" },
        MEA: { target: "", search: "" },
        MRA: { target: "", search: "" },
        FRA: { target: "", search: "" }
      }
    };
  },
  methods: {
    selectTargetCoordinate() {
      console.log("Selecting target coordinate for:", this.selectedVehicle);
      this.selectingTarget = !this.selectingTarget;
      if (this.selectingTarget) {
        this.target_button_text = "Done";
      } else {
        this.target_button_text = "Select";
        console.log("Selected target coordinate for:", this.selectedVehicle);
      }
    },
    selectSearchArea() {
      console.log("Selecting search area for:", this.selectedVehicle);
      this.selectingSearch = !this.selectingSearch;
      if (this.selectingSearch) {
        this.search_button_text = "Done";
      } else {
        this.search_button_text = "Select";
        console.log("Selected search area for:", this.selectedVehicle);
      }
    },
    reset() {
      console.log("Resetting selection state");
      this.selectingTarget = false;
      this.selectingSearch = false;
      this.targetCoord = "";
      this.searchCoords = [""];
      this.target_button_text = "Select";
      this.search_button_text = "Select";
    },
    submitCoordinates() {
      console.log("Submitting coordinates");
    },
    printMISSION_INFO() {
      console.log("Printing MISSION_INFO:", JSON.stringify(this.MISSION_INFO));
    },
    close() {
      console.log("Closing form");
    }
  },
  props: {
    missionNumber: {
      type: Number,
      required: true
    }
  },
  computed: {
    populateTarget(): string | number {
      console.log("Populating target coordinate for:", this.selectedVehicle);
      return "Populating target coordinate for:";
    },
    populateSearch(): string | number {
      console.log("Populating search area for:", this.selectedVehicle);
      return "Populated";
    }
  }
};
</script>

<style scoped>
.coordinate-form {
  max-width: 18em;
}
select {
  width: 100%;
  padding: 10px;
  border: none;
  border-radius: 4px;
  font-size: 16px;
}

select:focus {
  outline: none;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}
select option {
  padding: 10px;
  color: #000000;
}
</style>
