import { createTauRPCProxy, MissionsStruct } from "./bindings";
import { missionPiniaStore } from "./MissionStore";
import { mapPiniaStore } from "./MapStore";
import { telemetryPiniaStore } from "./TelemetryStore";
import { VehicleTelemetryData } from "./bindings";

//Declare store variables:
let missionStore: ReturnType<typeof missionPiniaStore>;
let mapStore: ReturnType<typeof mapPiniaStore>;
let telemetryStore: ReturnType<typeof telemetryPiniaStore>;

//Establish taurpc connections.
export const establishTaurpcConnection = () => {
  //Assign stores to their respective useStores():
  missionStore = missionPiniaStore();
  mapStore = mapPiniaStore();
  telemetryStore = telemetryPiniaStore();

// ===============================================
// Backend Event Listeners
// ===============================================
  const taurpc = createTauRPCProxy();
  taurpc.mission.get_all_missions().then((data) => {
    console.log("PINIA: Mission data fetched:", data);
    missionStore!.syncRustState(data);
  });

  taurpc.mission.on_updated.on((data: MissionsStruct) => {
    console.log("PINIA: Mission data updated:", data);
    missionStore!.syncRustState(data);
  });

  taurpc.telemetry.get_telemetry().then((data) => {
    if (!data) {
      taurpc.telemetry.get_default_data().then((defaultData) => {
        telemetryStore.syncRustState(defaultData);
        console.log("PINIA: Telemetry data is empty, using default data", defaultData);
      });
    }
    console.log("PINIA: Telemetry data received", data);
    telemetryStore.syncRustState(data);
  });
  
  taurpc.telemetry.on_updated.on((data: VehicleTelemetryData) => {
    console.log("PINIA: Telemetry data updated", data);
    telemetryStore.syncRustState(data);
  });
  
  // =============================================
  // Subscriptions
  // =============================================
  //Per every update of missionStore, update mapStore.
  missionStore.$subscribe((mutation, state) => {
    mapStore.updateLayerTracking(state.missionState as MissionsStruct);
  });
};
// ===============================================
// Movement Simulation --- FOR TESTING ONLY
// Uncomment when testing movement simulation and Uncomment Lines 52-73 and 90 from TelemetryStore.ts
// ===============================================
// let movementInterval: NodeJS.Timeout | undefined;

// export const startMovementSimulation = () => {
//   // Clear any existing interval
//   if (movementInterval) {
//     clearInterval(movementInterval);
//   }
  
//   movementInterval = setInterval(() => {
//     telemetryStore.simulateMovement();
//   }, 5000); // Update every 5 seconds
// };

// export const stopMovementSimulation = () => {
//   if (movementInterval) {
//     clearInterval(movementInterval);
//     movementInterval = undefined;
//   }
// };

// startMovementSimulation();
export { missionStore, mapStore, telemetryStore };

