const WebSocket = require('ws');

const port = 5136;

// Define the server's local IP address
const serverIpAddress = 'localhost';

// Create a function to initialize a new WebSocket server
function initializeWebSocketServer(host, port, path) {
    const server = new WebSocket.Server({ host, port, path });
    console.log(`Mock WebSocket server listening on ws://${host}:${port}${path}`);

    server.on('connection', (ws) => {
        console.log(`New client connected on ${path}`);
        ws.on('close', () => {
            console.log(`Client disconnected from ${path}`);
        });
    });

    return server;
}

const ERU_server = initializeWebSocketServer(serverIpAddress, port, '/ws/eru');

const vehicleStatuses = ['In Use', 'Standby', 'Emergency'];
// Event listener for new client connections
ERU_server.on('connection', (ws) => {
    console.log('New ERU client connected');

    // Sample vehicle data object
    let vehicleData = {
        key: 'ERU',
        speed: 1.1,
        pitch: 2.2,
        yaw: 3.3,
        roll: 4.4,
        altitude: 5.5,
        batteryLife: .1,
        lastUpdated: '00:00:00',
        currentPosition: {
            latitude: 7.7,
            longitude: 8.8,
        },
        dummyConnection: 0,
        vehicleStatus: 'Standby',
    };

    // Send the initial sample data to the client
    ws.send(JSON.stringify(vehicleData));
     // continuously send random battery values to client
    setInterval(() => {
        vehicleData.speed = Math.random() * 150;
        vehicleData.yaw = Math.random() * 30;
        vehicleData.altitude = 3423;
        vehicleData.batteryLife = Math.random().toFixed(2);
        vehicleData.currentPosition.longitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));
        vehicleData.currentPosition.latitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));
        vehicleData.lastUpdated = new Date().toLocaleTimeString();
        vehicleData.dummyConnection = Math.floor((Math.random() * (100 - 0 + 1)) + 0);
        vehicleData.vehicleStatus = vehicleStatuses[Math.floor(Math.random() * vehicleStatuses.length)];

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
        console.log('ERU Client disconnected');
    });
});