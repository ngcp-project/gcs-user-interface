import WebSocket from 'ws';

// this file was just to test receiving stuff from websocket connection. WebSocket connection is being used in StaticScreen.vue
let client = new WebSocket('ws://localhost:3000/');
client.on('open', () => {
    client.send('Hello');
})

client.addEventListener("message", (event) => {
    console.log(`Message from server: ${event.data}`);
});
