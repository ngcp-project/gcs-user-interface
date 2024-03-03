  <script>
  export default {
    data() {
      return {
        
      };
    },
    mounted() {
      this.initCam();
    },
    methods: {
      async initCam() {
        try {
          const constraints = { video: true };
          const stream = await navigator.mediaDevices.getUserMedia(constraints);
          this.streams.push(stream);
        } catch (error) {
          console.error('Error accessing camera:', error);
        }
      }
    }
  };
  </script>
  <template>
    <!-- <div class="app">
        <div class="video-section" v-for="(stream, index) in streams" :key="index">
          <video ref="videoPlayer" :srcObject="stream" controls></video>
      </div>
    </div> -->
    <div class="box video-section">
        <video ref="videoPlayer" :srcObject="stream" controls></video>
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
    width: 100%; /* Adjust according to your layout */
    height: 100%; /* Adjust according to your layout */
    box-sizing: border-box;
    border: 2px solid #ccc; /* Adding a small border */
  }
  
  .video-section video {
    width: 100%;
    height: 100%;
  }
  .box{
    border: 2px solid #030303; /* Adding a small border */
  }
  </style>
  