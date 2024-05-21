const WebSocket = require('ws');

const ports = [6135, 6136, 6137, 6138];
const vehicleStatuses = ['In Use', 'Standby', 'Emergency'];

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
        vehicleStatus: 1,
    };

    // Send the initial sample data to the client
    ws.send(JSON.stringify(vehicleData));
     // continuously send random battery values to client
    setInterval(() => {
        vehicleData.speed = Math.random() * 150;
        vehicleData.yaw = Math.random() * 30;
        vehicleData.altitude = 3423;
        // vehicleData.batteryLife = Math.floor((Math.random() * (1 - 0 + 1)) + 0);

        // send random timestamp/lastUpdated values
        const currentTime = Date.now();
        const random_change = Math.floor(Math.random() * 1000); 
        vehicleData.lastUpdated = currentTime - random_change;

        vehicleData.batteryLife = Math.random().toFixed(2);
        vehicleData.currentPosition.longitude = Number(((Math.random() * (0.01)) + (-0.005))) + -120.75365555392507;
        vehicleData.currentPosition.latitude = Number(((Math.random() * (0.01)) + (-0.005))) + 35.33358365056906;
        vehicleData.dummyConnection = Math.floor((Math.random() * (100 - 0 + 1)) + 0);
        // vehicleData.vehicleStatus = vehicleStatuses[Math.floor(Math.random() * vehicleStatuses.length)];
        vehicleData.vehicleStatus = Math.floor(Math.random() * 3);

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
        vehicleStatus: 1,
    };

    // Send the initial sample data to the client
    ws.send(JSON.stringify(vehicleData));
     // continuously send random battery values to client
    setInterval(() => {
        vehicleData.speed = Math.random() * 150; 
        vehicleData.yaw = 22;
        vehicleData.altitude = Math.random() * 10000; 
        // vehicleData.batteryLife = Math.floor((Math.random() * (1 - 0 + 1)) + 0);
        // vehicleData.batteryLife = Math.random().toFixed(2);
        vehicleData.batteryLife = 0;
        vehicleData.currentPosition.longitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));
        vehicleData.currentPosition.latitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));

        // send random timestamp/lastUpdated values
        const currentTime = Date.now();
        const random_change = Math.floor(Math.random() * 1000); 
        vehicleData.lastUpdated = currentTime - random_change;

        vehicleData.dummyConnection = Math.floor((Math.random() * (100 - 0 + 1)) + 0);
        // vehicleData.vehicleStatus = vehicleStatuses[Math.floor(Math.random() * vehicleStatuses.length)];
        vehicleData.vehicleStatus = Math.floor(Math.random() * 3);

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
        vehicleStatus: 1,
    };

    // Send the initial sample data to the client
    ws.send(JSON.stringify(vehicleData));
     // continuously send random battery values to client
    setInterval(() => {
        vehicleData.speed = Math.random() * 150; 
        vehicleData.yaw = Math.random() * 30;
        vehicleData.altitude = Math.random() * 10000; 
        // vehicleData.batteryLife = Math.floor((Math.random() * (1 - 0 + 1)) + 0);
        vehicleData.batteryLife = Math.random().toFixed(2);
        vehicleData.currentPosition.longitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));
        vehicleData.currentPosition.latitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));

        // send random timestamp/lastUpdated values
        const currentTime = Date.now();
        const random_change = Math.floor(Math.random() * 1000); 
        vehicleData.lastUpdated = currentTime - random_change;

        vehicleData.dummyConnection = Math.floor((Math.random() * (100 - 0 + 1)) + 0);
        // vehicleData.vehicleStatus = vehicleStatuses[Math.floor(Math.random() * vehicleStatuses.length)];
        vehicleData.vehicleStatus = Math.floor(Math.random() * 3);

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
    console.log('New MRA client connected');

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
        vehicleStatus: 1,
    };

    // Send the initial sample data to the client
    ws.send(JSON.stringify(vehicleData));
     // continuously send random battery values to client
    setInterval(() => {
        vehicleData.speed = Math.random() * 150; 
        vehicleData.yaw = Math.random() * 30;
        vehicleData.altitude = 4200; 
        // vehicleData.batteryLife = Math.floor((Math.random() * (1 - 0 + 1)) + 0);
        vehicleData.batteryLife = Math.random().toFixed(2);
        vehicleData.currentPosition.longitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));
        vehicleData.currentPosition.latitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));

        // send random timestamp/lastUpdated values
        const currentTime = Date.now();
        const random_change = Math.floor(Math.random() * 1000); 
        vehicleData.lastUpdated = currentTime - random_change;

        vehicleData.dummyConnection = Math.floor((Math.random() * (100 - 0 + 1)) + 0);
        // vehicleData.vehicleStatus = vehicleStatuses[Math.floor(Math.random() * vehicleStatuses.length)];
        vehicleData.vehicleStatus = Math.floor(Math.random() * 3);

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