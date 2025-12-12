import {
  createTauRPCProxy,
  VehicleTelemetryData,
  VehicleEnum
} from "@/lib/bindings";
import { ref, computed } from "vue";
import { LatLngExpression } from "leaflet";
import { defineStore } from "pinia";
import { mapPiniaStore } from "./MapStore";

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
export const telemetryPiniaStore = defineStore('telemetry', ()=>{
  const telemetryState = ref<VehicleTelemetryData | null>(initialState);
  const mapStore = mapPiniaStore();
  const syncRustState = (rustState: VehicleTelemetryData) => {
    telemetryState.value = rustState;
    // Update vehicle markers
    Object.entries(rustState).forEach(([vehicle, data]) => {
      if (data.current_position) {
        // Convert vehicle name to match VehicleEnum
        const vehicleEnum = vehicle.toUpperCase() as VehicleEnum;
        // Dynamically update coordinates from telemetry data
        mapStore.updateVehicleMarker(
          vehicleEnum,
          data.current_position.latitude,
          data.current_position.longitude
        );
      }
    });
  }
  const updateVehicleCoords = (vehicle: VehicleEnum, coords: LatLngExpression) => {
    // Update marker position in MapStore
    if (Array.isArray(coords) && coords.length === 2) {
      mapStore.updateMarkerCoords(
        vehicle,
        coords
      );
    }
  }

// ===============================================
// Movement Simulation --- FOR TESTING ONLY
// Uncomment when testing movement simulation (Remember to uncomment Line 93 aswell from the return and Lines 57-81 from StoresSync.ts)
// ===============================================
//   Function to simulate dynamic movement
  // const simulateMovement = () => {
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
  // }

  // --------------------------
  // Telemetry Data Getters
  // --------------------------
  const getTelemetry = () => {
    return computed(()=>telemetryState.value);
  }
  const getVehicle = (vehicle: VehicleEnum) => {
    return computed(()=>telemetryState.value![vehicle]);
  }
  return {
    telemetryState,
    syncRustState,
    updateVehicleCoords,
    getTelemetry,
    getVehicle,
    // simulateMovement
  }
})