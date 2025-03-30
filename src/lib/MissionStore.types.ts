import { MissionsStruct, MissionStruct, StageStruct, VehicleEnum, VehicleStruct } from "@/lib/bindings";

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

  setCurrentView: (view: ViewType) => void;
  setCurrentMissionID: (missionId: number | null) => void;
  setCurrentVehicleName: (vehicleName: VehicleEnum | null) => void;
  setCurrentStageID: (stageId: number | null) => void;

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


  addStage: (
    missionId: number,
    vehicleName: VehicleEnum,
    stageName: string
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
