<template>
  <div
    style="
      width: 12rem;
      height: 4rem;
      border: 2px solid rgb(52, 49, 49);
      background-color: rgb(255, 255, 255);
      margin-left: auto;
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: flex-start;
    "
  >
    <div>
      <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0)">Mission {{ missionNumber }}</span>
      <button
        style="border: 2px solid rgb(0, 0, 0); margin-left: 1.2rem; color: rgb(0, 0, 0); padding: 3px 6px; font-size: 0.8rem"
        type="button"
        @click=""
      >
        <router-link to="/" class="router-link">
          <span style="font-weight: bold; font-size: 0.8rem" :style="{ color: status === 'initiated' ? 'black' : 'blue' }">OPEN</span>
        </router-link>
      </button>
    </div>
    <div>
      <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0)">Status: {{ status }}</span>
    </div>
  </div>
</template>

<script>
export default {
  name: "MissionStatus",
  props: {
    missionNumber: {
      type: Number,
      required: true,
    },
    status: {
      type: String,
      required: true,
    },
  },
};
</script>
