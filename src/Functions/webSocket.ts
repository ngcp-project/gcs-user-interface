const connecton_url = 'ws://localhost:5136/ws/eru';

let wsConnection: WebSocket;

// initializes 4 websocket connection
export function initializeWSConnection() {
    wsConnection = new WebSocket(connecton_url);
}

// returns websocket connection
export function getConnection(): WebSocket {
    return wsConnection;
}

// closes all 4 websocket connections
// export function closeConnections() {
//     for (let vehicleKey in wsConnections) {
//         console.log("Closing websocket connection for " + vehicleKey);
//         wsConnections[vehicleKey].close();
//     }
// }