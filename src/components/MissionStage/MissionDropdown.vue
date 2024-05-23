<template>
  <div class="coordinate-form">
    <h2>Mission {{ missionNumber }}</h2>
    <form @submit.prevent="submitCoordinates">
      <label for="vehicle">Vehicle:</label>
      <select id="vehicle" v-model="selectedVehicle" required @change="reset()">
        <option v-for="vehicle in vehicles" :key="vehicle" :value= "vehicle" >
          {{ vehicle }}
        </option>
      </select>
      <div style="display: grid; gap: 10px">
        <label for="targetCoordinate">Target Coordinate: {{}}</label>
        <input id="targetCoordinate" v-model="this.vehicle_data[this.selectedVehicle].target" type="text" required /> <!-- current selected vehicle's target coord in v-model -->
        <button @click.prevent="selectTargetCoordinate">{{ target_button_text }}</button>

        <label for="searchArea">Search Area:</label>
        <input id="searchArea" v-model="this.vehicle_data[this.selectedVehicle].search" type="text" required /> <!-- current selected vehicle's search area coords in v-model -->
        <button @click.prevent="selectSearchArea">{{ search_button_text }}</button>
      </div>
      <button type="submit">Submit</button>
    </form>
  </div>
</template>

<script>
import { inject } from "vue";
export default {
  setup() {
    const { searchCoords, selectingSearch } = inject("SearchCoords");
    const { targetCoord, selectingTarget } = inject('TargetCoord');

    return { searchCoords, selectingSearch, selectingTarget, targetCoord };
  },
  data() {
    return {
      selectedVehicle: "ERU",
      vehicles: ["ERU", "MEA", "MRA", "FRA"], // replace with vehicle names from backend fetch
      latitude: null,
      longitude: null,
      target_button_text: "Select",
      search_button_text: "Select",
      vehicle_data: {
        "ERU": { target: "", search: "" },
        "MEA": { target: "", search: "" },
        "MRA": { target: "", search: "" },
        "FRA": { target: "", search: "" }
      }
    };
  },
  methods: {
    // clicking Select will set selectingTarget (from inject) to true, indicating to Map.vue to update targetCoord (from inject) to last clicked coordinate
    selectTargetCoordinate() {
      this.selectingTarget = !this.selectingTarget;   // toggles selectingTarget aka if we are currently selecting target coordinate or not

      // this sets text of button to indicate that user is selecting target coord based on result state | also updates targetCoord based on start state
      if (this.selectingTarget) { // selectingTarget went from false -> true, so don't update targetCoordate
        console.log("Selecting Target Coordinate for " + this.selectedVehicle + "...");
        this.target_button_text = "Done";
      } else { // selectingTarget went from true -> false, so set targetCoordate to targetCoord (from injected) 
        this.target_button_text = "Select";
        this.vehicle_data[this.selectedVehicle].target = this.targetCoord;
        console.log("Selected Target Coordinate for " + this.selectedVehicle + ": " + this.vehicle_data[this.selectedVehicle].target);
      }
    },
    selectSearchArea() {
      this.selectingSearch = !this.selectingSearch; // toggles selectingSearch

      if (this.selectingSearch) { // went from false -> true, so don't update searchArea
        console.log("Selecting Search Area Coords for " + this.selectedVehicle + "...");
        this.search_button_text = "Done"
      } else { // went from true -> false, so set searchArea to searchCoords (from injected)
        this.search_button_text = "Select"
        this.vehicle_data[this.selectedVehicle].search = this.searchCoords;    // update current vehicle's search area with selected search area
        console.log("Selected Search Area Coords for " + this.selectedVehicle + ": " + this.vehicle_data[this.selectedVehicle].search);
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
    }
  },
  props: {
    missionNumber: {
      type: Number,
      required: true,
    },
  },
};
</script>
<style scoped>
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
}
</style>
