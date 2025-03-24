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

<script setup lang="ts">
import "leaflet/dist/leaflet.css";
import { ref, watch, onMounted, inject } from "vue";
import { LMap, LTileLayer, LPolygon, LMarker } from "@vue-leaflet/vue-leaflet";
import { LeafletMouseEvent, LatLngTuple as LatLng, icon } from "leaflet";
import { LMarkerRotate } from "vue-leaflet-rotate-marker";
import { Button } from "@/components/ui/button";
import { SearchCoordsProvider } from "@/types/search-coords-provider";
import { TargetCoordsProvider } from "@/types/target-coords.provider";
import { MissionInfoProvider } from "@/types/mission-info-provider";
import {
  pushZoneInPolygons,
  pushZoneOutPolygons,
  clearZoneInPolygons,
  clearZoneOutPolygons,
  clearPolygons as clearGeofencePolygons,
  isInKeepInZone,
  isInKeepOutZone
} from "../Functions/geofence";

// Define Props
const props = defineProps<{
  ERU_coords: { latitude: number; longitude: number };
  ERU_yaw: number;
  MEA_coords: { latitude: number; longitude: number };
  MEA_yaw: number;
  MRA_coords: { latitude: number; longitude: number };
  MRA_yaw: number;
  FRA_coords: { latitude: number; longitude: number };
  FRA_yaw: number;
}>();

// Define Emits
const emit = defineEmits<{
  (e: "keepIn", vehicle: string, isIn: boolean): void;
  (e: "keepOut", vehicle: string, isOut: boolean): void;
}>();

// Inject providers
const { searchCoords, selectingSearch, updateSearchCoords } =
  inject<SearchCoordsProvider>("search-coords-provider")!;
const { targetCoord, selectingTarget } =
  inject<TargetCoordsProvider>("target-coords-provider")!;
const { load_MISSION_INFO } = inject<MissionInfoProvider>("mission-info-provider")!;

// State
const mapOrigin = ref<LatLng>([35.33004319829399, -120.75064544958856]);
const zoom = ref(16);
const localTileURL = ref("http://localhost:8080/tile/{z}/{x}/{y}.png");
const polygonPoints = ref<LatLng[]>([]); // current selected polygon (single)
const zoneInPolygons = ref<LatLng[][]>([]); // all zone in polygons from backend (multiple)
const zoneOutPolygons = ref<LatLng[][]>([]); // all zone out polygons from backend (multiple)
const searchPoints = ref<LatLng[]>([]); // current selected search area (single)

// Vehicle positions + icons
const ERU_position = ref<LatLng>([35.3308691455096, -120.74555890428901]);
const ERU_icon = icon({
  iconUrl: "../src/assets/ERU.png",
  iconSize: [38, 38]
});

const MEA_position = ref<LatLng>([35.32724060701405, -120.74394940698397]);
const MEA_icon = icon({
  iconUrl: "../src/assets/MEA.png",
  iconSize: [38, 38]
});

const MRA_position = ref<LatLng>([35.32682044954669, -120.74540868454052]);
const MRA_icon = icon({
  iconUrl: "../src/assets/MRA.png",
  iconSize: [38, 38]
});

const FRA_position = ref<LatLng>([35.3256474983931, -120.74015099334417]);
const FRA_icon = icon({
  iconUrl: "../src/assets/FRA.png",
  iconSize: [38, 38]
});

const target_coord_icon = icon({
  iconUrl: "../src/assets/target-coord-icon.png",
  iconSize: [20, 20]
});

// Methods
// creating current selected polygon
const addPoint = (event: LeafletMouseEvent) => {
  const lat = event.latlng.lat;
  const lng = event.latlng.lng;
  console.log("Clicked coordinates:", lat, lng);
  const latLng: LatLng = [lat, lng];
  
  if (!selectingSearch.value && !selectingTarget.value) {
    polygonPoints.value.push(latLng);
  }

  if (selectingSearch.value) {
    console.log("searchPoints: ", searchPoints.value);
    searchPoints.value.push(latLng);
    updateSearchCoords(searchPoints.value.map(point => `${point[0]},${point[1]}`));
  }

  if (selectingTarget.value) {
    targetCoord.value = `${lat},${lng}`;
    console.log("last clicked/currently selecting coordinate for target: " + targetCoord.value);
  }
};

// clear every polygon (selected and backend)
const clearPolygons = async (event: MouseEvent) => {
  event.stopPropagation();
  polygonPoints.value = [];
  zoneInPolygons.value = [];
  zoneOutPolygons.value = [];
  clearGeofencePolygons();
  try {
    const response = await fetch("http://localhost:5135/zones/in", {
      method: "DELETE",
      headers: { "Content-Type": "application/json" }
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
      headers: { "Content-Type": "application/json" }
    });
    if (!response) {
      throw new Error("Network response was not ok");
    }
    const res = await response.json();
    console.error("Cleared zoneOutPolygonPoints points:", res);
  } catch (error) {
    console.error("Error sending zoneOutPolygonPoints points:", error);
  }
  console.log("zoneOutnPolygonPoints:", polygonPoints.value);
  console.log("Cleared Selected zoneOutPolygons:", zoneInPolygons.value);
};

