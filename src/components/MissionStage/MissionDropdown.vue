<template>
  <div class="coordinate-form">
    <h2>{{ this.MISSION_INFO["missionName"] }}</h2>
    <form @submit.prevent="submitCoordinates()">
      <!-- Dropdown to show all stages -->
      <label for="stage">Stage:</label>
      <select id="stage" v-model="selectedStage" required @change="this.reset()">
        <option v-for="stage in this.getStageNames()" :key="stage" :value="stage">
          {{ stage }}
        </option>
      </select>

      <!-- Dropdown to show all vehicles -->
      <label for="vehicle">Vehicle:</label>
      <select id="vehicle" v-model="selectedVehicle" required @change="this.reset()">
        <option v-for="vehicle in vehicles" :key="vehicle" :value="vehicle">
          {{ vehicle }}
        </option>
      </select>
      <div style="display: grid; gap: 10px">
        <label for="targetCoordinate">Target Coordinate: {{}}</label>
        <input id="targetCoordinate" v-model="populateTarget" type="text" />
        <!-- current selected vehicle's target coord in v-model -->
        <button @click.prevent="selectTargetCoordinate">
          {{ target_button_text }}
        </button>

        <label for="searchArea">Search Area:</label>
        <input id="searchArea" v-model="populateSearch" type="text" />
        <!-- current selected vehicle's search area coords in v-model -->
        <button @click.prevent="selectSearchArea">
          {{ search_button_text }}
        </button>
      </div>
      <button type="submit" @click="printMISSION_INFO()">Submit</button>
    </form>
  </div>
</template>

<script>
import { inject } from "vue";
export default {
  setup() {
    const { searchCoords, selectingSearch } = inject("SearchCoords");
    const { targetCoord, selectingTarget } = inject("TargetCoord");
    const { MISSION_INFO, addStage, updateSearchArea, updateTarget, getStageNames, getStageInfo } =
      inject("Mission Info");

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
.coordinate-form {
  max-width: 18em;
}
select {
  width: 100%;
  padding: 10px;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  /* overflow:auto; */
}

select:focus {
  outline: none;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}
select option {
  padding: 10px;
}
</style>
