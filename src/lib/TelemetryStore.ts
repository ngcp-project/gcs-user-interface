import { createStore } from "zustand/vanilla";
import {
  createTauRPCProxy,
  VehicleTelemetryData,
  TelemetryData,
} from "@/lib/bindings";
import { DeepReadonly, reactive } from "vue";
import { TelemetryStore } from "@/lib/TelemetryStore.types";

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
  },
  
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
})

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