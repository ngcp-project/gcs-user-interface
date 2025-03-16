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
import { reactive } from "vue";

const taurpc = createTauRPCProxy();

type ViewType = "mission" | "vehicle" | "stage" | "zone";
interface ClientMission extends MissionStruct {
  isSubmitted: boolean;
}
interface ViewState {
  currentView: ViewType;
  clientMissions: ClientMission[] | undefined;
  setCurrentView: (view: ViewType) => void;
  addMission: () => void;
}

// extend MissionsStruct with additional zustand methods
interface MissionStore {
  // assign all mission states to state
  state: MissionsStruct;
  view: ViewState;
  // Helper methods
  getMissionData: (mission_id: number) => MissionStruct | ClientMission | undefined;
  getVehicleData: (mission_id: number, vehicle_name: VehicleEnum) => VehicleStruct | undefined;
  getStageData: (
    mission_id: number,
    vehicle_name: VehicleEnum,
    stage_id: number
  ) => StageStruct | undefined;
  getZoneData: (mission_id: number) => ZonesStruct | undefined;
}

// Fetch initial state from backend
const initialState: MissionsStruct = await taurpc.mission.get_all_missions();

const missionZustandStore = createStore<MissionStore>((set, get) => ({
  // Since we use missionsStruct as basis of store, we need to populate it
  // the backend state of the values
  state: initialState,

  //  Frontend State
  view: {
    currentView: "mission",
    clientMissions: undefined,
    setCurrentView: (view: ViewType) =>
      set((state) => ({ view: { ...state.view, currentView: view } })),
    addMission: () => {
      // add mission to clientmissions
      // set mission to submitted
      // set current view to mission
      // const newMission: ClientMission = {
      //   mission_name: `Mission`,
      //   isSubmitted: false,
      //   mission_status: "Inactive" as const,
      //   zones: {
      //     keep_in_zones: [],
      //     keep_out_zones: []
      //   }
      // };
      set((state) => ({
        // clientMissions
      }));
    }
  },
  // Get Methods
  getMissionData: (mission_id: number) => {
    // check if clientmission is submitted
    // If it is submitted return what the backend state is
    if (get().view.clientMissions?.[mission_id]?.isSubmitted) {
      return get().state.missions[mission_id];
    }
    return get().view.clientMissions?.[mission_id];
  },
  getVehicleData: (mission_id: number, vehicle_name: VehicleEnum) => {
    const mission = get().getMissionData(mission_id);

    return mission?.vehicles[vehicle_name];
  },
  getStageData: (mission_id: number, vehicle_name: VehicleEnum, stage_id: number) => {
    const vehicle = get().getVehicleData(mission_id, vehicle_name);

    return vehicle?.stages[stage_id];
  },
  getZoneData: (mission_id: number) => {
    const mission = get().getMissionData(mission_id);
    return mission?.zones;
  },
  // Set Methods
  // Submits a new mission
  submitMission: async (missionData: MissionStruct) => {
    await taurpc.mission.submit_mission(missionData);
  }
}));

export const missionStore = reactive(missionZustandStore.getState());

// export const useMissionStore = () => missionStore;

missionZustandStore.subscribe((newState) => {
  Object.assign(missionStore, newState);
});

// On initial page load, fetch the mission data from the backend
taurpc.mission.get_all_missions().then((data) => {
  console.log("Mission data fetched:", data);
  missionZustandStore.setState({ state: data });
});

// On mission data update from backend, update the store
taurpc.mission.on_updated.on((data: MissionsStruct) => {
  console.log("Mission data updated:", data);
  missionZustandStore.setState({ state: data });
});

// export const { getState, setState, subscribe } = missionStore;
