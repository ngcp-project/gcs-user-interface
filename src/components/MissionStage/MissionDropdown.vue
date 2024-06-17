<template>
  <DialogHeader>
    <DialogTitle>{{
      MISSION_INFO.missionName === "" ? "Unknown mission" : MISSION_INFO.missionName
    }}</DialogTitle>
    <DialogDescription> Make changes to your mission here. </DialogDescription>
  </DialogHeader>
  <form @submit.prevent="submitCoordinates()" class="flex flex-col gap-1">
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

    <label for="targetCoordinate">Target Coordinate: {{}}</label>
    <NgInput id="targetCoordinate" v-model="populateTarget" type="text" />
    <!-- current selected vehicle's target coord in v-model -->
    <NgButton @click.prevent="selectTargetCoordinate" size="sm" variant="secondary">
      {{ target_button_text }}
    </NgButton>

    <label for="searchArea">Search Area:</label>
    <NgInput id="searchArea" v-model="populateSearch" type="text" />
    <!-- current selected vehicle's search area coords in v-model -->
    <NgButton @click.prevent="selectSearchArea" size="sm" variant="secondary">
      {{ search_button_text }}
    </NgButton>
  </form>
  <DialogFooter>
    <DialogClose><NgButton variant="destructive" size="sm">Cancel</NgButton></DialogClose>
    <DialogClose>
      <NgButton type="submit" @click="printMISSION_INFO" size="sm">Submit</NgButton>
    </DialogClose>
  </DialogFooter>
</template>

<script lang="ts">
import { inject } from "vue";

import { MissionInfoProvider } from "@/types/mission-info-provider";
import { SearchCoordsProvider } from "@/types/search-coords-provider";
import { TargetCoordsProvider } from "@/types/target-coords.provider";
import {
  DialogClose,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue
} from "@/components/ui/select";

import { NgButton } from "../ui/button";
import { NgInput } from "../ui/input";

