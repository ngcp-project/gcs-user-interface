<!-- offline -->
<template>
  <div class="h-full w-full">
    <l-map
      ref="map"
      v-model:zoom="zoom"
      :use-global-leaflet="false"
      :center="[mapOrigin[0] as LatLng[0], mapOrigin[1] as LatLng[1]]"
      @click="addPoint"
    >
      <div class="absolute right-0 top-0 flex items-center gap-2 p-2" style="z-index: 1000">
        <Button @click="sendZoneInPolygonPoints">Zone In</Button>
        <Button @click="sendZoneOutPolygonPoints">Zone Out</Button>
        <!-- <button class="send-button" @click="FetchZones" >Get In/Out</button> -->
        <Button @click="clearPolygons">Clear All</Button>
        <Button @click="clearSelection">Clear Selected</Button>
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
        :lat-lng="[parseFloat(point.latitude || '0'), parseFloat(point.longitude || '0')]"
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
        :lat-lngs="zoneInPolygons[0]"
        :options="{ color: 'green', fillColor: 'green', fillOpacity: 0 }"
        :key="zoneInPolygons.length"
      ></l-polygon>
      <!-- POLYGON TO SHOW KEEP OUT ZONES -->
      <l-polygon
        v-if="zoneOutPolygons.length > 0"
        :lat-lngs="zoneOutPolygons[0]"
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
        :lat-lng="[parseFloat(targetCoord[0]), parseFloat(targetCoord[1])]"
      ></l-marker>
    </l-map>
  </div>
</template>

