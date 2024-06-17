import type { Coordinate, Stage } from ".";
import { MissionInformation } from "./mission-info";

export interface MissionInfoProvider {
  MISSION_INFO: MissionInformation;
  addStage: (stage: Stage) => void;
  updateSearchArea: (
    stageName: string,
    vehicleName: string,
    newSearchAreaCoords: Coordinate[]
  ) => void;
  updateTarget: (stageName: string, vehicleName: string, newTargetCoord: [string, string]) => void;
  getStageNames: () => string[];
  getStageInfo: (stageName: string) => void;
  checkStageExists: (stageName: string) => boolean;
  save_MISSION_INFO: () => void;
  load_MISSION_INFO: () => void;
}
