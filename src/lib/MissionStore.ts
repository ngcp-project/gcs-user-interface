import { createStore } from "zustand/vanilla";
import {
  createTauRPCProxy,
  MissionsStruct,
  MissionStruct,
  StageStruct,
  VehicleEnum,
  VehicleStruct,
  ZonesStruct
} from "@/lib/bindings";

const taurpc = createTauRPCProxy();

// extend MissionsStruct with additional zustand methods
interface missionStore extends MissionsStruct {
  mission_data: (mission_id: number) => MissionStruct;
  vehicle_data: (mission_id: number, vehicle_name: VehicleEnum) => VehicleStruct;
  stage_data: (mission_id: number, vehicle_name: VehicleEnum, stage_id: number) => StageStruct;
  zone_data: (mission_id: number) => ZonesStruct;
}

// Fetch initial state from backend
const initialState: MissionsStruct = await taurpc.mission.get_all_missions();

const missionStore = createStore<missionStore>((set, get) => ({
  // Since we use missionsStruct as basis of store, we need to populate it
  // the state of the values
  ...initialState,
  mission_data: (mission_id: number) => {
    return get().missions[mission_id];
  },
  vehicle_data: (mission_id: number, vehicle_name: VehicleEnum) => {
    const vehicles = get().missions[mission_id].vehicles;
    return vehicles[vehicle_name];
  },
  stage_data: (mission_id: number, vehicle_name: VehicleEnum, stage_id: number) => {
    const vehicle = get().vehicle_data(mission_id, vehicle_name);
    return vehicle.stages[stage_id];
  },
  zone_data: (mission_id: number) => {
    const zones = get().missions[mission_id].zones;
    return zones;
  }
}));

// On initial page load, fetch the mission data from the backend
taurpc.mission.get_all_missions().then((data) => {
  console.log("Mission data fetched:", data);
  missionStore.setState({ ...data });
});

// On mission data update from backend, update the store
taurpc.mission.on_updated.on((data: MissionsStruct) => {
  console.log("Mission data updated:", data);
  missionStore.setState({ ...data });
});

export const { getState, subscribe } = missionStore;
