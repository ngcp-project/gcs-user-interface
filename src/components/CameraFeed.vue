<script setup lang="ts">
import { ref, computed } from "vue";

const props = defineProps<{ cameraID: number }>();

const localIp = ref("http://192.168.1.173"); //maybe change accordingly .env?
const port = ref("5000"); //maybe change accordingly
const cameraUrls = ref({
  1: "video_feed", //cam url here
  2: "video_feed2"
});

const getCameraImageUrl = (cameraID: number) => {
  return `${localIp.value}:${port.value}/${cameraUrls.value[cameraID]}`;
};

// async initCam() {
//   try {
//     const constraints = { video: true };
//     const stream = await navigator.mediaDevices.getUserMedia(constraints);
//     this.streams.push(stream);
//   } catch (error) {
//     console.error('Error accessing camera:', error);
//   }
// },

const cameraImageUrl = computed(() => getCameraImageUrl(props.cameraID));
</script>

<template>
  <div class="h-full w-full border-2">
    <img :src="cameraImageUrl" class="h-full w-full" />
  </div>
</template>
