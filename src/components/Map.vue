<!-- offline -->
<template>
  <div class="h-full w-full">
    <l-map
      ref="map"
      v-model:zoom="zoom"
      :use-global-leaflet="false"
      :center="mapOrigin"
      @click="addPoint"
    >
      <div class="absolute right-0 top-0 flex items-center gap-2 p-2" style="z-index: 1000">
        <NgButton @click="sendZoneInPolygonPoints">Zone In</NgButton>
        <NgButton @click="sendZoneOutPolygonPoints">Zone Out</NgButton>
        <!-- <button class="send-button" @click="FetchZones" >Get In/Out</button> -->
        <NgButton @click="clearPolygons">Clear All</NgButton>
        <NgButton @click="clearSelection">Clear Selected</NgButton>
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

      <!------- VEHICLE + FIRE MARKERS -------->
      <l-marker-rotate
        :lat-lng="ERU_position"
        :icon="ERU_icon"
        :rotationAngle="ERU_yaw"
        class="-z-10"
      ></l-marker-rotate>
      <l-marker-rotate
        :lat-lng="MEA_position"
        :icon="MEA_icon"
        :rotationAngle="MEA_yaw"
      ></l-marker-rotate>
      <l-marker-rotate
        :lat-lng="MRA_position"
        :icon="MRA_icon"
        :rotationAngle="MRA_yaw"
      ></l-marker-rotate>
      <l-marker-rotate
        :lat-lng="FRA_position"
        :icon="FRA_icon"
        :rotationAngle="FRA_yaw"
      ></l-marker-rotate>

      <!---- UNCOMENT BELOW TO USE REGULAR MARKERS IF ROTATED MARKERS BUG OUT ---->
      <!-- <l-marker :lat-lng="ERU_position" :icon="ERU_icon"></l-marker>
      <l-marker :lat-lng="MEA_position" :icon="MEA_icon"></l-marker>
      <l-marker :lat-lng="MRA_position" :icon="MRA_icon"></l-marker>
      <l-marker :lat-lng="FRA_position" :icon="FRA_icon"></l-marker> -->
      <l-marker
        :icon="fire_icon"
        v-for="(point, index) in fireCoordsList"
        :key="index"
        :lat-lng="[point.latitude, point.longitude]"
      ></l-marker>

      <!-- POLYGON TO SHOW CURRENTLY SELECTED AREA FOR KEEP IN/OUT -->
      <l-polygon
        v-if="polygonPoints.length > 0"
        :lat-lngs="polygonPoints"
        :options="{ fillColor: 'blue', fillOpacity: 0.2 }"
        :key="polygonPoints.length"
      ></l-polygon>
      <!-- POLYGON TO SHOW KEEP IN ZONES -->
      <l-polygon
        v-if="zoneInPolygons.length > 0"
        :lat-lngs="zoneInPolygons"
        :options="{ color: 'green', fillColor: 'green', fillOpacity: 0 }"
        :key="zoneInPolygons.length"
      ></l-polygon>
      <!-- POLYGON TO SHOW KEEP OUT ZONES -->
      <l-polygon
        v-if="zoneOutPolygons.length > 0"
        :lat-lngs="zoneOutPolygons"
        :options="{ fillColor: 'red', fillOpacity: 0.3 }"
        :key="zoneOutPolygons.length"
      ></l-polygon>

      <!-- POLYGON TO SHOW SEARCH AREA -->
      <l-polygon
        v-if="searchPoints.length > 0"
        :lat-lngs="searchPoints"
        :options="{ color: 'purple', fillColor: '#CB59ED', fillOpacity: 0.2 }"
        :key="searchPoints.length"
      ></l-polygon>
      <l-marker
        v-if="selectingTarget && targetCoord[0] != null && targetCoord[1] != null"
        :icon="target_coord_icon"
        :lat-lng="targetCoord"
      ></l-marker>
    </l-map>
  </div>
</template>

<script lang="ts">
import "leaflet/dist/leaflet.css";
import { inject, ref } from "vue";
import { LMap, LTileLayer, LPolygon, LMarker } from "@vue-leaflet/vue-leaflet";
import { LeafletMouseEvent, LatLngExpression, icon } from "leaflet";

import {
  pushZoneInPolygons,
  pushZoneOutPolygons,
  clearZoneInPolygons,
  clearZoneOutPolygons,
  clearPolygons,
  isInKeepInZone,
  isInKeepOutZone
} from "../Functions/geofence";

