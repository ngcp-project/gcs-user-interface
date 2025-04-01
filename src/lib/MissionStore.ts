import { createStore } from "zustand/vanilla";
import {
  createTauRPCProxy,
  MissionsStruct,
  MissionStruct,
  StageStruct,
  VehicleEnum
} from "@/lib/bindings";
import { DeepReadonly, reactive } from "vue";
import { MissionStore, ViewState, ViewType } from "@/lib/MissionStore.types";

const taurpc = createTauRPCProxy();

// Fetch initial state from backend
const initialState: MissionsStruct = await taurpc.mission.get_all_missions();

export const missionZustandStore = createStore<MissionStore>((set, get) => ({
  // Backend States
  state: initialState, // Synced with rust state
  // method to update the store state with rust state
  syncRustState: (rustState: MissionsStruct) => {
    set(
      () =>
        ({
          state: rustState
        }) satisfies Partial<MissionStore>
    );
  },
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

  addStage: async (missionId: number, vehicleName: VehicleEnum) =>
    await taurpc.mission.add_stage(missionId, vehicleName, "New Stage"),
  deleteStage: async (missionId: number, vehicleName: VehicleEnum, stageId: number) =>
    await taurpc.mission.delete_stage(missionId, vehicleName, stageId),
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

  setCurrentMissionID: (missionId: number | null) =>
    set((state) => ({
      view: {
        ...state.view,
        currentMissionId: missionId
      } satisfies ViewState
    })),

  setCurrentVehicleName: (vehicleName: VehicleEnum | null) => {
    if (get().view.currentMissionId == undefined || get().view.currentMissionId === null)
      throw new Error("No mission selected");

    set((state) => ({
      view: {
        ...state.view,
        currentVehicleName: vehicleName
      } satisfies ViewState
    }));
  },

  setCurrentStageID: (stageId: number | null) => {
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
  // overwrite the reactive vue object and replace it with
  // the new store from zustand
  Object.assign(missionStore, newState);
  console.log("zustand change");
});

// ok so this will probably get lost to time but ive been working on this for a week straight
// we HAVE to use a setRustState within the store and we cant utilize a setState() otherwise
// it runs into weird issues with shallow merging, object overwriting, and issues triggering
// rerenders for dependencies in components

// Never EVER use missionZustandStore.setState() or else
// view properties both exist and dont exist, rust and frontend desync, etc.

// On initial page load, fetch the mission data from the backend
taurpc.mission.get_all_missions().then((data) => {
  console.log("Mission data fetched:", data);
  missionZustandStore.getState().syncRustState(data);
});

// On mission data update from backend, update the store
taurpc.mission.on_updated.on((data: MissionsStruct) => {
  console.log("Mission data updated:", data);
  missionZustandStore.getState().syncRustState(data);
});

// convert zustandStore to a reactive vue object that triggers rerenders
// to avoid frontend from modifying "private" properties (causes desync in state property)
// make reactive state readonlyi
// also use .getState() since we only ever read properties
export const missionStore: DeepReadonly<MissionStore> = reactive(missionZustandStore.getState());
