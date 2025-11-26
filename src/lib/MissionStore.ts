import {
  createTauRPCProxy,
  GeoCoordinateStruct,
  MissionsStruct,
  VehicleEnum,
  ZoneType
} from "@/lib/bindings";
import { DeepReadonly, reactive, ref, computed } from "vue";
import { MissionStore, ViewState, ViewType } from "@/lib/MissionStore.types";
import { LatLng } from "leaflet";
import { defineStore } from "pinia";

// =============================================
// Initialization
// ===============================================
const taurpc = createTauRPCProxy();

// Fetch initial state from backend
const initialState: MissionsStruct = await taurpc.mission.get_all_missions();

// =======================================================
//  Pinia Store
//  NOTE: All getters will returns computeds EXCEPT:  
//  getStageData(), getVehicleData() , and getZoneData().
// =======================================================

export const missionPiniaStore = defineStore("mission", () => {
  // --------------------------
  // Backend State
  // --------------------------
  const missionState = ref<MissionsStruct | null>(initialState);
  const syncRustState = (rustState: MissionsStruct) => {
    missionState.value = rustState;
  };

  // --------------------------
  // Frontend View State
  // --------------------------
  const viewState = ref<ViewState>({
    currentView: "mission",
    currentMissionId: null,
    currentVehicleName: null,
    currentStageId: null
  });
  const getViewState = () => {
    return computed(()=>viewState.value);
  };
  const getCurrentView = () => {
    return computed(()=>viewState.value.currentView);
  };
  const getCurrentMissionId = () => {
    return computed(()=>viewState.value.currentMissionId);
  };
  const getCurrentVehicleName = () => {
    return computed(()=>viewState.value.currentVehicleName);
  };
  const getCurrentStageId = () => {
    return computed(()=>viewState.value.currentStageId);
  };

  const setCurrentView = (view: ViewType) => {
    viewState.value.currentView = view;
  };
  const setCurrentMissionID = (missionId: number | null) => {
    viewState.value.currentMissionId = missionId;
  };
  const setCurrentVehicleName = (vehicleName: VehicleEnum | null) => {
    if (viewState.value.currentMissionId == undefined || viewState.value.currentMissionId === null)
      throw new Error("No mission selected");
    viewState.value.currentVehicleName = vehicleName;
  };
  const setCurrentStageID = (stageId: number | null) => {
    if (viewState.value.currentMissionId === null) {
      throw new Error("No mission selected");
    }
    if (viewState.value.currentVehicleName === null) {
      throw new Error("No vehicle selected");
    }
    viewState.value.currentStageId = stageId;
  };

  // --------------------------
  // Mission Data
  // --------------------------
  const getAllMissions = () => {
    return computed(() => missionState.value);
  };
  const getMissionData = (missionId: number) => {
    return computed(() =>
      missionState.value?.missions.find((mission) => mission.mission_id === missionId)
    );
  };
  const renameMission = async (missionId: number, missionName: string) => {
    return await taurpc.mission.rename_mission(missionId, missionName);
  };

  const createNewMission = async (missionName: string) => {
    return await taurpc.mission.create_mission(missionName);
  };

  const deleteMission = async (missionId: number) => {
    return await taurpc.mission.delete_mission(missionId);
  };
  const startMission = async (missionId: number) => {
    return await taurpc.mission.start_mission(missionId);
  };

  // --------------------------
  // Vehicle Data
  // --------------------------
  const getVehicleData = (missionId: number, vehicleName: VehicleEnum) => {
    return missionState.value?.missions.find((m) => m.mission_id === missionId)?.vehicles[
      vehicleName
    ];
  };
  const setAutoMode = async (missionId: number, vehicleName: VehicleEnum, isAuto: boolean) => {
    return await taurpc.mission.set_auto_mode(missionId, vehicleName, isAuto);
  };

  // --------------------------
  // Stage Data
  // --------------------------
  const getStageData = (missionId: number, vehicleName: VehicleEnum, stageId: number) => {
    return missionState.value?.missions
      .find((mission) => mission.mission_id === missionId)
      ?.vehicles[vehicleName].stages.find((s) => s.stage_id === stageId);
  };
  const addStage = async (missionId: number, vehicleName: VehicleEnum) => {
    return await taurpc.mission.add_stage(missionId, vehicleName, "New Stage");
  };
  const deleteStage = async (missionId: number, vehicleName: VehicleEnum, stageId: number) => {
    return await taurpc.mission.delete_stage(missionId, vehicleName, stageId);
  };
  const renameStage = async (
    missionId: number,
    vehicleName: VehicleEnum,
    stageId: number,
    stageName: string
  ) => {
    return await taurpc.mission.rename_stage(missionId, vehicleName, stageId, stageName);
  };
  const transitionStage = async (missionId: number, vehicleName: VehicleEnum) => {
    return await taurpc.mission.transition_stage(missionId, vehicleName);
  };
  const updateStageArea = async (
    missionId: number,
    vehicleName: VehicleEnum,
    stageId: number,
    area: GeoCoordinateStruct[]
  ) => {
    return await taurpc.mission.update_stage_area(missionId, vehicleName, stageId, area);
  };

  // --------------------------
  // Zone Data
  // --------------------------
  const getZoneData = (missionId: number, zoneType: ZoneType) => {
    const zoneMap: Record<ZoneType, "keep_in_zones" | "keep_out_zones"> = {
      KeepIn: "keep_in_zones",
      KeepOut: "keep_out_zones"
    };
    return missionState.value?.missions.find((mission) => mission.mission_id === missionId)?.zones[
      zoneMap[zoneType]
    ];
  };
  const updateZone = async (
    missionId: number,
    zoneType: ZoneType,
    zoneIndex: number,
    zoneCoords: GeoCoordinateStruct[]
  ) => {
    return await taurpc.mission.update_zone(missionId, zoneType, zoneIndex, zoneCoords);
  };
  const addZone = async (missionId: number, zoneType: ZoneType) => {
    return await taurpc.mission.add_zone(missionId, zoneType);
  };
  const deleteZone = async (missionId: number, zoneType: ZoneType, zoneIndex: number) => {
    return await taurpc.mission.delete_zone(missionId, zoneType, zoneIndex);
  };

  return {
    missionState,
    viewState,
    getAllMissions,
    syncRustState,
    getViewState,
    getCurrentView,
    getCurrentMissionId,
    getCurrentVehicleName,
    getCurrentStageId,
    setCurrentView,
    setCurrentMissionID,
    setCurrentVehicleName,
    setCurrentStageID,
    getMissionData,
    renameMission,
    createNewMission,
    deleteMission,
    startMission,
    getVehicleData,
    setAutoMode,
    getStageData,
    addStage,
    deleteStage,
    renameStage,
    transitionStage,
    updateStageArea,
    getZoneData,
    updateZone,
    addZone,
    deleteZone
  };
});
