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
        <!-- <button class="send-button" @click="FetchZones" >Get In/Out</button> -->
        <button class="clear-button" @click="clearPolygons">Clear All</button>
        <button class="clear-button" @click="clearSelection">Clear Selected</button>
        <!-- <div class="fire-info" v-if="fire">
          <h3>Fire Information</h3>
          <p>coords:{{ fire.longitude.toFixed(6) }} {{ fire.latitude.toFixed(6) }}</p>
        </div> -->
      </div>
      <l-tile-layer
        :url="localTileURL"
        :minZoom="14"
        :maxZoom="16"
        layer-type="base"
        name="CustomTiles"
        
      ></l-tile-layer>
      <l-marker
        :icon="fire_icon"
        v-for="(point, index) in fireCoordsList"
        :key="index"
        :lat-lng="[point.latitude, point.longitude]"
      ></l-marker>
      <l-polygon
        v-if="polygonPoints.length > 0"
        :lat-lngs="polygonPoints"
        :options="{ fillColor: 'blue', fillOpacity: 0.2 }"
        :key="polygonPoints.length"
      ></l-polygon>
      <l-polygon
        v-if="zoneInPolygons.length > 0"
        :lat-lngs="zoneInPolygons"
        :options="{ color: 'green', fillColor: 'green', fillOpacity: 0 }"
        :key="zoneInPolygons.length"
      ></l-polygon>
      <l-polygon
        v-if="zoneOutPolygons.length > 0"
        :lat-lngs="zoneOutPolygons"
        :options="{ fillColor: 'red', fillOpacity: 0.3 }"
        :key="zoneOutPolygons.length"
      ></l-polygon>
    </l-map>
  </div>
</template>

<script lang="ts">
import "leaflet/dist/leaflet.css";
import { LMap, LTileLayer, LPolygon, LMarker  } from "@vue-leaflet/vue-leaflet";
import { LeafletMouseEvent, LatLngExpression, icon } from "leaflet";
interface Coordinates {
  latitude: number;
  longitude: number;
}

