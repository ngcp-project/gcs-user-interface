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
        <button @click.prevent="selectTargetCoordinate">Select</button>

        <label for="searchArea">Search Area:</label>
        <input id="searchArea" v-model="searchArea" type="text" required />
        <button @click.prevent="selectSearchArea">Select</button>
      </div>
      <button type="submit">Submit</button>
    </form>
  </div>
</template>

<script>
import { inject } from "vue";
export default {
  setup() {
    const { coords } = inject("Coords");
    return { coords };
  },
  data() {
    return {
      selectedVehicle: null,
      vehicles: ["Vehicle 1", "Vehicle 2", "Vehicle 3"], // replace with vehicle names from backend fetch
      targetCoordinate: null,
      searchArea: null,
      latitude: null,
      longitude: null,
    };
  },
  methods: {
    // add in logic to select coordinates from map
    selectTargetCoordinate() {
      this.targetCoordinate = this.coords;
    },
    selectSearchArea() {},
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
