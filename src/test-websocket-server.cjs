// Import required modules
const express = require("express");
const http = require("http");
const WebSocket = require("ws");

// Create an Express app instance
const app = express();
// Create an HTTP server using the Express app instance
const server = http.createServer(app);

// Create a WebSocket server instance and attach it to the HTTP server
const websocketServer = new WebSocket.Server({ server });

let battery = 0;
//Listen for WebSocket connections
websocketServer.on('connection', (socket) => {
    // Log a message when a new client connects
    console.log('client connected.');

    let vehicleData = {
        battery: 6.6,
        currentPosition: {
            latitude: 7.7,
            longitude: 8.8,
        },
        dummy_connection: 0,
        vehicleStatus: 0,
    }

    // continuously send random battery values to client
    var count = 89;
    var counterIncrement = -1;
    const testBattery = setInterval(function() {
        count = count+counterIncrement;
        if (count == 0 || count == 100 ) {
            counterIncrement = -counterIncrement;
        }
        vehicleData.battery = count;
        socket.send(JSON.stringify(vehicleData));
    }, 100);

    // continuously send random coordinate values to client
    setInterval(() => {
        vehicleData.currentPosition.longitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)).toFixed(6));
        vehicleData.currentPosition.latitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)).toFixed(6));
        socket.send(JSON.stringify(vehicleData));
    }, 100);

    // continuously send random connection values to client
    setInterval(() => {
        vehicleData.dummy_connection = Math.floor((Math.random() * (100 - 0 + 1)) + 0);
        socket.send(JSON.stringify(vehicleData));
     }, 1000);

    // Listen for incoming WebSocket messages
    socket.on('message', (message) => {
        socket.send(`Your message ${message}`);
    });
    // Listen for WebSocket connection close events
    socket.on('close', () => {
      // Log a message when a client disconnects
      console.log('Client disconnected');
    });
});

// Start the server listening on port 3000
server.listen(3000, () => {
    console.log("Websocket server started on port 3000");
});