export default {
  components: {
    LMap,
    LTileLayer,
    LPolygon,
    LMarker,
  },
  props: {
    //import fire prop from telemetry
    firePoint: { required: false, type: Object},
  },
  data() {
    return {
      mapOrigin: [35.33004319829399, -120.75064544958856], //area of interest origin, CPP: 34.058, -117.819
      zoom: 16,
      localTileURL: "http://localhost:8001/{z}/{x}/{y}.png", // Update to local server URL
      polygonPoints: [] as LatLngExpression[], //current selected polygons
      zoneInPolygons: [] as LatLngExpression[], //all zone in polygons from backend
      zoneOutPolygons: [] as LatLngExpression[], //all zone out polygons from backend
      fireCoordsList: [],
      maxFireCoordsCount: 10,
      lastUpdate: 0,
      updateInterval: 500, // Adjust as needed

      fire_icon: icon({
          iconUrl: "../src/assets/fire-icon.png",
          iconSize: [24, 34],
        }),
    };
  },
  methods: {
    //creating the current selected polygon
    addPoint(event: LeafletMouseEvent) {
      const lat = event.latlng.lat;
      const lng = event.latlng.lng;
      console.log("Clicked coordinates:", lat, lng);
      const latLng: LatLngExpression = [event.latlng.lat, event.latlng.lng];
      this.polygonPoints.push(latLng);
      console.log("polygonPoints:", this.polygonPoints);
      //testing fire pts list
      // const coords: Coordinates = {
      //   latitude: lat,
      //   longitude: lng
      // };
      // this.updateFireCoords(coords)
    },
    //clear every polygons (selected and backend)
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
      try {
        const response = await fetch('http://localhost:5135/zones/out', {
          method: 'DELETE',
          headers: {
            'Content-Type': 'application/json'
          },
        });
        if (!response) {
          throw new Error('Network response was not ok');
        }
        
        const res = await response.json();
        console.error('Cleared zoneOutPolygonPoints points:', res);
      } catch (error) {
        console.error('Error sending zoneOutPolygonPoints points:', error);

      }
      console.log("zoneOutnPolygonPoints:", this.polygonPoints);
      console.log("Cleared Selected zoneOutPolygons:", this.zoneInPolygons);
    },
    //clear current selected polygons 
    async clearSelection(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      this.polygonPoints = [];
      console.log("Cleared Selected zoneInPolygonPoints:", this.polygonPoints);
      
    },
    //send current selected polygons as zone in polygons
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
          "keepIn": true,
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
        await this.getZoneIn(event);
        await this.clearSelection(event);
        // const multiPolygon = [
        //   [ // First polygon
        //     [ // Outer ring of the first polygon
        //       [35.33165465225685, -120.75643812675729], [35.32603702617105, -120.75843434541602], [35.324846949352334, -120.74817421080436], [35.335119535355524, -120.74864643457312]
        //     ],
        //     [ // Hole of the first polygon
        //       [35.334402071757125, -120.74561990950984], [35.32983465415885, -120.74881815230721], [35.32796211334611, -120.74018933616938],
        //     ]
        //   ],
        //   [ // Second polygon
        //     [ // Outer ring of the second polygon
        //       [35.33167215203964, -120.75609469128912], [35.32671956232397, -120.75285351905823], [35.333002124445976, -120.7487108287234],
        //     ]
        //   ],
        // ];
        // this.zoneInPolygons = multiPolygon;
        // console.log(this.zoneInPolygons);
        
      } 
      catch (error) {
        console.error('Error sending zoneInPolygonPoints points:', error);
      }
      
    },
    //send current selected polygons as zone out polygons
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
          "keepIn": true,
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
        await this.getZoneOut(event);
        await this.clearSelection(event);
      } 
      catch (error) {
        console.error('Error sending sendZoneOutPolygonPoints points:', error);
      }
    },
    //get all zone in polygons 
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
        this.zoneInPolygons = []; //need to reset displayed zone in polygons
        let zones = res.data.split("|").map((zone : any) => JSON.parse(zone))
        //console.log("zoneIn prev", this.zoneInPolygons);
        zones.forEach((zone : any) => {
          //console.log(zone.coordinates.map(coordinate => [coordinate.latitude, coordinate.longitude]));
          const coordinates = zone.coordinates.map((coordinate : any) => [coordinate.latitude, coordinate.longitude]);
          this.zoneInPolygons.push([coordinates]);
          
        });
        console.log("Updated Zone In Polygons", this.zoneInPolygons);
      }
      catch (error) {
        console.error('Error sending sendZoneOutPolygonPoints points:', error);
      }
    },
    //get all zone out polygons 
    async getZoneOut(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      try {
        const response = await fetch('http://localhost:5135/zones/out', {
          method: 'GET',
        });
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        const res = await response.json();
        this.zoneOutPolygons = []; //need to reset displayed zone out polygons
        let zones = res.data.split("|").map((zone : any) => JSON.parse(zone))
        //console.log(zones);
        //console.log("Zoneout prev", this.zoneOutPolygons);
        zones.forEach((zone : any) => {
          //console.log(zone.coordinates);
          const coordinates = zone.coordinates.map((coordinate : any) => [coordinate.latitude, coordinate.longitude]);
          this.zoneOutPolygons.push([coordinates]);
        });
        console.log("Updated Zone Out Polygons" , this.zoneOutPolygons);
      }
      catch (error) {
        console.error('Error sending sendZoneOutPolygonPoints points:', error);
      }
    },
    updateFireCoords(coords : Coordinates) {
      if (this.fireCoordsList.length > this.maxFireCoordsCount) {
        this.fireCoordsList.shift();
      }
      //pass the fire coords here
      this.fireCoordsList.push(coords);
      // this.fireCoordsList.push(firePoint);
      console.log("firstptslist:", this.fireCoordsList)
    },
  },
  watch: {
    // uses deep watch to watch for changes in longitude and latitude properties in firePoint
    firePoint: {
      handler(newFireCoords) {
        console.log("From watcher function in Map.vue: " + newFireCoords.latitude + " | " + newFireCoords.longitude);
        const currentTime = Date.now();
        if (currentTime - this.lastUpdate >= this.updateInterval) {
          this.updateFireCoords(newFireCoords);
          this.lastUpdate = currentTime;
        }
      },
      deep: true
    }
    
    // firePoints(newFireCoords) {
    //   const currentTime = Date.now();
    //   if (currentTime - this.lastUpdate >= this.updateInterval) {
    //     this.updateFireCoords(newFireCoords);
    //     this.lastUpdate = currentTime;
    //   }
    // },
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