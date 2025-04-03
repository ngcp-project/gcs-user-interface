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


// =============================================
// Initialization
// ===============================================
const taurpc = createTauRPCProxy();

// Fetch initial state from backend
const initialState: MissionsStruct = await taurpc.mission.get_all_missions();

// =============================================
// Zustand Store
// ===============================================
export const missionZustandStore = createStore<MissionStore>((set, get) => ({

  // --------------------------
  // Backend State
  // --------------------------
  state: initialState, // Synced with rust state

  syncRustState: (rustState: MissionsStruct) => {
    set(
      () =>
        ({
          state: rustState
        }) satisfies Partial<MissionStore>
    );
  },

  // --------------------------
  // Frontend View State
  // --------------------------
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
  },

  // --------------------------
  // Mission Data
  // --------------------------
  getAllMissions: () => get().state.missions,

  getMissionData: (missionId: number) =>
    get().state.missions.find((mission) => mission.mission_id === missionId),

  setMissionData: async (missionData: MissionStruct) =>
    await taurpc.mission.set_mission_data(missionData),

  createNewMission: async (missionName: string) => {
    return await taurpc.mission.create_mission(missionName);
  },

  // --------------------------
  // Vehicle Data 
  // --------------------------
  getVehicleData: (missionId: number, vehicleName: VehicleEnum) =>
    get().state.missions.find((mission) => mission.mission_id === missionId)?.vehicles[vehicleName],

  setAutoMode: async (missionId: number, vehicleName: VehicleEnum, isAuto: boolean) => {
    return await taurpc.mission.set_auto_mode(missionId, vehicleName, isAuto);
  },

  // --------------------------
  // Stage Data 
  // --------------------------
  getStageData: (missionId: number, vehicleName: VehicleEnum, stageId: number) =>
    get()
      .state.missions.find((mission) => mission.mission_id === missionId)
      ?.vehicles[vehicleName].stages.find((stage) => stage.stage_id === stageId),

  addStage: async (missionId: number, vehicleName: VehicleEnum) =>
    await taurpc.mission.add_stage(missionId, vehicleName, "New Stage"),

  deleteStage: async (missionId: number, vehicleName: VehicleEnum, stageId: number) =>
    await taurpc.mission.delete_stage(missionId, vehicleName, stageId),

  transitionStage: async (missionId: number, vehicleName: VehicleEnum) => {
    return await taurpc.mission.transition_stage(missionId, vehicleName);
  },


}));


// =============================================
// Backend Event Listeners 
// ===============================================
// IMPORTANT: Never use missionZustandStore.setState() directly
// - use syncRustState to modify the state property
// - directly modifying the store will cause desync issues

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


// =============================================
// Reactive Vue Zustand Store
// ===============================================
// Convert store to reactive vue object to allow
// components to automatically rerender on property changes

// Syncs reactive vue store with zustand changes
// Reassign the entire state to ensure methods returns
// are synced with new zustand state
missionZustandStore.subscribe((newState) => {
  Object.assign(missionStore, newState);
  console.log("Zustand Store updated", missionStore);
});

// Make all properties readonly to avoid desync
export const missionStore: DeepReadonly<MissionStore> = reactive(missionZustandStore.getState());
