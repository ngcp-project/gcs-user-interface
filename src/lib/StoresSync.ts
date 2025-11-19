import { missionPiniaStore } from "./MissionStore";
import { createTauRPCProxy, MissionsStruct } from "./bindings";

//Declare store variables: 
let missionStore: ReturnType<typeof missionPiniaStore>;

//Establish taurpc connections.
export const establishTaurpcConnection = () => {
  //Assign stores to their respective useStores():
  missionStore = missionPiniaStore();


// =============================================
// Backend Event Listeners
// ===============================================
  const taurpc = createTauRPCProxy();
  taurpc.mission.get_all_missions().then((data) => {
    console.log("PINIA: Mission data fetched:", data);
    missionStore!.syncRustState(data);
  });

  taurpc.mission.on_updated.on((data: MissionsStruct) => {
    console.log("PINIA: Mission data updated:", data);
    missionStore!.syncRustState(data);
  });
};

export { missionStore };