// clear current selected polygons
const clearSelection = async (event: MouseEvent) => {
  event.stopPropagation();
  polygonPoints.value = [];
  searchPoints.value = [];
  updateSearchCoords([]);
  targetCoord.value = "";
};

// sned current selected polygons as zone in polygons
const sendZoneInPolygonPoints = async (event: MouseEvent) => {
  event.stopPropagation();
  if (polygonPoints.value.length < 3) {
    console.log("Please select at least 3 points");
    return;
  }
  try {
    const coordinates = polygonPoints.value.map(([lat, lng]) => ({
      lat: lat,
      long: lng
    }));
    const payload = {
      keepIn: true,
      coordinates
    };

    const response = await fetch("http://localhost:5135/zones/in", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload)
    });

    if (!response.ok) {
      throw new Error("Network response was not ok");
    }

    const res = await response.json();
    console.log("zone In PolygonPoints sent successfully:", res);
    await getZoneIn();
    await clearSelection(event);
  } catch (error) {
    console.error("Error sending zoneInPolygonPoints points:", error);
  }
};

// send current selected polygons as zone out polygons
const sendZoneOutPolygonPoints = async (event: MouseEvent) => {
  event.stopPropagation();
  if (polygonPoints.value.length < 3) {
    console.log("Please select at least 3 points");
    return;
  }
  try {
    const coordinates = polygonPoints.value.map(([lat, lng]) => ({
      lat: lat,
      long: lng
    }));
    const payload = {
      keepIn: true,
      coordinates
    };

    const response = await fetch("http://localhost:5135/zones/out", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload)
    });

    if (!response.ok) {
      throw new Error("Network response was not ok");
    }

    const res = await response.json();
    console.log("zone out PolygonPoints sent successfully:", res);
    await getZoneOut();
    await clearSelection(event);
  } catch (error) {
    console.error("Error sending sendZoneOutPolygonPoints points:", error);
  }
};

// get all zone in polygons
const getZoneIn = async () => {
  try {
    const response = await fetch("http://localhost:5135/zones/in", {
      method: "GET"
    });
    if (!response.ok) {
      throw new Error("Network response was not ok");
    }
    const res = await response.json();
    zoneInPolygons.value = [];
    clearZoneInPolygons();
    let zones = res.data.split("|").map((zone: any) => JSON.parse(zone));
    zones.forEach((zone: any) => {
      const coordinates = zone.coordinates.map((coordinate: any) => [
        coordinate.lat,
        coordinate.long
      ] as LatLng);
      zoneInPolygons.value.push(coordinates);
      pushZoneInPolygons(coordinates);
    });
  } catch (error) {
    console.error("Error getting zone in polygons:", error);
  }
};

// get all zone out polygons
const getZoneOut = async () => {
  try {
    const response = await fetch("http://localhost:5135/zones/out", {
      method: "GET"
    });
    if (!response.ok) {
      throw new Error("Network response was not ok");
    }
    const res = await response.json();
    zoneOutPolygons.value = [];
    clearZoneOutPolygons();
    let zones = res.data.split("|").map((zone: any) => JSON.parse(zone));
    zones.forEach((zone: any) => {
      const coordinates = zone.coordinates.map((coordinate: any) => [
        coordinate.lat,
        coordinate.long
      ] as LatLng);
      zoneOutPolygons.value.push(coordinates);
      pushZoneOutPolygons(coordinates);
    });
  } catch (error) {
    console.error("Error getting zone out polygons:", error);
  }
};

// Watchers
watch(() => props.ERU_coords, (newERUcoords) => {
  const position: LatLng = [newERUcoords.latitude, newERUcoords.longitude];
  ERU_position.value = position;
  emit("keepIn", "ERU", isInKeepInZone(position));
  emit("keepOut", "ERU", isInKeepOutZone(position));
}, { deep: true });

watch(() => props.MEA_coords, (newMEAcoords) => {
  const position: LatLng = [newMEAcoords.latitude, newMEAcoords.longitude];
  MEA_position.value = position;
  emit("keepIn", "MEA", isInKeepInZone(position));
  emit("keepOut", "MEA", isInKeepOutZone(position));
}, { deep: true });

watch(() => props.MRA_coords, (newMRAcoords) => {
  const position: LatLng = [newMRAcoords.latitude, newMRAcoords.longitude];
  MRA_position.value = position;
  emit("keepIn", "MRA", isInKeepInZone(position));
  emit("keepOut", "MRA", isInKeepOutZone(position));
}, { deep: true });

watch(() => props.FRA_coords, (newFRAcoords) => {
  const position: LatLng = [newFRAcoords.latitude, newFRAcoords.longitude];
  FRA_position.value = position;
  emit("keepIn", "FRA", isInKeepInZone(position));
  emit("keepOut", "FRA", isInKeepOutZone(position));
}, { deep: true });

// Lifecycle hooks
onMounted(() => {
  load_MISSION_INFO();
  getZoneIn();
  getZoneOut();
});
</script>
