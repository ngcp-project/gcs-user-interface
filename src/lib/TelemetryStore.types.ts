import {
  VehicleTelemetryData,
  VehicleEnum,
} from "@/lib/bindings"
import { LatLngExpression } from "leaflet"

export interface TelemetryStore { 
  // --------------------------
  // Sync with Rust backend state
  // --------------------------
  state: VehicleTelemetryData;
  syncRustState: (state: VehicleTelemetryData) => void;

  // --------------------------
  // Methods
  // --------------------------
  
  updateVehicleMarkers: (state: VehicleTelemetryData) => void;
  updateVehicleCoords: (vehicle: VehicleEnum, coords: LatLngExpression) => void;
  // simulateMovement: () => void;
  
  // Telemetry Data
  getTelemetry: () => VehicleTelemetryData; // Gets current state of telemetry
}