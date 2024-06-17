<script setup lang="ts">
import { ref, onBeforeUnmount } from "vue";
import { NgInput } from "@/components/ui/input";
import { NgButton } from "@/components/ui/button";

// const webSocketUrl = "ws://192.168.1.65:5135/ws"; // WebSocket server URL
const webSocketUrl = "ws://localhost:5137/ws/fra"; // WebSocket server URL
const socket = ref<WebSocket | null>(null);
const receivedData = ref<any>(null);

// Reactive variables for user input
const key = ref("Vehicle");
const speed = ref<number>(0);
const pitch = ref<number>(0);
const yaw = ref<number>(0);
const roll = ref<number>(0);
const altitude = ref<number>(0);
const batteryLife = ref<number>(0);
const latitude = ref<number>(0);
const longitude = ref<number>(0);

// Function to establish the WebSocket connection
function establishWebSocketConnection() {
  socket.value = new WebSocket(webSocketUrl);

  // Set up event listeners
  socket.value.onopen = () => {
    console.log("WebSocket connection opened.");
  };

  socket.value.onmessage = (event: MessageEvent) => {
    // Handle incoming message
    const data = JSON.parse(event.data);
    console.log("Received data:", data);

    // Update receivedData with the parsed data
    receivedData.value = data;
  };

  socket.value.onerror = (error: Event) => {
    console.error("WebSocket error:", error);
  };

  socket.value.onclose = () => {
    console.log("WebSocket connection closed.");
  };
}

// Function to send updated vehicle data to the server
function sendUpdatedData() {
  if (socket.value) {
    // Create a vehicle data object using user input
    const updatedData = {
      key: key.value,
      speed: speed.value,
      pitch: pitch.value,
      yaw: yaw.value,
      roll: roll.value,
      altitude: altitude.value,
      batteryLife: batteryLife.value,
      lastUpdated: new Date().toLocaleTimeString(),
      currentPosition: {
        latitude: latitude.value,
        longitude: longitude.value
      },
      vehicleStatus: receivedData.value?.vehicleStatus ?? 0 // Use existing vehicle status if available
    };

    // Send the updated data to the server as a JSON string
    socket.value.send(JSON.stringify(updatedData));
    console.log("Sent updated data:", updatedData);
  }
}

// Establish the WebSocket connection when the component is created
establishWebSocketConnection();

// Close the WebSocket connection on component unmount
onBeforeUnmount(() => {
  if (socket.value) {
    socket.value.close();
  }
});
</script>

<template>
  <div class="w-full max-w-xl">
    <div class="text-xl font-bold">Vehicle Data</div>
    <div v-if="receivedData">
      <p><strong>Key:</strong> {{ receivedData.key }}</p>
      <p><strong>Speed:</strong> {{ receivedData.speed }}</p>
      <p><strong>Pitch:</strong> {{ receivedData.pitch }}</p>
      <p><strong>Yaw:</strong> {{ receivedData.yaw }}</p>
      <p><strong>Roll:</strong> {{ receivedData.roll }}</p>
      <p><strong>Altitude:</strong> {{ receivedData.altitude }}</p>
      <p><strong>Battery Life:</strong> {{ receivedData.batteryLife }}</p>
      <p><strong>Last Updated:</strong> {{ receivedData.lastUpdated }}</p>
      <p>
        <strong>Current Position:</strong>
        <span>Latitude: {{ receivedData.currentPosition.latitude }}</span>
        <span>Longitude: {{ receivedData.currentPosition.longitude }}</span>
      </p>
      <p><strong>Vehicle Status:</strong> {{ receivedData.vehicleStatus }}</p>
    </div>
    <div v-else>
      <p>No data received yet.</p>
    </div>
    <div class="text-xl font-bold">Update Vehicle Data</div>

    <form @submit.prevent="sendUpdatedData" class="grid gap-2">
      <div>
        <label>Key:</label>
        <NgInput type="text" v-model="key" />
      </div>
      <div>
        <label>Speed:</label>
        <NgInput type="number" v-model="speed" step="0.1" />
      </div>
      <div>
        <label>Pitch:</label>
        <NgInput type="number" v-model="pitch" step="0.1" />
      </div>
      <div>
        <label>Yaw:</label>
        <NgInput type="number" v-model="yaw" step="0.1" />
      </div>
      <div>
        <label>Roll:</label>
        <NgInput type="number" v-model="roll" step="0.1" />
      </div>
      <!-- <div>
        <label>Altitude:</label>
        <NgInput type="number" v-model="altitude" step="0.1" />
      </div> -->
      <!-- <div>
        <label>Battery Life:</label>
        <NgInput type="number" v-model="batteryLife" step="0.1" />
      </div> -->
      <!-- <div>
        <label>Latitude:</label>
        <NgInput type="number" v-model="latitude" step="0.1" />
      </div>
      <div>
        <label>Longitude:</label>
        <NgInput type="number" v-model="longitude" step="0.1" />
      </div> -->
      <NgButton type="submit">Send Updated Data</NgButton>
    </form>
  </div>
</template>
