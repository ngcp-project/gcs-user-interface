import { createStore } from "zustand/vanilla";
import {
  createTauRPCProxy,
  VehicleTelemetryData,
  VehicleEnum
} from "@/lib/bindings";
import { DeepReadonly, reactive } from "vue";
import { TelemetryStore } from "@/lib/TelemetryStore.types";
import mapStore from "./MapStore";
import { LatLngExpression } from "leaflet";

// --------------------------
// Create TauRPC proxy
// --------------------------
const taurpc = createTauRPCProxy();

// Fetch initial telemetry state from backend.
// This is assumed to return a VehicleTelemetryData structure
const initialState: VehicleTelemetryData = await taurpc.telemetry.get_default_data();

// =============================================
// Zustand Store
// =============================================
export const telemetryZustandStore = createStore<TelemetryStore>((set, get) => ({
  // --------------------------
  // Backend State
  // --------------------------
  state: initialState,
  
  // Sync new telemetry state from backend updates
  syncRustState: (rustState: VehicleTelemetryData) => {
    set(() => ({
      state: rustState,
    }));

    // Update vehicle markers
    get().updateVehicleMarkers(rustState);
  },

  // Update vehicle markers based on telemetry data
  updateVehicleMarkers: (rustState: VehicleTelemetryData) => {
    Object.entries(rustState).forEach(([vehicle, data]) => {
      if (data.current_position) {
        // Convert vehicle name to match VehicleEnum
        const vehicleEnum = vehicle.toUpperCase() as VehicleEnum;
        // Dynamically update coordinates from telemetry data
        mapStore.updateMarkerCoords(
          vehicleEnum,
          [data.current_position.latitude, data.current_position.longitude]
        );
      }
    });
  },

  updateVehicleCoords: (vehicle: VehicleEnum, coords: LatLngExpression) => {
    // Update marker position in MapStore
    if (Array.isArray(coords) && coords.length === 2) {
      mapStore.updateMarkerCoords(
        vehicle,
        coords
      );
    }
  },

// ===============================================
// Movement Simulation --- FOR TESTING ONLY
// Uncomment when testing movement simulation
// ===============================================
  // Function to simulate dynamic movement
  // simulateMovement: () => {
  //   const baseLat = 33.932573934575075;
  //   const baseLng = -117.63059569114814;
    
  //   // Update each vehicle's position with slight offset
  //   const vehicles = ['ERU', 'MEA', 'MRA'] as const;
  //   vehicles.forEach((vehicle, index) => {
  //     const offset = (index - 1) * 0.001; 
  //     const newCoords: LatLngExpression = [
  //       baseLat + (Math.random() * 0.001),
  //       baseLng + offset + (Math.random() * 0.001)
  //     ];
      
  //     mapStore.updateMarkerCoords(vehicle, newCoords);
  //     console.log(`Updated ${vehicle} position to:`, newCoords);
  //   });
  // },

  // --------------------------
  // Telemetry Data Getters
  // --------------------------
  getTelemetry: () => {
    return get().state;
  },
}));

// =============================================
// Backend Event Listeners
// ===============================================
// Use get_default_data() if get_telemetry() returns nothing.
taurpc.telemetry.get_telemetry().then((data) => {
  if (!data) {
    taurpc.telemetry.get_default_data().then((defaultData) => {
      telemetryZustandStore.getState().syncRustState(defaultData);
      console.log("Telemetry data is empty, using default data", defaultData);
    });
  }
  console.log("Telemetry data received", data);
  telemetryZustandStore.getState().syncRustState(data);
});

taurpc.telemetry.on_updated.on((data: VehicleTelemetryData) => {
  console.log("Telemetry data updated", data);
  telemetryZustandStore.getState().syncRustState(data);
});

// ===============================================
// Reactive Vue Zustand Store
// ===============================================
// Subscribe to Zustand store changes and assign to a reactive Vue object.
telemetryZustandStore.subscribe((newState) => {
  Object.assign(telemetryStore, newState);
  console.log("Telemetry Zustand Store updated", telemetryStore);
});

// Make store properties readonly to avoid unintended modifications.
export const telemetryStore: DeepReadonly<TelemetryStore> = reactive(telemetryZustandStore.getState());

// ===============================================
// Movement Simulation --- FOR TESTING ONLY
// Uncomment when testing movement simulation
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