import { LMarkerRotate } from "vue-leaflet-rotate-marker";
import { NgButton } from "@/components/ui/button";
import { SearchCoordsProvider } from "@/types/search-coords-provider";
import { TargetCoordsProvider } from "@/types/target-coords.provider";
import { MissionInfoProvider } from "@/types/mission-info-provider";
interface Coordinates {
  latitude: number;
  longitude: number;
}

export default {
  setup() {
    const { searchCoords, selectingSearch, updateSearchCoords } =
      inject<SearchCoordsProvider>("search-coords-provider")!;
    const { targetCoord, selectingTarget } =
      inject<TargetCoordsProvider>("target-coords-provider")!;
    const { load_MISSION_INFO } = inject<MissionInfoProvider>("mission-info-provider")!;

    return {
      searchCoords,
      selectingSearch,
      updateSearchCoords,
      targetCoord,
      selectingTarget,
      load_MISSION_INFO
    };
  },
  components: {
    LMap,
    LTileLayer,
    LPolygon,
    LMarker,
    LMarkerRotate,
    // eslint-disable-next-line vue/no-reserved-component-names
    NgButton
  },
  props: {
    //import fire prop from telemetry
    firePoint: { required: false, type: Object },

    // vehicle coordinate and yaw props to pass into vehicle markers
    ERU_coords: { required: true, type: Object },
    ERU_yaw: { required: true, type: Number },
    MEA_coords: { required: true, type: Object },
    MEA_yaw: { required: true, type: Number },
    MRA_coords: { required: true, type: Object },
    MRA_yaw: { required: true, type: Number },
    FRA_coords: { required: true, type: Object },
    FRA_yaw: { required: true, type: Number }
  },
  data() {
    return {
      mapOrigin: [35.33004319829399, -120.75064544958856], // area of interest origin, CPP: 34.058, -117.819
      zoom: 16,
      localTileURL: "http://localhost:8001/{z}/{x}/{y}.png", // Update to local server URL
      polygonPoints: [] as LatLngExpression[], // current selected polygons
      zoneInPolygons: [] as LatLngExpression[], // all zone in polygons from backend
      zoneOutPolygons: [] as LatLngExpression[], // all zone out polygons from backend
      fireCoordsList: [] as Coordinates[],
      searchPoints: [] as LatLngExpression[],
      maxFireCoordsCount: 10,
      lastUpdate: 0,
      updateInterval: 500, // Adjust as needed

      fire_icon: icon({
        iconUrl: "../src/assets/fire-icon.png",
        iconSize: [24, 34]
      }),
      ERU_position: [35.3308691455096, -120.74555890428901], // initial position
      ERU_icon: icon({
        iconUrl: "../src/assets/ERU.png",
        iconSize: [38, 38]
      }),
      MEA_position: [35.32724060701405, -120.74394940698397], // initial position
      MEA_icon: icon({
        iconUrl: "../src/assets/MEA.png",
        iconSize: [38, 38]
      }),
      MRA_position: [35.32682044954669, -120.74540868454052], // initial position
      MRA_icon: icon({
        iconUrl: "../src/assets/MRA.png",
        iconSize: [38, 38]
      }),
      FRA_position: [35.3256474983931, -120.74015099334417], // initial position
      FRA_icon: icon({
        iconUrl: "../src/assets/FRA.png",
        iconSize: [38, 38]
      }),
      target_coord_icon: icon({
        iconUrl: "../src/assets/target-coord-icon.png",
        iconSize: [20, 20]
      })
    };
  },
  methods: {
    //creating the current selected polygon
    addPoint(event: LeafletMouseEvent) {
      console.log("event", event);
      const lat = event.latlng.lat;
      const lng = event.latlng.lng;
      console.log("Clicked coordinates:", lat, lng);
      const latLng: LatLngExpression = [event.latlng.lat, event.latlng.lng];
      if (!this.selectingSearch && !this.selectingTarget) {
        this.polygonPoints.push(latLng);
      }

      if (this.selectingSearch) {
        console.log("searchPoints: ", this.searchPoints);
        this.searchPoints.push(latLng);
        this.updateSearchCoords(this.searchPoints);
      }
      console.log("polygonPoints:", this.polygonPoints);

      // if selectingTarget from App.vue is true, set targetCoord (also from App.vue) to the latest point you clicked on Map
      if (this.selectingTarget) {
        this.targetCoord = latLng;
        console.log("last clicked/currently selecting coordinate for target: " + this.targetCoord);
      }
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
      clearPolygons();
      try {
        const response = await fetch("http://localhost:5135/zones/in", {
          method: "DELETE",
          headers: {
            "Content-Type": "application/json"
          }
        });
        if (!response) {
          throw new Error("Network response was not ok");
        }

        const res = await response.json();
        console.error("Cleared zoneInPolygonPoints points:", res);
      } catch (error) {
        console.error("Error sending zoneInPolygonPoints points:", error);
      }
      try {
        const response = await fetch("http://localhost:5135/zones/out", {
          method: "DELETE",
          headers: {
            "Content-Type": "application/json"
          }
        });
        if (!response) {
          throw new Error("Network response was not ok");
        }

        const res = await response.json();
        console.error("Cleared zoneOutPolygonPoints points:", res);
      } catch (error) {
        console.error("Error sending zoneOutPolygonPoints points:", error);
      }
      console.log("zoneOutnPolygonPoints:", this.polygonPoints);
      console.log("Cleared Selected zoneOutPolygons:", this.zoneInPolygons);
    },
    //clear current selected polygons
    async clearSelection(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      console.log(this.searchCoords.value);
      this.polygonPoints = [];
      this.searchPoints = [];
      console.log("Cleared Selected zoneInPolygonPoints:", this.polygonPoints);
      this.updateSearchCoords(this.polygonPoints); // also clear currently selected coords for current vehicle's search area
      this.targetCoord = []; // also clear currently selected coord for current vehicle's target
    },
    //send current selected polygons as zone in polygons
    async sendZoneInPolygonPoints(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      if (this.polygonPoints.length < 3) {
        console.log("Please select at least 3 points");
        return;
      }
      try {
        const coordinates = this.polygonPoints.map((proxyArray) => {
          // Extract latitude and longitude from each Proxy object
          const latitude = proxyArray[0];
          const longitude = proxyArray[1];
          return { latitude, longitude };
        });

        const payload = {
          keepIn: true,
          coordinates: coordinates
        };

        const response = await fetch("http://localhost:5135/zones/in", {
          method: "POST",
          headers: {
            "Content-Type": "application/json"
          },
          body: JSON.stringify(payload)
        });

        if (!response.ok) {
          throw new Error("Network response was not ok");
        }

        const res = await response.json();
        console.log("zone In PolygonPoints sent successfully:", res);
        await this.getZoneIn();
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
      } catch (error) {
        console.error("Error sending zoneInPolygonPoints points:", error);
      }
    },
    //send current selected polygons as zone out polygons
    async sendZoneOutPolygonPoints(event: LeafletMouseEvent) {
      event.stopPropagation(); // Stop event propagation
      if (this.polygonPoints.length < 3) {
        console.log("Please select at least 3 points");
        return;
      }
      try {
        const coordinates = this.polygonPoints.map((proxyArray) => {
          // Extract latitude and longitude from each Proxy object
          const latitude = proxyArray[0];
          const longitude = proxyArray[1];
          return { latitude, longitude };
        });

        const payload = {
          keepIn: true,
          coordinates: coordinates
        };

        const response = await fetch("http://localhost:5135/zones/out", {
          method: "POST",
          headers: {
            "Content-Type": "application/json"
          },
          body: JSON.stringify(payload)
        });

        if (!response.ok) {
          throw new Error("Network response was not ok");
        }

        const res = await response.json();
        console.log("zone out PolygonPoints sent successfully:", res);
        await this.getZoneOut();
        await this.clearSelection(event);
      } catch (error) {
        console.error("Error sending sendZoneOutPolygonPoints points:", error);
      }
    },
    //get all zone in polygons
    async getZoneIn() {
      try {
        const response = await fetch("http://localhost:5135/zones/in", {
          method: "GET"
        });
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
        const res = await response.json();
        this.zoneInPolygons = []; //need to reset displayed zone in polygons
        clearZoneInPolygons();
        let zones = res.data.split("|").map((zone: any) => JSON.parse(zone));
        //console.log("zoneIn prev", this.zoneInPolygons);
        zones.forEach((zone: any) => {
          //console.log(zone.coordinates.map(coordinate => [coordinate.latitude, coordinate.longitude]));
          const coordinates = zone.coordinates.map((coordinate: any) => [
            coordinate.latitude,
            coordinate.longitude
          ]);
          this.zoneInPolygons.push([coordinates]);
          pushZoneInPolygons(coordinates);
        });
        console.log("Updated Zone In Polygons", this.zoneInPolygons);
      } catch (error) {
        console.error("Error sending sendZoneOutPolygonPoints points:", error);
      }
    },
    //get all zone out polygons
    async getZoneOut() {
      try {
        const response = await fetch("http://localhost:5135/zones/out", {
          method: "GET"
        });
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
        const res = await response.json();
        this.zoneOutPolygons = []; //need to reset displayed zone out polygons
        clearZoneOutPolygons();
        let zones = res.data.split("|").map((zone: any) => JSON.parse(zone));
        //console.log(zones);
        //console.log("Zoneout prev", this.zoneOutPolygons);
        zones.forEach((zone: any) => {
          //console.log(zone.coordinates);
          const coordinates = zone.coordinates.map((coordinate: any) => [
            coordinate.latitude,
            coordinate.longitude
          ]);
          this.zoneOutPolygons.push([coordinates]);
          pushZoneOutPolygons(coordinates);
        });
        console.log("Updated Zone Out Polygons", this.zoneOutPolygons);
      } catch (error) {
        console.error("Error sending sendZoneOutPolygonPoints points:", error);
      }
    },
    updateFireCoords(coords: Coordinates) {
      if (this.fireCoordsList.length > this.maxFireCoordsCount) {
        this.fireCoordsList.shift();
      }
      //pass the fire coords here
      this.fireCoordsList.push({ ...coords });
      localStorage.setItem("fireCoordsList", JSON.stringify(this.fireCoordsList));
      // this.fireCoordsList.push(firePoint);
      // console.log("firstptslist:", this.fireCoordsList)
    }
  },
  mounted() {
    this.load_MISSION_INFO();
    this.getZoneIn();
    this.getZoneOut();
    const storedFireCoords = localStorage.getItem("fireCoordsList");
    if (storedFireCoords) {
      try {
        this.fireCoordsList = JSON.parse(storedFireCoords);
      } catch (e) {
        console.error("Error parsing fireCoordsList from localStorage", e);
      }
    } else {
      // Initialize localStorage with an empty list if it doesn't exist
      localStorage.setItem("fireCoordsList", JSON.stringify(this.fireCoordsList));
    }
  },
  watch: {
    // uses deep watch to watch for changes in longitude and latitude properties in firePoint
    firePoint: {
      handler(newFireCoords) {
        // console.log("From watcher function in Map.vue: " + newFireCoords.latitude + " | " + newFireCoords.longitude);
        const currentTime = Date.now();
        if (currentTime - this.lastUpdate >= this.updateInterval) {
          this.updateFireCoords(newFireCoords);
          this.lastUpdate = currentTime;
        }
      },
      deep: true
    },

    // firePoints(newFireCoords) {
    //   const currentTime = Date.now();
    //   if (currentTime - this.lastUpdate >= this.updateInterval) {
    //     this.updateFireCoords(newFireCoords);
    //     this.lastUpdate = currentTime;
    //   }
    // },

    ERU_coords: {
      handler(newERUcoords) {
        this.ERU_position = [newERUcoords.latitude, newERUcoords.longitude];
        // console.log("ERU IN KEEP IN ZONE: "+ isInKeepInZone(this.ERU_position));
        this.$emit("keepIn", "ERU", isInKeepInZone(this.ERU_position));
        this.$emit("keepOut", "ERU", isInKeepOutZone(this.ERU_position));
        // console.log("IN KEEP OUT ZONE:  "+ isInKeepOutZone(this.ERU_position));
      },
      deep: true
    },
    MEA_coords: {
      handler(newMEAcoords) {
        this.MEA_position = [newMEAcoords.latitude, newMEAcoords.longitude];
        this.$emit("keepIn", "MEA", isInKeepInZone(this.MEA_position));
        this.$emit("keepOut", "MEA", isInKeepOutZone(this.MEA_position));
        // console.log("IN KEEP IN ZONE: "+ isInKeepInZone(this.MEA_position));
        // console.log("IN KEEP OUT ZONE:  "+ isInKeepOutZone(this.MEA_position));
      },
      deep: true
    },
    MRA_coords: {
      handler(newMRAcoords) {
        this.MRA_position = [newMRAcoords.latitude, newMRAcoords.longitude];
        this.$emit("keepIn", "MRA", isInKeepInZone(this.MRA_position));
        this.$emit("keepOut", "MRA", isInKeepOutZone(this.MRA_position));
        // console.log("IN KEEP IN ZONE: "+ isInKeepInZone(this.MRA_position));
        // console.log("IN KEEP OUT ZONE:  "+ isInKeepOutZone(this.MRA_position));
      },
      deep: true
    },
    FRA_coords: {
      handler(newFRAcoords) {
        this.FRA_position = [newFRAcoords.latitude, newFRAcoords.longitude];
        this.$emit("keepIn", "FRA", isInKeepInZone(this.FRA_position));
        this.$emit("keepOut", "FRA", isInKeepOutZone(this.FRA_position));
        // console.log("IN KEEP IN ZONE: "+ isInKeepInZone(this.FRA_position));
        // console.log("IN KEEP OUT ZONE:  "+ isInKeepOutZone(this.FRA_position));
      },
      deep: true
    }
  }
};
</script>
