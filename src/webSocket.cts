const connections = [
    'ws://localhost:5135/ws/eru',
    'ws://localhost:5136/ws/mea',
    'ws://localhost:5137/ws/fra',
    'ws://localhost:5138/ws/mra',

]

let wsClientsList: { [key: string]: WebSocket } = {};
let test: { [key: string]: string } = {};

let names = [
    'eru',
    'mea',
    'fra',
    'mra'
]
const telemetryData = [];

function getWebsocketConnection(): string {
    for (let i = 0; i < connections.length; i++) {
        let newWebSocket = new WebSocket(connections[i]);
        let vehicleName = names[i];
        wsClientsList[names[i]] = newWebSocket;
        console.log(connections[i]);
    }

    for (const [key, value] of Object.entries(wsClientsList)) {
        console.log(key);
        // console.log(value);
    }

    // connections.forEach(connection => {
    //     // let newWebSocket = new WebSocket(connection);
    //     // websocketConnections.append(newWebSocket);  
    //     console.log("penis");
    // });
    return "hi";
}

let bruh = getWebsocketConnection();