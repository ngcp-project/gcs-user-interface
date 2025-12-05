import {
  VehicleEnum,
} from "@/lib/bindings";

export type ViewType = "mission" | "vehicle" | "stage" | "zone";
export interface ViewState {
  currentView: ViewType;
  currentMissionId: number | null;
  currentVehicleName: VehicleEnum | null;
  currentStageId: number | null;
}

