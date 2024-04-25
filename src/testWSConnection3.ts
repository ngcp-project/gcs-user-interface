// import { reactive, ref } from 'vue';

// // initialize reactive variables for each vehicle's telemetry data (the object is reactive, so each key/value pair is also reactive)
// // const ERU_data = reactive({batteryPct: 0, connection: 0, coordinates: {longitude: 0, latitude: 0}, status: 'Standby'});
// // const MEA_data = reactive({batteryPct: 0, connection: 0, coordinates: {longitude: 0, latitude: 0}, status: 'Standby'});
// // const MRA_data = reactive({batteryPct: 0, connection: 0, coordinates: {longitude: 0, latitude: 0}, status: 'Standby'});
// // const FRA_data = reactive({batteryPct: 0, connection: 0, coordinates: {longitude: 0, latitude: 0}, status: 'Standby'});
// const ERU_data = ref<any>(null);
// const MEA_data = ref<any>(null);
// const MRA_data = ref<any>(null);
// const FRA_data = ref<any>(null);

// // dictionary where vehicle name is key and websocket URLs is value | put websocket URLs here
// const connections = {
//     'eru': 'ws://localhost:5135/ws/eru',
//     'mea': 'ws://localhost:5136/ws/mea',
//     'fra': 'ws://localhost:5137/ws/fra',
//     'mra': 'ws://localhost:5138/ws/mra',
// };
// let wsConnections: { [key: string]: WebSocket } = {};       // dictionary where vehicle name is the key (eru, fra, etc), and its websocket connection is the value

// // initializes 4 websocket connections for each endpoint (each of the 4 vehicles) and store them in the wsConnections dictionary
// function initializeWSConnections() {
//     // create WS connections
//     for (const [vehicleKey, connectionURL] of Object.entries(connections)) { 
//         let newWebSocket = new WebSocket(connectionURL);
//         wsConnections[vehicleKey] = newWebSocket;
//         console.log("Make connection for " + vehicleKey);
//     }

//     // add event listeners to update reactive variables
//     wsConnections['eru'].addEventListener("message", (event) => {
//         const receivedData = JSON.parse(event.data);
//         ERU_data.value = receivedData;      
//     })
//     wsConnections['mea'].addEventListener("message", (event) => {
//         const receivedData = JSON.parse(event.data);
//         MEA_data.value = receivedData;     
//     })
//     wsConnections['fra'].addEventListener("message", (event) => {
//         const receivedData = JSON.parse(event.data);
//         FRA_data.value = receivedData;  
//     })
//     wsConnections['mra'].addEventListener("message", (event) => {
//         const receivedData = JSON.parse(event.data);
//         MRA_data.value = receivedData;   
//     })
// }

// export { ERU_data, MEA_data, FRA_data, MRA_data };