  <script>
  export default {
    props: ['cameraNumber'],
    data() {
      return {
        localIp: 'http://192.168.1.173', //maybe change accordingly
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
        getCameraImageUrl(cameraNumber) {
          return `${this.localIp}:${this.port}/${this.cameraUrls[cameraNumber]}`;
      }
    }
  };
  </script>
  <template>
    <div class="box video-section">
        <img :src="getCameraImageUrl(cameraNumber)">
    </div>
  </template>
  <style scoped>
  .app {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
  }
  
  .video-section {
    width: 100%; 
    height: 100%; 
    box-sizing: border-box;
    border: 2px solid #ccc; 
  }
  
  .video-section video {
    width: 100%;
    height: 100%;
  }
  .box{
    border: 2px solid #030303; 
  }
  </style>
  