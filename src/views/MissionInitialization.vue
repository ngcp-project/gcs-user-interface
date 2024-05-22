<template>
  <div style="width: fit-content; margin: auto; min-width: 35%">
    <h1>Mission Initialization</h1>
    <form @submit.prevent="submitForm" style="display: grid; gap: 10px">
      <label for="missionName">Mission Name:</label>
      <input id="missionName" v-model="missionName" placeholder="Mission Name" required />

      <label for="stageName">Stage Name:</label>
      <input id="stageName" v-model="stageName" placeholder="Stage Name" required />

      <div v-for="(vehicle, index) in vehicleKeys" :key="index" style="display: grid; gap: 10px">
        <label :for="'vehicleName' + index">Vehicle Name:</label>
        <input :id="'vehicleName' + index" v-model="vehicle.vehicleName" placeholder="Vehicle Name" required />

        <div style="display: grid; gap: 10px">
          <div style="display: grid; gap: 10px">
            <label :for="'targetLatitude' + index">Target Latitude:</label>
            <input :id="'targetLatitude' + index" v-model="vehicle.target.latitude" placeholder="Target Latitude" required />
            <input :id="'targetLongitude' + index" v-model="vehicle.target.longitude" placeholder="Target Longitude" required />
          </div>
          <div style="display: flex">
            <div
              v-for="(area, index) in vehicle.searchArea"
              :key="index"
              style="display: grid; gap: 10px; padding: 0 10px; width: 100%"
            >
              <label :for="'searchAreaLatitude' + index">Search Area:</label>
              <input :id="'searchAreaLatitude' + index" v-model="area.latitude" placeholder="Search Area Latitude" />
              <input :id="'searchAreaLongitude' + index" v-model="area.longitude" placeholder="Search Area Longitude" />
            </div>
          </div>
        </div>
      </div>

      <button type="submit" @click="toStaticScreen">Submit</button>
    </form>
  </div>
</template>

<script lang="ts">
import { useRouter } from 'vue-router';

export default {
  setup() {
    const router = useRouter();
    // function to use Vue router to navigate to StaticScreen after pressing submit
    const toStaticScreen = () => {
      router.push(`/StaticScreen`);
    }
    
    return {
      toStaticScreen
    }
  },
  
  data() {
    return {
      missionName: "",
      stageName: "",
      vehicleKeys: [
        {
          vehicleName: "",
          target: {
            latitude: "",
            longitude: "",
          },
          searchArea: [
            {
              latitude: "",
              longitude: "",
            },
            {
              latitude: "",
              longitude: "",
            },
          ],
        },
        {
          vehicleName: "",
          target: {
            latitude: "",
            longitude: "",
          },
          searchArea: [
            {
              latitude: "",
              longitude: "",
            },
            {
              latitude: "",
              longitude: "",
            },
          ],
        },
      ],
    };
  },
  methods: {
    submitForm() {
      // Here you can handle the form submission, for example send a POST request to your server
      console.log(this.missionName, this.stageName, this.vehicleKeys);
    },
  },
};
</script>
