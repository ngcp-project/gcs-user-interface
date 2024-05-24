const WebSocket = require('ws');

const ports = [6135, 6136, 6137, 6138];
const vehicleStatuses = ['In Use', 'Standby', 'Emergency'];

// random vehicle paths to test moving markers
const ERU_testpath = [[35.327625749440806, -120.75019461407126], [35.327725749440806, -120.75009461407126], [35.327825749440806, -120.74999461407126], [35.327925749440806, -120.74989461407126], [35.328025749440806, -120.74979461407126], [35.328125749440806, -120.74969461407126], [35.328225749440806, -120.74959461407126], [35.328325749440806, -120.74949461407126], [35.328425749440806, -120.74939461407126], [35.328525749440806, -120.74929461407126], [35.328625749440806, -120.74919461407126], [35.328725749440806, -120.74909461407126], [35.328825749440806, -120.74899461407126], [35.328925749440806, -120.74889461407126], [35.329025749440806, -120.74879461407126], [35.329125749440806, -120.74869461407126], [35.329225749440806, -120.74859461407126], [35.329325749440806, -120.74849461407126], [35.329425749440806, -120.74839461407126], [35.329525749440806, -120.74829461407126], [35.329625749440806, -120.74819461407126], [35.329725749440806, -120.74809461407126], [35.329825749440806, -120.74799461407126], [35.329925749440806, -120.74789461407126], [35.330025749440806, -120.74779461407126], [35.330125749440806, -120.74769461407126], [35.330225749440806, -120.74759461407126], [35.330325749440806, -120.74749461407126], [35.330425749440806, -120.74739461407126], [35.330525749440806, -120.74729461407126], [35.330625749440806, -120.74719461407126], [35.330725749440806, -120.74709461407126], [35.330825749440806, -120.74699461407126], [35.330925749440806, -120.74689461407126], [35.331025749440806, -120.74679461407126], [35.331125749440806, -120.74669461407126], [35.331225749440806, -120.74659461407126], [35.331325749440806, -120.74649461407126], [35.331425749440806, -120.74639461407126]];
const MEA_testpath = [[35.325174811774005, -120.74397086694808], [35.32522731516862, -120.74435882301494], [35.32527981856324, -120.7447467790818], [35.32533232195786, -120.74513473514866], [35.32538482535248, -120.74552269121552], [35.3254373287471, -120.74591064728238], [35.32548983214172, -120.74629860334924], [35.32554233553634, -120.7466865594161], [35.32559483893096, -120.74707451548296], [35.32564734232558, -120.74746247154982], [35.3256998457202, -120.74785042761668], [35.32575234911482, -120.74823838368354], [35.32580485250944, -120.7486263397504], [35.32585735590406, -120.74901429581726], [35.32590985929868, -120.74940225188412], [35.3259623626933, -120.74979020795098], [35.32601486608792, -120.75017816401784], [35.32606736948254, -120.7505661200847], [35.32611987287716, -120.75095407615156], [35.32617237627178, -120.75134203221842], [35.3262248796664, -120.75172998828528], [35.32627738306102, -120.75211794435214], [35.32632988645564, -120.752505900419], [35.32638238985026, -120.75289385648586], [35.32643489324488, -120.75328181255272], [35.3264873966395, -120.75366976861958], [35.32653990003412, -120.75405772468644], [35.32659240342874, -120.7544456807533], [35.32664490682336, -120.75483363682016], [35.32669741021798, -120.75522159288702], [35.3267499136126, -120.75560954895388], [35.32680241700722, -120.75599750502074], [35.32685492040184, -120.7563854610876]];
const MRA_testpath = [[35.32345911119576, -120.759057221687], [35.32354891487462, -120.7586504561236], [35.32363871855348, -120.7582436905602], [35.32372852223234, -120.7578369249968], [35.3238183259112, -120.7574301594334], [35.32390812959006, -120.75702339387], [35.32399793326892, -120.7566166283066], [35.32408773694778, -120.7562098627432], [35.32417754062664, -120.7558030971798], [35.3242673443055, -120.7553963316164], [35.32435714798436, -120.754989566053], [35.32444695166322, -120.7545828004896], [35.32453675534208, -120.7541760349262], [35.32462655902094, -120.7537692693628], [35.3247163626998, -120.7533625037994]]
const FRA_testpath = [[35.32559497779411, -120.73937843463776], [35.32558402809835, -120.74003406835225], [35.32557307830259, -120.74068970206674], [35.32556212850683, -120.74134533578123], [35.32555117871107, -120.74200096949572], [35.32554022891531, -120.74265660321021], [35.32552927911955, -120.7433122369247], [35.32551832932379, -120.74396787063919], [35.32550737952803, -120.74462350435368], [35.32549642973227, -120.74527913806817], [35.32548547993651, -120.74593477178266], [35.32547453014075, -120.74659040549715], [35.32546358034499, -120.74724603921164], [35.32545263054923, -120.74790167292613], [35.32544168075347, -120.74855730664062], [35.32543073095771, -120.74921294035511], [35.32541978116195, -120.7498685740696], [35.32540883136619, -120.75052420778409], [35.32539788157043, -120.75117984149858], [35.32538693177467, -120.75183547521307]]

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

    let counter = 0
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
        vehicleData.yaw = Math.random() * 360;
        vehicleData.pitch = Math.random() * 360;
        vehicleData.roll = Math.random() * 360;
        vehicleData.altitude = 3423;
        // vehicleData.batteryLife = Math.floor((Math.random() * (1 - 0 + 1)) + 0);

        // send random timestamp/lastUpdated values
        const currentTime = Date.now();
        const random_change = Math.floor(Math.random() * 1000); 
        vehicleData.lastUpdated = currentTime - random_change;

        vehicleData.batteryLife = Math.random().toFixed(2);
        // vehicleData.currentPosition.longitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));
        // vehicleData.currentPosition.latitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));
        // vehicleData.currentPosition.longitude = Number(((Math.random() * (0.01)) + (-0.005))) + -120.75365555392507;
        // vehicleData.currentPosition.latitude = Number(((Math.random() * (0.01)) + (-0.005))) + 35.33358365056906;

        // below is used to constantly loop through mock ERU path coordinates
        let coord = ERU_testpath[counter];
        vehicleData.currentPosition.latitude = coord[0];
        vehicleData.currentPosition.longitude = coord[1];
        counter += 1;
        if (counter == ERU_testpath.length) {
            counter = 0;
        }

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
    let counter = 0;

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
        vehicleData.yaw = Math.random() * 360;
        vehicleData.pitch = Math.random() * 360;
        vehicleData.roll = Math.random() * 360;
        vehicleData.altitude = Math.random() * 10000; 
        // vehicleData.batteryLife = Math.floor((Math.random() * (1 - 0 + 1)) + 0);
        // vehicleData.batteryLife = Math.random().toFixed(2);
        vehicleData.batteryLife = 0;
        // vehicleData.currentPosition.longitude = Number(((Math.random() * (0.01)) + (-0.005))) + -120.74489148571469;
        // vehicleData.currentPosition.latitude = Number(((Math.random() * (0.01)) + (-0.005))) + 35.32692711672588;

        // below is used to constantly loop through mock MEA path coordinates
        let coord = MEA_testpath[counter];
        vehicleData.currentPosition.latitude = coord[0];
        vehicleData.currentPosition.longitude = coord[1];
        counter += 1;
        if (counter == MEA_testpath.length) {
            counter = 0;
        }

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
    let counter = 0;

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
        fireCoordinates: {
            latitude: 1.0,
            longitude: 2.0,    
        },
    };

    // Send the initial sample data to the client
    ws.send(JSON.stringify(vehicleData));
     // continuously send random battery values to client
    setInterval(() => {
        vehicleData.speed = Math.random() * 150; 
        vehicleData.yaw = Math.random() * 360;
        vehicleData.pitch = Math.random() * 360;
        vehicleData.roll = Math.random() * 360;
        vehicleData.altitude = Math.random() * 10000; 
        // vehicleData.batteryLife = Math.floor((Math.random() * (1 - 0 + 1)) + 0);
        vehicleData.batteryLife = Math.random().toFixed(2);
        // vehicleData.currentPosition.longitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));
        // vehicleData.currentPosition.latitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));

        // below is used to constantly loop through mock ERU path coordinates
        let coord = FRA_testpath[counter];
        vehicleData.currentPosition.latitude = coord[0];
        vehicleData.currentPosition.longitude = coord[1];
        counter += 1;
        if (counter == FRA_testpath.length) {
            counter = 0;
        }

        //send random fire coordinate values
        vehicleData.fireCoordinates.longitude = Number(((Math.random() * (0.01)) + (-0.005))) + -120.75365555392507;
        vehicleData.fireCoordinates.latitude = Number(((Math.random() * (0.01)) + (-0.005))) + 35.33358365056906;

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

    let counter = 0;
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
        vehicleData.yaw = Math.random() * 360;
        vehicleData.pitch = Math.random() * 360;
        vehicleData.roll = Math.random() * 360;
        vehicleData.altitude = 4200; 
        // vehicleData.batteryLife = Math.floor((Math.random() * (1 - 0 + 1)) + 0);
        vehicleData.batteryLife = Math.random().toFixed(2);
        // vehicleData.currentPosition.longitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));
        // vehicleData.currentPosition.latitude = Number(((Math.random() * (180 - (-180) + 1)) + (-180)));

        // below is used to constantly loop through mock ERU path coordinates
        let coord = MRA_testpath[counter];
        vehicleData.currentPosition.latitude = coord[0];
        vehicleData.currentPosition.longitude = coord[1];
        counter += 1;
        if (counter == MRA_testpath.length) {
            counter = 0;
        }

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