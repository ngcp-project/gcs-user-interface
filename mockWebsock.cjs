const WebSocket = require('ws');

const ports = [5135, 5136, 5137, 5138];

// Define the server's local IP address
const serverIpAddress = 'localhost';

// Create a function to initialize a new WebSocket server
function initializeWebSocketServer(host, port, path) {
    const server = new WebSocket.Server({ host, port, path });
    console.log(`Mock WebSocket server listening on ws://${host}:${port}${path}`);

    server.on('connection', (ws) => {
        console.log(`New client connected on ${path}`);
        // Implement server handling for each connection
        // ...
        ws.on('close', () => {
            console.log(`Client disconnected from ${path}`);
        });
    });

    return server;
}

// Initialize the WebSocket servers with appropriate port and path for each server
const ERU = initializeWebSocketServer(serverIpAddress, ports[0], '/ws/eru');
const MEA = initializeWebSocketServer(serverIpAddress, ports[1], '/ws/mea');
const FRA = initializeWebSocketServer(serverIpAddress, ports[2], '/ws/fra');
const MRA = initializeWebSocketServer(serverIpAddress, ports[3], '/ws/mra');

// Event listener for new client connections
ERU.on('connection', (ws) => {
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
        vehicleStatus: 0,
    };

    // Send the initial sample data to the client
    ws.send(JSON.stringify(vehicleData));
     // continuously send random battery values to client
    setInterval(() => {
        vehicleData.altitude = Math.random() * 10000; 
        // vehicleData.batteryLife = Math.floor((Math.random() * (1 - 0 + 1)) + 0);
        vehicleData.batteryLife = Math.random().toFixed(2);
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
        console.log('ERU Client disconnected');
    });
});

// Event listener for new client connections
MEA.on('connection', (ws) => {
    console.log('New MEA client connected');

    // Sample vehicle data object
    let vehicleData = {
        key: 'MEA',
        speed: 1.1,
        pitch: 2.2,
        yaw: 3.3,
        roll: 4.4,
        altitude: 5.5,
        batteryLife: .2,
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
        vehicleData.batteryLife = Math.random().toFixed(2);
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
        console.log('MEA Client disconnected');
    });
});

// Event listener for new client connections
FRA.on('connection', (ws) => {
    console.log('New FRA client connected');

    // Sample vehicle data object
    let vehicleData = {
        key: 'FRA',
        speed: 1.1,
        pitch: 2.2,
        yaw: 3.3,
        roll: 4.4,
        altitude: 5.5,
        batteryLife: .8,
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
        vehicleData.batteryLife = Math.random().toFixed(2);
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
        console.log('FRA Client disconnected');
    });
});

// Event listener for new client connections
MRA.on('connection', (ws) => {
    console.log('New client connected');

    // Sample vehicle data object
    let vehicleData = {
        key: 'MRA',
        speed: 1.1,
        pitch: 2.2,
        yaw: 3.3,
        roll: 4.4,
        altitude: 5.5,
        batteryLife: .6,
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
        vehicleData.batteryLife = Math.random().toFixed(2);
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
        console.log('MRA Client disconnected');
    });
});