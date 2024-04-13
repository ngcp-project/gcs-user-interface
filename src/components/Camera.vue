  <script>
  export default {
    props: ['cameraID'],
    data() {
      return {
        localIp: 'http://192.168.1.65', //maybe change accordingly .env?
        port: '5000', //maybe change accordingly
        cameraUrls: { 
          1: 'video_feed', //cam url here
          2: 'video_feed2',

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
    <div class="box video-section">
        <img :src="getCameraImageUrl(cameraID)" class="video-image">
    </div>
  </template>
  <style scoped>
  /* .app {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
  } */
  
  .video-section {
    width: 100%; 
    height: 100%; 
    box-sizing: border-box;
    border: 2px solid #ccc; 
  }
  
  .video-image {
    width: 100%;
    height: 100%;
    /* object-fit: contain; */
  }
  .box{
    border: 2px solid #030303; 
  }
  </style>
  