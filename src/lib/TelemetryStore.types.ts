import {
  VehicleTelemetryData,
  VehicleEnum,
} from "@/lib/bindings"

export interface TelemetryStore { 
  // --------------------------
  // Sync with Rust backend state
  // --------------------------
  state: VehicleTelemetryData;
  syncRustState: (state: VehicleTelemetryData) => void;

  // --------------------------
  // Methods
  // --------------------------
  
  // Telemetry Data
  getTelemetry: () => VehicleTelemetryData; // Gets current state of telemetry
}