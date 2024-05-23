<template>
  <div class="coordinate-form">
    <h2>Mission {{ missionNumber }}</h2>
    <form @submit.prevent="submitCoordinates">
      <label for="vehicle">Vehicle:</label>
      <select id="vehicle" v-model="selectedVehicle" required>
        <option v-for="vehicle in vehicles" :key="vehicle" :value="vehicle">
          {{ vehicle }}
        </option>
      </select>
      <div style="display: grid; gap: 10px">
        <label for="targetCoordinate">Target Coordinate: {{}}</label>
        <input id="targetCoordinate" v-model="targetCoordinate" type="text" required />
        <button @click.prevent="selectTargetCoordinate">{{ target_button_text }}</button>

        <label for="searchArea">Search Area:</label>
        <input id="searchArea" v-model="searchArea" type="text" required />
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
      selectedVehicle: null,
      vehicles: ["Vehicle 1", "Vehicle 2", "Vehicle 3"], // replace with vehicle names from backend fetch
      targetCoordinate: null,
      searchArea: null,
      latitude: null,
      longitude: null,
      target_button_text: "Select",
      search_button_text: "Select"
    };
  },
  methods: {
    // clicking Select will set selectingTarget (from inject) to true, indicating to Map.vue to update targetCoord (from inject) to last clicked coordinate
    selectTargetCoordinate() {
      
      this.selectingTarget = !this.selectingTarget;   // toggles selectingTarget aka if we are currently selecting target coordinate or not

      // this sets text of button to indicate that user is selecting target coord based on result state
      // also updates targetCoord based on start state
      if (this.selectingTarget) { // selectingTarget went from false -> true, so don't update targetCoordate
        console.log("Selecting target coordinate...");
        this.target_button_text = "Done"
      } else { // selectingTarget went from true -> false, so set targetCoordate to targetCoord (from injected) 
        this.targetCoordinate = this.targetCoord;
        console.log("Selected Target Coordinate: " + this.targetCoord);
        this.target_button_text = "Select"
      }
    },
    selectSearchArea() {
      this.selectingSearch = !this.selectingSearch; // toggles selectingSearch

      if (this.selectingSearch) { // went from false -> true, so don't update searchArea
        console.log("Selecting search area coords...");
        this.search_button_text = "Done"
      } else { // went from true -> false, so set searchArea to searchCoords (from injected)
        this.searchArea = this.searchCoords;
        console.log("Selected Search Area Coords: " + this.searchCoords);
        this.search_button_text = "Select"
      }
      
    },
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
