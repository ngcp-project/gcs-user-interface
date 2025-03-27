import { createStore, StoreApi } from "zustand/vanilla";
import {
  createTauRPCProxy,
  MissionsStruct,
  MissionStruct,
  StageStruct,
  VehicleEnum,
  VehicleStruct,
  ZonesStruct
} from "@/lib/bindings";
import { DeepReadonly, reactive, watch } from "vue";

const taurpc = createTauRPCProxy();

type ViewType = "mission" | "vehicle" | "stage" | "zone";
interface ViewState {
  currentView: ViewType;
  currentMissionId: number | null;
  currentVehicleName: VehicleEnum | null;
  currentStageId: number | null;
}

interface MissionStore {
  state: MissionsStruct;
  view: ViewState;

  setCurrentView: (view: ViewType) => void;
  setCurrentMissionID: (missionId: number) => void;
  setCurrentVehicleName: (vehicleName: VehicleEnum) => void;
  setCurrentStageID: (stageId: number) => void;

  getAllMissions: () => MissionStruct[];

  getMissionData: (missionId: number) => MissionStruct | undefined;
  setMissionData: (missionData: MissionStruct) => Promise<null>;
  createNewMission: (missionName: string) => Promise<null>;

  getVehicleData: (missionId: number, vehicleName: VehicleEnum) => VehicleStruct | undefined;
  // TODO:
  setVehicleStatus: (
    missionId: number,
    vehicleName: VehicleEnum,
    vehicleStatus: string
  ) => Promise<null>;

  getStageData: (
    missionId: number,
    vehicleName: VehicleEnum,
    stageId: number
  ) => StageStruct | undefined;
  setStageData: (
    missionId: number,
    vehicleName: VehicleEnum,
    stageId: number,
    stageData: StageStruct
  ) => Promise<null>;
  transitionStage: (missionId: number, vehicleName: VehicleEnum) => Promise<null>;
}

// Fetch initial state from backend
const initialState: MissionsStruct = await taurpc.mission.get_all_missions();

export const missionZustandStore = createStore<MissionStore>((set, get) => ({
  // Since we use missionsStruct as basis of store, we need to populate it
  // the backend state of the values
  state: initialState,
  //  Frontend State
  view: {
    currentView: "mission",
    currentMissionId: null,
    currentVehicleName: null,
    currentStageId: null
  },
  setCurrentView: (view: ViewType) => {
    set((state) => ({
      view: {
        ...state.view,
        currentView: view
      } satisfies ViewState
    }));
  },
  setVehicleStatus: async (missionId: number, vehicleName: VehicleEnum, vehicleStatus: string) => {
    return undefined as Promise<null>;
  },
  setStageData: async (
    missionId: number,
    vehicleName: VehicleEnum,
    stageId: number,
    stageData: StageStruct
  ) => {
    return undefined as Promise<null>;
  },
  transitionStage: async (missionId: number, vehicleName: VehicleEnum) => {
    return await taurpc.mission.transition_stage(missionId, vehicleName);
  },

  getAllMissions: () => get().state.missions,
  getMissionData: (missionId: number) =>
    get().state.missions.find((mission) => mission.mission_id === missionId),

  getVehicleData: (missionId: number, vehicleName: VehicleEnum) =>
    get().state.missions.find((mission) => mission.mission_id === missionId)?.vehicles[vehicleName],
  getStageData: (missionId: number, vehicleName: VehicleEnum, stageId: number) =>
    get()
      .state.missions.find((mission) => mission.mission_id === missionId)
      ?.vehicles[vehicleName].stages.find((stage) => stage.stage_id === stageId),
  createNewMission: async (missionName: string) => {
    // TODO: Uses random integer id, change when database is done
    const mission_id = Math.floor(Math.random() * 1000);
    return await taurpc.mission.create_mission(missionName, mission_id);
  },

  setMissionData: async (missionData: MissionStruct) =>
    await taurpc.mission.set_mission_data(missionData),

  setCurrentMissionID: (missionId: number) =>
    set((state) => ({
      view: {
        ...state.view,
        currentMissionId: missionId
      } satisfies ViewState
    })),

  setCurrentVehicleName: (vehicleName: VehicleEnum) => {
    if (get().view.currentMissionId === null) throw new Error("No mission selected");

    set((state) => ({
      view: {
        ...state.view,
        currentVehicleName: vehicleName
      } satisfies ViewState
    }));
  },

  setCurrentStageID: (stageId: number) => {
    if (get().view.currentMissionId === null) {
      throw new Error("No mission selected");
    }
    if (get().view.currentVehicleName === null) {
      throw new Error("No vehicle selected");
    }
    set((state) => ({
      view: {
        ...state.view,
        currentStageId: stageId
      } satisfies ViewState
    }));
  }
}));

// listen to zustandStore changes and update the reactive object
missionZustandStore.subscribe((newState) => {
  const clone = structuredClone(newState);
  Object.assign(missionStore, clone);
  console.log("zustand change");
});

// use replace to true to replace the state instead of shallow merging to allow vue to detect nested changes

// SetState expects the entire state object, but spreading the state object does a shallow merge even with replace set to true
// so we define MissionStore as a partial as a bandaid fix
// I have no idea why this happens or if theres a better alternative
// If you're reading this in the future and have a better solution please let me know

// On initial page load, fetch the mission data from the backend
taurpc.mission.get_all_missions().then((data) => {
  console.log("Mission data fetched:", data);
  (missionZustandStore as StoreApi<Partial<MissionStore>>).setState({ state: data }, true);
});

// On mission data update from backend, update the store
taurpc.mission.on_updated.on((data: MissionsStruct) => {
  console.log("Mission data updated:", data);
  (missionZustandStore as StoreApi<Partial<MissionStore>>).setState({ state: data }, true);
});

// convert zustandStore to a reactive object that triggers rerenders
// to avoid desync make reactive state readonly
export const missionStore: DeepReadonly<MissionStore> = reactive(missionZustandStore.getState());
