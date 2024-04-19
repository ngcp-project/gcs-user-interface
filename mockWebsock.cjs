const WebSocket = require('ws');

// Define the WebSocket server port
const PORT = 5135;

// Define the server's local IP address
const serverIpAddress = 'localhost';

// Create a new WebSocket server using the server's local IP address and port
const wss = new WebSocket.Server({ host: serverIpAddress, port: PORT });

console.log(`Mock WebSocket server listening on ws://${serverIpAddress}:${PORT}/ws`);

// Event listener for new client connections
wss.on('connection', (ws) => {
    console.log('New client connected');

    // Sample vehicle data object
    let vehicleData = {
        key: 'ERU',
        speed: 1.1,
        pitch: 2.2,
        yaw: 3.3,
        roll: 4.4,
        altitude: 5.5,
        batteryLife: 6.6,
        lastUpdated: '00:00:00',
        currentPosition: {
            latitude: 7.7,
            longitude: 8.8,
        },
        dummyConnection: 0,
        vehicleStatus: 0,
    };

    // Send the initial sample data to the client
    ws.send(JSON.stringify(vehicleData));
     // continuously send random battery values to client
    setInterval(() => {
        vehicleData.altitude = Math.random() * 10000; 
        // vehicleData.batteryLife = Math.floor((Math.random() * (1 - 0 + 1)) + 0);
        vehicleData.batteryLife = Math.round(Math.random() * (100)) / 100;
        vehicleData.currentPosition.longitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)).toFixed(6));
        vehicleData.currentPosition.latitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)).toFixed(6));
        vehicleData.lastUpdated = new Date().toLocaleTimeString();
        vehicleData.dummyConnection = Math.floor((Math.random() * (100 - 0 + 1)) + 0);

        ws.send(JSON.stringify(vehicleData));
    }, 1000);

    // Event listener for messages from the client
    ws.on('message', (message) => {
        console.log(`Received message from client: ${message}`);

        // Parse the received message as JSON
        const updatedData = JSON.parse(message);

        // Update the vehicleData object with the received data
        vehicleData = {
            ...vehicleData,
            ...updatedData, // Merge the updated data into the existing data
            lastUpdated: new Date().toLocaleTimeString() // Update the last updated time
        };

        // Print out the updated data
        console.log('Updated vehicle data:', vehicleData);

        // Optionally, you can send the updated data back to the client
        ws.send(JSON.stringify(vehicleData));
    });

    // Event listener for client disconnection
    ws.on('close', () => {
        console.log('Client disconnected');
    });
});
