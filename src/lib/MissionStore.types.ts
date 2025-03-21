import {
  MissionStruct,
  VehicleEnum,
  MissionsStruct,
  VehicleStruct,
  StageStruct,
  ZonesStruct
} from "@/lib/bindings";

export type DeepReadonly<T> = {
  readonly [P in keyof T]: DeepReadonly<T[P]>;
};

export type ViewType = "mission" | "vehicle" | "stage" | "zone";

export interface ViewState {
  currentView: ViewType;
  tabState: {
    currentMissionId: number | null;
    currentVehicleName: VehicleEnum | null;
    currentStageId: number | null;
  };
  clientMission: MissionStruct | null;
  setCurrentMissionId: (missionId: number) => void;
  setCurrentVehicleName: (vehicleName: VehicleEnum) => void;
  setCurrentStageId: (stageId: number) => void;
  getAllMissions: () => MissionStruct[];
  setCurrentView: (view: ViewType) => void;
  addMission: () => void;
  deleteClientMission: () => void;
}

export interface MissionStore {
  state: MissionsStruct;
  view: ViewState;

  getMissionData: (mission_id: number) => MissionStruct | undefined;
  getVehicleData: (mission_id: number, vehicle_name: VehicleEnum) => VehicleStruct | undefined;
  getStageData: (
    mission_id: number,
    vehicle_name: VehicleEnum,
    stage_id: number
  ) => StageStruct | undefined;
  getZoneData: (mission_id: number) => ZonesStruct | undefined;
  nextStage: (missionId: number, vehicleName: VehicleEnum) => void;
  submitMission: (missionId: number) => void;
}
