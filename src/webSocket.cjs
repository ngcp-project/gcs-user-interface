
const connections = [
    'ws://localhost:5135/ws/eru',
    'ws://localhost:5135/ws/fra',
    'ws://localhost:5135/ws/mea',
    'ws://localhost:5135/ws/mra',

]

let websocketConnections: never[] = [];
const telemetryData = [];


function getWebsocketConnection() {
    connections.forEach(connection => {
        let newWebSocket = new WebSocket(connection);
        websocketConnections.append(newWebSocket);  
    });

    websocketConnections.forEach(client => {
        client.addEventListener("message", (event) => {
            const receivedData = ref<any>(null);
            const data = JSON.parse(event.data);
            receivedData.value = data; 
    
            console.log("Received data from mockWebsock:", receivedData);
            telemetryData.append(receivedData);
        });
    });
    return telemetryData;
}

export function getWebsocketConnection;