export default {
  components: {
    DialogClose,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    NgButton,
    NgInput,
    // eslint-disable-next-line vue/no-reserved-component-names
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
      selectedStage: null,

      selectedVehicle: "ERU",
      vehicles: ["ERU", "MEA", "MRA", "FRA"], // replace with vehicle names from backend fetch
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
    // clicking Select will set selectingTarget (from inject) to true, indicating to Map.vue to update targetCoord (from inject) to last clicked coordinate
    selectTargetCoordinate() {
      this.selectingTarget = !this.selectingTarget; // toggles selectingTarget aka if we are currently selecting target coordinate or not

      // this sets text of button to indicate that user is selecting target coord based on result state | also updates targetCoord based on start state
      if (this.selectingTarget) {
        // selectingTarget went from false -> true, so don't update targetCoordate
        console.log("Selecting Target Coordinate for " + this.selectedVehicle + "...");
        this.target_button_text = "Done";
      } else {
        // selectingTarget went from true -> false, so set targetCoordate to targetCoord (from injected)
        this.target_button_text = "Select";
        this.vehicle_data[this.selectedVehicle].target = this.targetCoord;
        console.log(
          "Selected Target Coordinate for " +
            this.selectedVehicle +
            ": " +
            this.vehicle_data[this.selectedVehicle].target
        );

        // THIS UPDATES TARGET COORDINATE FOR SPECIFIC VEHICLE OF SPECIFIED STAGE IN MISSION_INFO (massive JSON object from App.vue)
        this.updateTarget(this.selectedStage, this.selectedVehicle, this.targetCoord); // this updates target coord for specific vehicle of selected stage
        this.getStageInfo(this.selectedStage);
      }
    },
    selectSearchArea() {
      this.selectingSearch = !this.selectingSearch; // toggles selectingSearch

      if (this.selectingSearch) {
        // went from false -> true, so don't update searchArea
        console.log("Selecting Search Area Coords for " + this.selectedVehicle + "...");
        this.search_button_text = "Done";
      } else {
        // went from true -> false, so set searchArea to searchCoords (from injected)
        this.search_button_text = "Select";
        this.vehicle_data[this.selectedVehicle].search = this.searchCoords; // update current vehicle's search area with selected search area
        console.log(
          "Selected Search Area Coords for " +
            this.selectedVehicle +
            ": " +
            this.vehicle_data[this.selectedVehicle].search
        );

        // THIS UPDATES SEARCH AREA COORDS FOR SPECIFIC VEHICLE OF SPECIFIED STAGE IN MISSION_INFO (massive JSON object from App.vue)
        let preprocess = [];
        for (let i = 0; i < this.searchCoords.length; i++) {
          let lat = this.searchCoords[i][0];
          let long = this.searchCoords[i][1];
          preprocess.push({ latitude: lat, longitude: long });
        }
        this.updateSearchArea(this.selectedStage, this.selectedVehicle, preprocess); // this updates target coord for specific vehicle of selected stage
        this.getStageInfo(this.selectedStage);
      }
    },
    reset() {
      // when switching between vehicles in dropdown, set 'selectingTarget' and 'selectingSearch' back to false to reset flow
      // also set targetCoord and searchCoords (from provide/inject) to empty
      this.selectingTarget = false;
      this.selectingSearch = false;
      this.targetCoord = "";
      this.searchCoords = "";
      this.target_button_text = "Select";
      this.search_button_text = "Select";
    },
    submitCoordinates() {
      fetch("http://localhost:5135/MissionStage", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify(this.MISSION_INFO)
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error("Network response was not ok");
          }
          return response.json();
        })
        .then((data) => console.log(data))
        .catch((error) => console.error("Error initializing Mission Info:", error));
      this.close();
    },
    printMISSION_INFO() {
      console.log("Printing MISSION_INFO");
      console.log(JSON.stringify(this.MISSION_INFO));
    },
    close() {
      this.$emit("close");
    }
  },
  props: {
    missionNumber: {
      type: Number,
      required: true
    }
  },
  computed: {
    populateTarget() {
      // loop through stages
      for (let i = 0; i < this.MISSION_INFO["stages"].length; i++) {
        let currentStage = this.MISSION_INFO["stages"][i];
        if (currentStage["stageName"] == this.selectedStage) {
          // if we found selectedStage
          for (let j = 0; j < currentStage["vehicleKeys"].length; j++) {
            let vehicle = currentStage["vehicleKeys"][j];
            if (vehicle["vehicleName"] == this.selectedVehicle) {
              // if we found selectedVehicle
              if (vehicle["target"] != null) {
                return vehicle["target"].latitude + "," + vehicle["target"].longitude;
              } else {
                return "";
              }
            }
          }
        }
      }
      return "";
    },
    populateSearch() {
      // loop through stages
      for (let i = 0; i < this.MISSION_INFO["stages"].length; i++) {
        let currentStage = this.MISSION_INFO["stages"][i];
        if (currentStage["stageName"] == this.selectedStage) {
          // if we found selectedStage
          for (let j = 0; j < currentStage["vehicleKeys"].length; j++) {
            let vehicle = currentStage["vehicleKeys"][j];
            if (vehicle["vehicleName"] == this.selectedVehicle) {
              // if we found selectedVehicle
              let result = "";
              if (vehicle["searchArea"] != null) {
                for (let k = 0; k < vehicle["searchArea"].length; k++) {
                  // console.log(vehicle["searchArea"][k].longitude);
                  result += vehicle["searchArea"][k].latitude;
                  result += ",";
                  result += vehicle["searchArea"][k].longitude;
                  if (k != vehicle["searchArea"].length - 1) {
                    result += ",";
                  }
                }
              }
              return result;
            }
          }
        }
      }
      return "";
    }
  }
};
</script>
<style scoped>
/* .coordinate-form {
  max-width: 18em;
} */
/* select {
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
} */
</style>
