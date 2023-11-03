<script setup lang="ts">
import { ref } from "vue";

const serverMessage = ref("");
const messageInput = ref("");

// Connect to the WebSocket server with the URL from the .env file
const webSocket = new WebSocket(import.meta.env.VITE_WS_URL);

// On connection, listen for any message sent from the server
webSocket.addEventListener("message", (event: { data: string }) => {
  serverMessage.value = `Database updated: ${event.data}`;
});

// when form submitted send messageInput to server
function updateMessage() {
  webSocket.send(messageInput.value);
}
</script>

<template>
  <form class="row" @submit.prevent="updateMessage">
    <input
      id="messageInput"
      v-model="messageInput"
      placeholder="Send a Message"
    />
    <button type="submit">Send</button>
  </form>

  <p>{{ serverMessage }}</p>
</template>
