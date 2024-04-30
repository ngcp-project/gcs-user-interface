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

<!-- offline -->
<template>
  <div class="map">
    <l-map
      ref="map"
      v-model:zoom="zoom"
      :use-global-leaflet="false"
      :center="mapOrigin"
      @click="addPoint"
    >
      <div class="button-container">
        <button class="send-button" @click="sendZoneInPolygonPoints">Zone In</button>
        <button class="send-button" @click="sendZoneOutPolygonPoints">Zone Out</button>
        <button class="send-button" @click="getZoneIn">Get In</button>
        <button class="send-button" @click="getZoneOut">Get Out</button>
        <button class="clear-button" @click="clearPolygons">Clear</button>
        <button class="clear-button" @click="clearSelection">Clear Selected</button>
      </div>
      <l-tile-layer
        :url="localTileURL"
        :minZoom="14"
        :maxZoom="16"
        layer-type="base"
        name="CustomTiles"
        
      ></l-tile-layer>
      <l-polygon
        v-if="polygonPoints.length > 0"
        :lat-lngs="polygonPoints"
        :options="{ fillColor: 'blue', fillOpacity: 0.4 }"
        :key="polygonPoints.length"
      ></l-polygon>
      <l-polygon
        v-if="zoneInPolygons.length > 0"
        :lat-lngs="zoneInPolygons"
        :options="{ fillColor: 'green', fillOpacity: 0.4 }"
        :key="zoneInPolygons.length"
      ></l-polygon>
      <!-- <l-polygon
        v-for="(polygon, index) in zoneInPolygons"
        :key="index"
        :lat-lngs="polygon.latLngs"
        :options="polygon.options"
      ></l-polygon> -->
    </l-map>
  </div>
</template>

<script lang="ts">
import "leaflet/dist/leaflet.css";
import { LMap, LTileLayer, LPolygon  } from "@vue-leaflet/vue-leaflet";
import { LeafletMouseEvent, LatLngExpression  } from "leaflet";
import Coordinate from "./VehicleStatus/Coordinate.vue";


export default {
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
      zoneInPolygons: [] as LatLngExpression[],
      zoneOutPolygons: [] as LatLngExpression[],
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
    },
    async clearPolygons(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      this.polygonPoints = [];
      this.zoneInPolygons = [];
      this.zoneOutPolygons = [];
      try {
        const response = await fetch('http://localhost:5135/zones/in', {
          method: 'DELETE',
          headers: {
            'Content-Type': 'application/json'
          },
        });
        if (!response) {
          throw new Error('Network response was not ok');
        }
        
        const res = await response.json();
        console.error('Cleared zoneInPolygonPoints points:', res);
      } catch (error) {
        console.error('Error sending zoneInPolygonPoints points:', error);

      }
      console.log("zoneInPolygonPoints:", this.polygonPoints);
      console.log("Cleared Selected zoneInPolygons:", this.zoneInPolygons);
    },
    async clearSelection(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      this.polygonPoints = [];
      console.log("Cleared Selected zoneInPolygonPoints:", this.polygonPoints);
      
    },
    async sendZoneInPolygonPoints(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      try {
        const coordinates = this.polygonPoints.map(proxyArray => {
          // Extract latitude and longitude from each Proxy object
          const latitude = proxyArray[0];
          const longitude = proxyArray[1];
          return { latitude, longitude};
        });
        
        const payload = {
          name: "TEST",
          shapeType: "polygon",
          coordinates: coordinates,
        };
        
        const response = await fetch('http://localhost:5135/zones/in', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(payload)
        });

        if (!response.ok) {
          throw new Error('Network response was not ok');
        }

        const res = await response.json();
        console.log('zone In PolygonPoints sent successfully:', res);
        const multiPolygon = [
          [ // First polygon
            [ // Outer ring of the first polygon
              [35.33165465225685, -120.75643812675729], [35.32603702617105, -120.75843434541602], [35.324846949352334, -120.74817421080436], [35.335119535355524, -120.74864643457312]
            ],
            [ // Hole of the first polygon
              [35.334402071757125, -120.74561990950984], [35.32983465415885, -120.74881815230721], [35.32796211334611, -120.74018933616938],
            ]
          ],
          [ // Second polygon
            [ // Outer ring of the second polygon
              [35.33167215203964, -120.75609469128912], [35.32671956232397, -120.75285351905823], [35.333002124445976, -120.7487108287234],
            ]
          ],
        ];
        this.zoneInPolygons = multiPolygon;
        // console.log(this.zoneInPolygons);
        
      } 
      catch (error) {
        console.error('Error sending zoneInPolygonPoints points:', error);
      }
      
    },
    async sendZoneOutPolygonPoints(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      try {
        const coordinates = this.polygonPoints.map(proxyArray => {
          // Extract latitude and longitude from each Proxy object
          const latitude = proxyArray[0];
          const longitude = proxyArray[1];
          return { latitude, longitude};
        });
        
        const payload = {
          name: "TEST",
          shapeType: "polygon",
          coordinates: coordinates,
        };
        
        const response = await fetch('http://localhost:5135/zones/out', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(payload)
        });

        if (!response.ok) {
          throw new Error('Network response was not ok');
        }

        const res = await response.json();
        console.log('zone out PolygonPoints sent successfully:', res);
      } 
      catch (error) {
        console.error('Error sending sendZoneOutPolygonPoints points:', error);
      }
    },
    async getZoneIn(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      try {
        const response = await fetch('http://localhost:5135/zones/in', {
          method: 'GET',
        });

        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        const res = await response.json();
        //console.log(res);
        res.forEach(item => {
          const obj = JSON.parse(item);
          //console.log(obj);
          console.log(obj.coordinates);
          this.zoneInPolygons.push(obj.coordinates);
        });
        console.log(this.zoneInPolygons);
        this.zoneInPolygons.forEach(element => {
          console.log(element);
        });
        //console.log(this.zoneInPolygons[0][0].latitude);
      }
      catch (error) {
        console.error('Error sending sendZoneOutPolygonPoints points:', error);
      }
    },
    async getZoneOut(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      try {
        const response = await fetch('http://localhost:5135/zones/out', {
          method: 'GET',
        });

        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        const data = await response.text(); // Read response as text
        const res = data.split('|');
        //const res = await response.json();
        console.log(res);
        // res.data.array.forEach(polygon => {
        //   console.log(polygon);
        // });
        // data.forEach(item => {
          //console.log(obj);
          // console.log(obj.coordinates);
          // this.zoneOutPolygons.push(obj.coordinates);
        // });
        console.log(this.zoneOutPolygons);
        this.zoneOutPolygons.forEach(element => {
          console.log(element);
        });
        //console.log(this.zoneInPolygons[0][0].latitude);
      }
      catch (error) {
        console.error('Error sending sendZoneOutPolygonPoints points:', error);
      }
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