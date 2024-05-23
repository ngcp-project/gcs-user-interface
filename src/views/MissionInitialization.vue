<template>
  <div style="width: fit-content; margin: auto; min-width: 35%">
    <h1>Mission Initialization</h1>
    <h2 v-if="missionExists" style="text-align: center;"> Already in this Mission </h2>
    <form @submit.prevent="submitForm();" style="display: grid; gap: 10px">
      <label for="missionName">Mission Name:</label>
      <input id="missionName" v-model="missionName" placeholder="Mission Name" required @change="missionExists = false;"/>

      <label for="stageName">Initial Stage Name:</label>
      <input id="stageName" v-model="stageName" placeholder="Stage Name" required />

      <!-- <div v-for="(vehicle, index) in vehicleKeys" :key="index" style="display: grid; gap: 10px">
        <label :for="'vehicleName' + index">Vehicle Name:</label>
        <input :id="'vehicleName' + index" v-model="vehicle.vehicleName" placeholder="Vehicle Name" required /> -->

        <!-- <div style="display: grid; gap: 10px">
          <div style="display: grid; gap: 10px">
            <label :for="'targetLatitude' + index">Target Latitude:</label>
            <input :id="'targetLatitude' + index" v-model="vehicle.target.latitude" placeholder="Target Latitude" />
            <input :id="'targetLongitude' + index" v-model="vehicle.target.longitude" placeholder="Target Longitude" />
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
        </div> -->
      <!-- </div> -->

      <button type="submit" @click="submitForm();">Submit</button>
    </form>

  <div v-if="MISSION_INFO['missionName'] != ''">
    <h1 style="margin-top: 20%;">Current Mission: {{ MISSION_INFO['missionName'] }} </h1>
    <h2 style="text-align: center;"> Create New Stage </h2>
    <h2 v-if="stageExists" style="text-align: center;"> Stage with that name already Exists! </h2>
    <form @submit.prevent="" style="display: grid; gap: 10px;">
      <label for="stageName">Stage Name:</label>
      <input id="stageName" v-model="newStageName" placeholder="Stage Name" required @change="stageExists = false;"/>
      <button type="submit" @click="createNewStage();">Submit</button>
    </form>
  </div>

  </div>
</template>

<script lang="ts">
import { useRouter } from 'vue-router';
import { inject } from "vue";
import { Stage } from "../Functions/types";

export default {
  setup() {
    const { MISSION_INFO, addStage, updateSearchArea, updateTarget, checkStageExists } = inject("Mission Info");

    const router = useRouter();
    // function to use Vue router to navigate to StaticScreen after pressing submit
    const toStaticScreen = () => {
      router.push(`/StaticScreen`);
    }
    
    return { toStaticScreen, MISSION_INFO, addStage, updateSearchArea, updateTarget, checkStageExists };
  },
  
  data() {
    return {
      missionName: "",
      stageName: "",
      newStageName: "",
      stageExists: false,
      missionExists: false,
      vehicleKeys: [
            {
                "vehicleName": "ERU",
                "target": null,
                "searchArea": null
            },
            {
                "vehicleName": "FRA",
                "target": null,
                "searchArea": null
            },{
                "vehicleName": "MEA",
                "target": null,
                "searchArea": null
            },{
                "vehicleName": "MRA",
                "target": null,
                "searchArea": null
            }
          ]
    };
  },
  methods: {
    submitForm() {
      if ( this.MISSION_INFO["missionName"] == this.missionName) {
        console.log("Already in this Mission");
        this.missionExists = true;
        return;
      } 

      // if there is a current mission, reset MISSION_INFO
      this.missionExists = false;
      this.MISSION_INFO["missionName"] = "";
      this.MISSION_INFO["stages"] = [] as Stage[];

      // initalizes MISSION_INFO with name and stage
      this.MISSION_INFO["missionName"] = this.missionName;
      const firstStage: Stage = {stageName: this.stageName,
                                 vehicleKeys: this.vehicleKeys};
      this.addStage(firstStage);

      // Here you can handle the form submission, for example send a POST request to your server
      console.log(this.missionName, this.stageName, this.vehicleKeys);
            fetch('http://localhost:5135/MissionInfo', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ 
                  missionName: this.missionName,
                  stageName: this.stageName,
                  vehicleKeys: this.vehicleKeys
                 })
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                return response.json();
            })
            .then(data => console.log(data))
            .catch(error => console.error('Error initializing Mission Info:', error));

            this.toStaticScreen();
          },

    createNewStage() {
      // check if a stage with the current newStageName already exists in this mission
      if (this.checkStageExists(this.newStageName)) {
        console.log("Stage with name " + this.newStageName + " already exists! Not creating new one");
        this.stageExists = true;
      } else {
        const newStage: Stage = {stageName: this.newStageName,
                               vehicleKeys: this.vehicleKeys};
        this.addStage(newStage);
        this.toStaticScreen();
        this.stageExists = false;
      }
    }
  },
};
</script>
