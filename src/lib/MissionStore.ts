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
import { DeepReadonly, reactive } from "vue";
import { MissionStore, ViewType } from "@/lib/MissionStore.types";

const taurpc = createTauRPCProxy();

// Fetch initial state from backend
const initialState: MissionsStruct = await taurpc.mission.get_all_missions();

const missionZustandStore = createStore<MissionStore>((set, get) => ({
  // Since we use missionsStruct as basis of store, we need to populate it
  // the backend state of the values
  state: initialState,

  //  Frontend State
  view: {
    currentView: "mission",
    tabState: {
      currentMissionId: null,
      currentVehicleName: null,
      currentStageId: null
    },
    clientMission: null,
    setCurrentView: (view: ViewType) =>
      set((state) => ({ view: { ...state.view, currentView: view } })),

    setCurrentMissionId: (missionId: number) =>
      set((state) => ({
        view: { ...state.view, tabState: { ...state.view.tabState, currentMissionId: missionId } }
      })),

    setCurrentVehicleName: (vehicleName: VehicleEnum) => {
      if (get().view.tabState.currentMissionId === null) throw new Error("No mission selected");

      set((state) => ({
        view: {
          ...state.view,
          tabState: { ...state.view.tabState, currentVehicleName: vehicleName }
        }
      }));
    },

    setCurrentStageId: (stageId: number) => {
      if (get().view.tabState.currentMissionId === null) {
        throw new Error("No mission selected");
      }
      if (get().view.tabState.currentVehicleName === null) {
        throw new Error("No vehicle selected");
      }
      set((state) => ({
        view: { ...state.view, tabState: { ...state.view.tabState, currentStageId: stageId } }
      }));
    },

    getAllMissions: () => {
      // return backend missions with clientMission at end
      const clientMission = get().view.clientMission;
      // if there is no clientMission
      if (clientMission === null) {
        return get().state.missions;
      }
      return [...get().state.missions, clientMission];
    },

    addMission: () => {
      console.log("added mission");
      const newMission: MissionStruct = {
        mission_name: "Mission",
        mission_id: Math.random(),
        mission_status: "Inactive",
        zones: {
          keep_in_zones: [],
          keep_out_zones: []
        },
        vehicles: {
          ERU: {
            vehicle_name: "ERU",
            current_stage: 0,
            patient_status: null,
            stages: []
          },
          MEA: {
            vehicle_name: "MEA",
            current_stage: 0,
            patient_status: null,
            stages: []
          },
          MRA: {
            vehicle_name: "MRA",
            current_stage: 0,
            patient_status: null,
            stages: []
          }
        }
      };
      set((state) => ({
        // TODO: id is random int, add some logic to add noncolluding ids
        view: {
          ...state.view,
          clientMission: newMission
        }
      }));
      console.log(missionStore.view);
    },

    deleteClientMission: () => {
      set((state) => ({
        view: {
          ...state.view,
          clientMission: null
        }
      }));
    }
  },

  // Get Methods
  getMissionData: (mission_id: number) => {
    const missions = get().view.getAllMissions();
    return missions.find((mission) => mission.mission_id === mission_id);
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
  nextStage: (missionId: number, vehicleName: VehicleEnum) => {
    const vehicle = get().getVehicleData(missionId, vehicleName);
    if (!vehicle) throw new Error("Vehicle does not exist");

    const currentStage = vehicle.current_stage;
    if (currentStage === vehicle.stages.length - 1) {
      throw new Error("Vehicle is already at last stage");
    }

    // TODO: add taurpc command
    console.log(`added next stage in ${vehicleName} for mission id: ${missionId}`);
  },
  // Submits a new mission
  submitMission: async (clientMissionId: number) => {
    console.log("Submitting mission", clientMissionId);
    const missionData = get().getMissionData(clientMissionId);
    if (!missionData) throw new Error("Mission does not exist");

    // clear clientMission
    set((state) => ({
      view: {
        ...state.view,
        clientMission: null
      }
    }));

    await taurpc.mission.submit_mission(missionData);
  }
}));

// listen to zustandStore changes and update the reactive object
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

// convert zustandStore to a reactive object that triggers rerenders
// to avoid desync make reactive state readonly
export const missionStore: DeepReadonly<MissionStore> = reactive(missionZustandStore.getState());
