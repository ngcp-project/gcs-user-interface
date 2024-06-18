<script>
export default {
  props: ["cameraID"],
  data() {
    return {
      localIp: "http://192.168.1.173", //maybe change accordingly .env?
      port: "5000", //maybe change accordingly
      cameraUrls: {
        1: "video_feed", //cam url here
        2: "video_feed2"
      }
    };
  },
  mounted() {
    // this.initCam();
  },
  methods: {
    // async initCam() {
    //   try {
    //     const constraints = { video: true };
    //     const stream = await navigator.mediaDevices.getUserMedia(constraints);
    //     this.streams.push(stream);
    //   } catch (error) {
    //     console.error('Error accessing camera:', error);
    //   }
    // },
    getCameraImageUrl(cameraID) {
      return `${this.localIp}:${this.port}/${this.cameraUrls[cameraID]}`;
    }
  }
};
</script>
<template>
  <div class="h-full w-full border-2">
    <img :src="getCameraImageUrl(cameraID)" class="h-full w-full" />
  </div>
</template>
