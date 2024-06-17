import { Ref, ref } from "vue";

export interface TargetCoordsProvider {
  targetCoord: Ref<string>;
  selectingTarget: Ref<boolean>;
}

export const defaultTargetCoords: TargetCoordsProvider = {
  targetCoord: ref(""),
  selectingTarget: ref(false)
};
