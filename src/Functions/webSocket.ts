// dictionary where vehicle name is key and websocket URLs is value | put websocket URLs here
// const connections = {
//     'eru': 'ws://localhost:5135/ws/eru',
//     'mea': 'ws://localhost:5135/ws/mea',
//     'fra': 'ws://localhost:5135/ws/fra',
//     'mra': 'ws://localhost:5135/ws/mra',
// };
const connections = {
    'eru': 'ws://localhost:6135/ws/eru',
    'mea': 'ws://localhost:6136/ws/mea',
    'fra': 'ws://localhost:6137/ws/fra',
    'mra': 'ws://localhost:6138/ws/mra',
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

// enum for vehicle status
enum VehicleStatus {
    InUse = 0,
    Standby = 1,
    EmergencyStopped = 2
}

export function getVehicleStatus(status_num: number) {
    if (status_num == VehicleStatus.InUse) {
        return "In Use";
    } else if (status_num == VehicleStatus.Standby) {
        return "Standby";
    } else if (status_num == VehicleStatus.EmergencyStopped) {
        return "Emergency";
    } else {
        return "Error";
    }
}