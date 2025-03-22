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
  mission_id: -1;
  isClient: true;
}

export interface ViewState {
  clientMission: ClientMission | null;
  tabState: {
    currentView: ViewType;
    currentMissionId: number | null;
    currentVehicleName: VehicleEnum | null;
    currentStageId: number | null;

    setCurrentView: (view: ViewType) => void;
    setCurrentMissionId: (missionId: number) => void;
    setCurrentVehicleName: (vehicleName: VehicleEnum) => void;
    setCurrentStageId: (stageId: number) => void;
  };
  getAllMissions: () => Array<ClientMission | MissionStruct>;
  addClientMission: () => void;
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
