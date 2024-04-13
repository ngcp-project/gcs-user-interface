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

    // continuously send random battery values to client
    setInterval(() => {
        battery = Math.floor((Math.random() * (100 - 0 + 1)) + 0);
        socket.send(battery);
     }, 1000);

    // continuously send random coordinate values to client
    // setInterval(() => {
    //     longitute = Number(((Math.random() * (180 - (-180) + 1)) + (-180)).toFixed(6));
    //     latitutde = Number(((Math.random() * (180 - (-180) + 1)) + (-180)).toFixed(6));
    //     let testCoordinateObject = {
    //         longitude: longitute,
    //         latitude: latitutde
    //     } 
    //     socket.send(JSON.stringify(testCoordinateObject));       
    // }, 100);

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