<!-- <template>
  <div class="map">
    <l-map
      ref="map"
      v-model:zoom="zoom"
      :use-global-leaflet="false"
      :center="mapOrigin"
    >
      <l-tile-layer
        :url="tileServer"
        layer-type="base"
        name="OpenStreetMap"
      ></l-tile-layer>
    </l-map>
  </div>
</template>

<script lang="ts">
import "leaflet/dist/leaflet.css";
import { LMap, LTileLayer } from "@vue-leaflet/vue-leaflet";

export default {
  components: {
    LMap,
    LTileLayer,
  },
  data() {
    return {
      tileServer:
        import.meta.env.VITE_OSM_SERVER ||
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
      mapOrigin: [34.058, -117.819],
      zoom: 17,
    };
  },
};
</script>

<style scoped>
.map {
  height: 100%;
}
</style> -->
<!-- <script setup lang="ts">
import { inject } from 'vue'

const { coords, updateCoords } = inject('Coords')

</script> -->

<!-- offline -->
<template>
  <div class="map">
    <l-map ref="map" v-model:zoom="zoom" :use-global-leaflet="false" :center="mapOrigin" @click="addPoint">
      <div class="button-container">
        <button class="send-button" @click="sendPolygonPoints">Send</button>
        <button class="clear-button" @click="clearPoints">Clear</button>
      </div>
      <l-tile-layer :url="localTileURL" :minZoom="14" :maxZoom="16" layer-type="base" name="CustomTiles"></l-tile-layer>
      <l-polygon
        v-if="polygonPoints.length > 0"
        :lat-lngs="polygonPoints"
        :options="{ fillColor: 'blue', fillOpacity: 0.4 }"
        :key="polygonPoints.length"
      ></l-polygon>
    </l-map>
  </div>
</template>

<script lang="ts">
import "leaflet/dist/leaflet.css";
import { inject, ref } from "vue";
import { LMap, LTileLayer, LPolygon  } from "@vue-leaflet/vue-leaflet";
import { LeafletMouseEvent, LatLngExpression  } from "leaflet";



export default {
  setup() {
    const {searchCoords, updateSearchCoords} = inject('SearchCoords');
    const { targetCoord, selectingTarget } = inject('TargetCoord');

    return { searchCoords, updateSearchCoords, targetCoord, selectingTarget}
  },
  components: {
    LMap,
    LTileLayer,
    LPolygon,
  },
  data() {
    return {
      mapOrigin: [35.33004319829399, -120.75064544958856], //area of interest origin, CPP: 34.058, -117.819
      zoom: 16,
      localTileURL: "http://localhost:8001/{z}/{x}/{y}.png", // Update to local server URL
      polygonPoints: [] as LatLngExpression[],
    };
  },
  methods: {
    addPoint(event: LeafletMouseEvent) {
      const lat = event.latlng.lat;
      const lng = event.latlng.lng;
      console.log("Clicked coordinates:", lat, lng);
      const latLng: LatLngExpression = [event.latlng.lat, event.latlng.lng];
      this.polygonPoints.push(latLng);
      console.log("polygonPoints:", this.polygonPoints);

      // if selectingTarget from App.vue is true, set targetCoord (also from App.vue) to the latest point you clicked on Map
      if (this.selectingTarget) {
        this.targetCoord = latLng;
        console.log("last clicked/currently selecting coordinate for target: " + this.targetCoord);
      }      
    },
    clearPoints(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      console.log(this.searchCoords.value)
      this.polygonPoints = []
      console.log("polygonPoints:", this.polygonPoints);
    },
    sendPolygonPoints(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      this.updateSearchCoords(this.polygonPoints);
      // console.log("asd",this.coords)
      // Send a POST request to the backend API

      // axios.post('backend-api-url', data)
      // .then(response => {
      //   console.log('Polygon points sent successfully:', response.data);
      // })
      // .catch(error => {
      //   console.error('Error sending polygon points:', error);
      // });

      console.log("polygonPoints sent:", this.polygonPoints);

    },
  },
};
</script>

<style scoped>
.map {
  height: 100%;
}
.clear-button,
.send-button {
  padding: 12px 24px;
  font-size: 16px;
  border: none;
  border-radius: 8px;
  background-color: #496ecc;
  color: white;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  transition-duration: 0.4s;
  cursor: pointer;
  margin: 10px;
}
.clear-button:hover,
.send-button:hover {
  background-color: #3d569c;
}
.button-container {
  position: absolute;
  right: 0px;
  top: 0px;
  display: flex;
  z-index: 999;
}
</style>
