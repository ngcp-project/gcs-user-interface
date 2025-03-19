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
import { ClientMission, MissionStore, ViewType } from "@/lib/MissionStore.types";

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
    clientMissions: [],
    setCurrentView: (view: ViewType) =>
      set((state) => ({ view: { ...state.view, currentView: view } })),
    setCurrentMissionId: (missionId: number) =>
      set((state) => ({
        view: { ...state.view, tabState: { ...state.view.tabState, currentMissionId: missionId } }
      })),
    setCurrentVehicleName: (vehicleName: VehicleEnum) =>
      set((state) => ({
        view: {
          ...state.view,
          tabState: { ...state.view.tabState, currentVehicleName: vehicleName }
        }
      })),
    setCurrentStageId: (stageId: number) =>
      set((state) => ({
        view: { ...state.view, tabState: { ...state.view.tabState, currentStageId: stageId } }
      })),
    getAllMissions: () => {
      // convert missions object to array
      // get all keys and map them to mission data
      // filter out missions that have undefined data
      return get().view.clientMissions;
    },
    addMission: () => {
      console.log("added mission");
      const newMission: ClientMission = {
        mission_name: "Mission",
        mission_id: Math.random(),
        mission_status: "Inactive",
        isSubmitted: false,
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
          clientMissions: [...state.view.clientMissions, newMission]
        }
      }));
      console.log(get().view.clientMissions);
    },
    deleteMission: (missionIndex: number) => {
      const missionData = get().getMissionData(missionIndex);
      if (missionData && "isSubmitted" in missionData && missionData.isSubmitted === true) {
        console.log("Delete mission on backend");
        // return taurpc.mission.delete_mission(missionIndex);
      }
      const filteredMissions = get()
        .view.getAllMissions()
        .filter((mission) => mission.mission_id !== missionIndex);

      set((state) => ({
        view: { ...state.view, clientMissions: filteredMissions }
      }));
    }
  },
  // Get Methods
  getMissionData: (mission_id: number) => {
    // check if clientmission is submitted
    // If it is submitted return what the backend state is
    // find object in the clientMissions array based on the mission_id property
    const mission = get().view.clientMissions.find((mission) => mission.mission_id === mission_id);
    if (mission?.isSubmitted) {
      return get().state.missions.find((mission) => mission.mission_id === mission_id);
    }
    return mission;
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
  submitMission: async (clientMissionId: number) => {
    console.log("Submitting mission", clientMissionId);
    const missionData = get().getMissionData(clientMissionId);
    if (missionData === undefined) {
      throw new Error("Mission does not exist");
    }
    if ("isSubmitted" in missionData && missionData.isSubmitted === true) {
      throw new Error("Mission is already submitted");
    }

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

  // Sync clientMissions with backend missions
  // note that this will overwrite any existing clientMissions not submitted
  missionZustandStore.setState({ view: { ...missionStore.view, clientMissions: data.missions } });
  // console.log("asda", missionStore.view.clientMissions?.[0]?.mission_name);
});

// On mission data update from backend, update the store
taurpc.mission.on_updated.on((data: MissionsStruct) => {
  console.log("Mission data updated:", data);
  missionZustandStore.setState({ state: data });
  // Receive any updates from rust state and merge them with the clientMissions
  missionZustandStore.setState({
    view: {
      ...missionStore.view,
      clientMissions: { ...data.missions, ...missionZustandStore.getState().view.clientMissions }
    }
  });
});

// convert zustandStore to a reactive object that triggers rerenders
// to avoid desync make reactive state readonly
export const missionStore: DeepReadonly<MissionStore> = reactive(missionZustandStore.getState());
