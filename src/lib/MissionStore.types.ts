import {
  MissionsStruct,
  MissionStruct,
  StageStruct,
  VehicleEnum,
  VehicleStruct
} from "@/lib/bindings";

export type ViewType = "mission" | "vehicle" | "stage" | "zone";
export interface ViewState {
  currentView: ViewType;
  currentMissionId: number | null;
  currentVehicleName: VehicleEnum | null;
  currentStageId: number | null;
}

export interface MissionStore {
  state: MissionsStruct;
  syncRustState: (state: MissionsStruct) => void;
  view: ViewState;

  // Frontend State
  setCurrentView: (view: ViewType) => void;
  setCurrentMissionID: (missionId: number | null) => void;
  setCurrentVehicleName: (vehicleName: VehicleEnum | null) => void;
  setCurrentStageID: (stageId: number | null) => void;

  // Mission Data
  getAllMissions: () => MissionStruct[];

  getMissionData: (missionId: number) => MissionStruct | undefined;
  setMissionData: (missionData: MissionStruct) => Promise<null>;
  createNewMission: (missionName: string) => Promise<null>;

  // Vehicle Data
  getVehicleData: (missionId: number, vehicleName: VehicleEnum) => VehicleStruct | undefined;
  setAutoMode: (missionId: number, vehicleName: VehicleEnum, isAuto: boolean) => Promise<null>;

  // Stage Data
  addStage: (missionId: number, vehicleName: VehicleEnum, stageName: string) => Promise<null>;
  deleteStage: (missionId: number, vehicleName: VehicleEnum, stageId: number) => Promise<null>;
  getStageData: (
    missionId: number,
    vehicleName: VehicleEnum,
    stageId: number
  ) => StageStruct | undefined;
  transitionStage: (missionId: number, vehicleName: VehicleEnum) => Promise<null>;
}
