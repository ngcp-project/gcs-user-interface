<template>
  <div class="flex w-full justify-center px-6 py-4">
    <div class="flex w-full max-w-xl flex-col items-center">
      <h1>Mission Initialization</h1>
      <h2>Already in this Mission</h2>
      <form @submit.prevent="submitForm()" class="grid w-full gap-2">
        <label for="missionName">Mission Name:</label>
        <NgInput
          id="missionName"
          v-model="missionName"
          placeholder="New mission"
          required
          @change="missionExists = false"
        />

        <label for="stageName">Initial Stage Name:</label>
        <NgInput id="stageName" v-model="stageName" placeholder="Stage Name" required />

        <NgButton type="submit" @click="submitForm()">Submit</NgButton>
      </form>

      <div class="w-full" v-if="MISSION_INFO['missionName'] != ''">
        <h1>Current Mission: {{ MISSION_INFO["missionName"] }}</h1>
        <h2 style="text-align: center">Create New Stage</h2>
        <h2 v-if="stageExists" style="text-align: center">Stage with that name already Exists!</h2>
        <form @submit.prevent="" style="display: grid; gap: 10px">
          <label for="stageName">Stage Name:</label>
          <NgInput
            id="stageName"
            v-model="newStageName"
            placeholder="Stage Name"
            required
            @change="stageExists = false"
          />
          <NgButton type="submit" @click="createNewStage()">Submit</NgButton>
        </form>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { useRouter } from "vue-router";
import { inject } from "vue";

import { MissionInfoProvider } from "@/types/mission-info-provider";
import { Stage } from "@/types";
import { NgInput } from "@/components/ui/input";
import { NgButton } from "@/components/ui/button";

export default {
  components: { NgInput, NgButton },
  setup() {
    const {
      MISSION_INFO,
      addStage,
      updateSearchArea,
      updateTarget,
      checkStageExists,
      save_MISSION_INFO
    } = inject<MissionInfoProvider>("mission-info-provider")!;

    const router = useRouter();
    // function to use Vue router to navigate to StaticScreen after pressing submit
    const toStaticScreen = () => {
      router.push(`/StaticScreen`);
    };

    return {
      toStaticScreen,
      MISSION_INFO,
      addStage,
      updateSearchArea,
      updateTarget,
      checkStageExists,
      save_MISSION_INFO
    };
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
          vehicleName: "ERU",
          target: null,
          searchArea: null
        },
        {
          vehicleName: "FRA",
          target: null,
          searchArea: null
        },
        {
          vehicleName: "MEA",
          target: null,
          searchArea: null
        },
        {
          vehicleName: "MRA",
          target: null,
          searchArea: null
        }
      ]
    };
  },
  methods: {
    submitForm() {
      if (this.MISSION_INFO["missionName"] == this.missionName) {
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
      const firstStage: Stage = {
        stageName: this.stageName,
        vehicleKeys: this.vehicleKeys
      };
      this.addStage(firstStage);

      // Here you can handle the form submission, for example send a POST request to your server
      fetch("http://localhost:5135/MissionInfo", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          missionName: this.missionName,
          stageName: this.stageName,
          vehicleKeys: this.vehicleKeys
        })
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error("Network response was not ok");
          }
          return response.json();
        })
        .then((data) => console.log(data))
        .catch((error) => console.error("Error initializing Mission Info:", error));

      this.toStaticScreen();
    },

    createNewStage() {
      // check if a stage with the current newStageName already exists in this mission
      if (this.checkStageExists(this.newStageName)) {
        console.log(
          "Stage with name " + this.newStageName + " already exists! Not creating new one"
        );
        this.stageExists = true;
      } else {
        const newStage: Stage = {
          stageName: this.newStageName,
          vehicleKeys: this.vehicleKeys
        };
        this.addStage(newStage);
        this.toStaticScreen();
        this.stageExists = false;
      }
    }
  }
};
</script>

<style scoped>
h1 {
  @apply text-xl font-bold;
}

h2 {
  @apply text-lg font-semibold;
}

form label {
  @apply text-foreground/70;
}
</style>
