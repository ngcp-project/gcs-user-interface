const connections = [
    'ws://localhost:5135/ws/eru',
    'ws://localhost:5136/ws/mea',
    'ws://localhost:5137/ws/fra',
    'ws://localhost:5138/ws/mra',
]

let wsConnections: { [key: string]: WebSocket } = {};       // dictionary where vehicle name is the key (eru, fra, etc), and its websocket connection is the value
let vehicleKeys = ['eru', 'mea', 'fra', 'mra']

// this initializes 4 websocket connections for each endpoint (each of the 4 vehicles) and store them in the wsConnections dictionary || This is called in main.ts
export function initializeWSConnections() {
    for (let i = 0; i < connections.length; i++) {
        let newWebSocket = new WebSocket(connections[i]);
        let vehicleName = vehicleKeys[i];
        wsConnections[vehicleName] = newWebSocket;
    }
    console.log("Initialize 4 websocket connections from main.ts")
    // for (const [key, value] of Object.entries(wsConnections)) {
    //     console.log("Created websocket connection from UI to mock server for " + key);
    // }
}

// this returns wsConnections (dictionary containing the 4 websocket connections)
export function getAllConnections() {
    return wsConnections;
}

// this returns a single websocket connection for a specific vehicle
export function getConnection(vehicleKey: string): WebSocket {
    return wsConnections[vehicleKey];
}

// this closes all 4 websocket connections
export function closeConnections() {
    for (let vehicleKey in wsConnections) {
        console.log("Closing websocket connection for " + vehicleKey);
        wsConnections[vehicleKey].close();
    }
}
