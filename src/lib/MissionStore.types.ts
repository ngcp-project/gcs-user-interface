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

export interface ClientMission extends MissionStruct {
  isSubmitted?: boolean;
}
export interface ViewState {
  currentView: ViewType;
  tabState: {
    currentMissionId: number | null;
    currentVehicleName: VehicleEnum | null;
    currentStageId: number | null;
  };
  clientMissions: ClientMission[];
  setCurrentMissionId: (missionId: number) => void;
  setCurrentVehicleName: (vehicleName: VehicleEnum) => void;
  setCurrentStageId: (stageId: number) => void;
  getAllMissions: () => ClientMission[];
  setCurrentView: (view: ViewType) => void;
  addMission: () => void;
  deleteMission: (missionIndex: number) => void;
}

export interface MissionStore {
  state: MissionsStruct;
  view: ViewState;

  getMissionData: (mission_id: number) => MissionStruct | ClientMission | undefined;
  getVehicleData: (mission_id: number, vehicle_name: VehicleEnum) => VehicleStruct | undefined;
  getStageData: (
    mission_id: number,
    vehicle_name: VehicleEnum,
    stage_id: number
  ) => StageStruct | undefined;
  getZoneData: (mission_id: number) => ZonesStruct | undefined;
  submitMission: (missionId: number) => void;
}
