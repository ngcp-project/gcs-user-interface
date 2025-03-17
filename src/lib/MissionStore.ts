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
  isSubmitted?: boolean;
}
interface ViewState {
  currentView: ViewType;
  clientMissions: Partial<{ [key in number]: ClientMission }>;
  getAllMissions: () => ClientMission[];
  setCurrentView: (view: ViewType) => void;
  addMission: () => void;
  deleteMission: (missionIndex: number) => void;
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
    clientMissions: {},
    setCurrentView: (view: ViewType) =>
      set((state) => ({ view: { ...state.view, currentView: view } })),
    getAllMissions: () => {
      // convert missions object to array
      // get all keys and map them to mission data
      // filter out missions that have undefined data
      const missionArr = Object.keys(get().view.clientMissions)
        .map((key) => get().getMissionData(Number(key)))
        .filter((mission) => mission !== undefined) as ClientMission[];

      return missionArr;
    },
    addMission: () => {
      console.log("add mission");
      const newMission: ClientMission = {
        mission_name: "Mission 1",
        mission_status: "Active",
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
            stages: {}
          },
          MEA: {
            vehicle_name: "MEA",
            current_stage: 0,
            patient_status: null,
            stages: {}
          },
          MRA: {
            vehicle_name: "MRA",
            current_stage: 0,
            patient_status: null,
            stages: {}
          }
        }
      };
      set((state) => ({
        // TODO: id is random int, add some logic to add noncolluding ids
        view: {
          ...state.view,
          clientMissions: { ...state.view.clientMissions, [Math.random()]: newMission }
        }
      }));
    },
    deleteMission: (missionIndex: number) => {
      const missionData = get().getMissionData(missionIndex);
      if (missionData && "isSubmitted" in missionData && missionData.isSubmitted === true) {
        console.log("Delete mission on backend");
        // return taurpc.mission.delete_mission(missionIndex);
      }
      const filteredMissions = Object.fromEntries(
        Object.entries(get().view.getAllMissions()).filter(([key]) => Number(key) !== missionIndex)
      );
      set((state) => ({
        view: { ...state.view, clientMissions: filteredMissions }
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

// convert zustandStore to a reactive object that triggers rerenders
export const missionStore = reactive(missionZustandStore.getState());

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
  console.log("asda", missionStore.view.clientMissions?.[0]?.mission_name);
});

// On mission data update from backend, update the store
taurpc.mission.on_updated.on((data: MissionsStruct) => {
  console.log("Mission data updated:", data);
  missionZustandStore.setState({ state: data });
});
