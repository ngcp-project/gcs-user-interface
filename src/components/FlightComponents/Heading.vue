<script>
import { Heading } from "vue-flight-indicators";

export default {
  name: "heading",
  props: {
    yaw: { required: true, type: Number }
  },
  components: {
    Heading
  }
};
</script>

<template>
  <div class="heading-div">
    <Heading :size="200" :heading="yaw" />
    <span style="font-size: 1.2em"> Yaw: {{ yaw }} </span>
  </div>
</template>

<style scoped>
.heading-div {
  position: relative;
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2%;
}
</style>