<script lang="ts">
import "leaflet/dist/leaflet.css";
import { inject, ref } from "vue";
import { LMap, LTileLayer, LPolygon, LMarker } from "@vue-leaflet/vue-leaflet";
import { LeafletMouseEvent, LatLngTuple as LatLng, icon } from "leaflet";

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
import { Button } from "@/components/ui/button";
import { SearchCoordsProvider } from "@/types/search-coords-provider";
import { TargetCoordsProvider } from "@/types/target-coords.provider";
import { MissionInfoProvider } from "@/types/mission-info-provider";

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
    Button
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
      mapOrigin: [35.33004319829399, -120.75064544958856] as LatLng,
      zoom: 16,
      localTileURL: "http://localhost:8080/tile/{z}/{x}/{y}.png",
      polygonPoints: [] as LatLng[], // current selected polygon (single)
      zoneInPolygons: [] as LatLng[][], // all zone in polygons from backend (multiple)
      zoneOutPolygons: [] as LatLng[][], // all zone out polygons from backend (multiple)
      searchPoints: [] as LatLng[], // current selected search area (single)
      fireCoordsList: [] as LatLng[],
      maxFireCoordsCount: 10,
      lastUpdate: 0,
      updateInterval: 500,

      fire_icon: icon({
        iconUrl: "../src/assets/fire-icon.png",
        iconSize: [24, 34]
      }) as any,
      ERU_position: [35.3308691455096, -120.74555890428901] as LatLng,
      ERU_icon: icon({
        iconUrl: "../src/assets/ERU.png",
        iconSize: [38, 38]
      }) as any,
      MEA_position: [35.32724060701405, -120.74394940698397] as LatLng,
      MEA_icon: icon({
        iconUrl: "../src/assets/MEA.png",
        iconSize: [38, 38]
      }) as any,
      MRA_position: [35.32682044954669, -120.74540868454052] as LatLng,
      MRA_icon: icon({
        iconUrl: "../src/assets/MRA.png",
        iconSize: [38, 38]
      }) as any,
      FRA_position: [35.3256474983931, -120.74015099334417] as LatLng,
      FRA_icon: icon({
        iconUrl: "../src/assets/FRA.png",
        iconSize: [38, 38]
      }) as any,
      target_coord_icon: icon({
        iconUrl: "../src/assets/target-coord-icon.png",
        iconSize: [20, 20]
      }) as any
    };
  },
  methods: {
    //creating the current selected polygon
    addPoint(event: LeafletMouseEvent) {
      const lat = event.latlng.lat;
      const lng = event.latlng.lng;
      console.log("Clicked coordinates:", lat, lng);
      const latLng: LatLng = [lat, lng];
      
      if (!this.selectingSearch && !this.selectingTarget) {
        this.polygonPoints.push(latLng);
      }

      if (this.selectingSearch) {
        console.log("searchPoints: ", this.searchPoints);
        this.searchPoints.push(latLng);
        this.updateSearchCoords(this.searchPoints.map(point => `${point[0]},${point[1]}`));
      }

      if (this.selectingTarget) {
        this.targetCoord = `${lat},${lng}`;
        console.log("last clicked/currently selecting coordinate for target: " + this.targetCoord);

      }
    },
    //clear every polygons (selected and backend)
    async clearPolygons(event: MouseEvent) {
      event.stopPropagation();
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
    async clearSelection(event: MouseEvent) {
      event.stopPropagation();
      this.polygonPoints = [];
      this.searchPoints = [];
      this.updateSearchCoords([]);
      this.targetCoord = "";
    },
    //send current selected polygons as zone in polygons
    async sendZoneInPolygonPoints(event: MouseEvent) {
      event.stopPropagation();
      if (this.polygonPoints.length < 3) {
        console.log("Please select at least 3 points");
        return;
      }
      try {
        const coordinates = this.polygonPoints.map(([lat, lng]) => ({
          lat: lat,
          long: lng
        }));
        const payload = {
          keepIn: true,
          coordinates
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
      } catch (error) {
        console.error("Error sending zoneInPolygonPoints points:", error);
      }
    },
    //send current selected polygons as zone out polygons
    async sendZoneOutPolygonPoints(event: MouseEvent) {
      event.stopPropagation();
      if (this.polygonPoints.length < 3) {
        console.log("Please select at least 3 points");
        return;
      }
      try {
        const coordinates = this.polygonPoints.map(([lat, lng]) => ({
          lat: lat,
          long: lng
        }));
        const payload = {
          keepIn: true,
          coordinates
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
        this.zoneInPolygons = [];
        clearZoneInPolygons();
        let zones = res.data.split("|").map((zone: any) => JSON.parse(zone));
        zones.forEach((zone: any) => {
          const coordinates = zone.coordinates.map((coordinate: any) => [
            coordinate.lat,
            coordinate.long
          ] as LatLng);
          this.zoneInPolygons.push(coordinates);
          pushZoneInPolygons(coordinates);
        });
      } catch (error) {
        console.error("Error getting zone in polygons:", error);
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
        this.zoneOutPolygons = [];
        clearZoneOutPolygons();
        let zones = res.data.split("|").map((zone: any) => JSON.parse(zone));
        zones.forEach((zone: any) => {
          const coordinates = zone.coordinates.map((coordinate: any) => [
            coordinate.lat,
            coordinate.long
          ] as LatLng);
          this.zoneOutPolygons.push(coordinates);
          pushZoneOutPolygons(coordinates);
        });
      } catch (error) {
        console.error("Error getting zone out polygons:", error);
      }
    },
    updateFireCoords(coords: { latitude: number, longitude: number}) {
      if (this.fireCoordsList.length > this.maxFireCoordsCount) {
        this.fireCoordsList.shift();
      }
      // Create new LatLng object
      const latlng: LatLng = [coords.latitude, coords.longitude];
      this.fireCoordsList.push(latlng);
      localStorage.setItem("fireCoordsList", JSON.stringify(this.fireCoordsList));
    }
  },
  mounted() {
    this.load_MISSION_INFO();
    this.getZoneIn();
    this.getZoneOut();
    const storedFireCoords = localStorage.getItem("fireCoordsList");
    if (storedFireCoords) {
      try {
        // Convert parsed coordinates to LatLng objects
        const parsed = JSON.parse(storedFireCoords);
        this.fireCoordsList = parsed.map((coord: { latitude: number, longitude: number }) => [coord.latitude, coord.longitude] as LatLng);
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
        const ERU_position: LatLng = [newERUcoords.latitude, newERUcoords.longitude];
        this.$emit("keepIn", "ERU", isInKeepInZone(ERU_position));
        this.$emit("keepOut", "ERU", isInKeepOutZone(ERU_position));
      },
      deep: true
    },
    MEA_coords: {
      handler(newMEAcoords) {
        const MEA_position: LatLng = [newMEAcoords.latitude, newMEAcoords.longitude];
        this.$emit("keepIn", "MEA", isInKeepInZone(MEA_position));
        this.$emit("keepOut", "MEA", isInKeepOutZone(MEA_position));
      },
      deep: true
    },
    MRA_coords: {
      handler(newMRAcoords) {
        const MRA_position: LatLng = [newMRAcoords.latitude, newMRAcoords.longitude];
        this.$emit("keepIn", "MRA", isInKeepInZone(MRA_position));
        this.$emit("keepOut", "MRA", isInKeepOutZone(MRA_position));
      },
      deep: true
    },
    FRA_coords: {
      handler(newFRAcoords) {
        const FRA_position: LatLng = [newFRAcoords.latitude, newFRAcoords.longitude];
        this.$emit("keepIn", "FRA", isInKeepInZone(FRA_position));
        this.$emit("keepOut", "FRA", isInKeepOutZone(FRA_position));
      },
      deep: true
    }
  }
};
</script>
