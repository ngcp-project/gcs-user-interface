// dictionary where vehicle name is key and websocket URLs is value | put websocket URLs here
const connections = {
    'eru': 'ws://localhost:5135/ws/eru',
    'mea': 'ws://localhost:5135/ws/mea',
    'fra': 'ws://localhost:5135/ws/fra',
    'mra': 'ws://localhost:5135/ws/mra',
};
let wsConnections: { [key: string]: WebSocket } = {};       // dictionary where vehicle name is the key (eru, fra, etc), and its websocket connection is the value

// initializes 4 websocket connections for each endpoint (each of the 4 vehicles) and store them in the wsConnections dictionary || This is called in main.ts
export function initializeWSConnections() {
    for (const [vehicleKey, connectionURL] of Object.entries(connections)) { 
        let newWebSocket = new WebSocket(connectionURL);
        wsConnections[vehicleKey] = newWebSocket;
    }
}

// returns wsConnections (dictionary containing the 4 websocket connections)
export function getAllConnections() {
    return wsConnections;
}

// returns a single websocket connection for a specific vehicle
export function getConnection(vehicleKey: string): WebSocket {
    return wsConnections[vehicleKey];
}

// closes all 4 websocket connections
export function closeConnections() {
    for (let vehicleKey in wsConnections) {
        console.log("Closing websocket connection for " + vehicleKey);
        wsConnections[vehicleKey].close();
    }
}
