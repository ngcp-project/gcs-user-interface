import { Ref, ref } from "vue";

export interface SearchCoordsProvider {
  searchCoords: Ref<string[]>;
  selectingSearch: Ref<boolean>;
  updateSearchCoords: (coordinate: string[]) => void;
}

export const defaultSearchCoords: SearchCoordsProvider = {
  searchCoords: ref([]),
  selectingSearch: ref(false),
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  updateSearchCoords: function (coordinate: string[]): void {
    throw new Error("Function not implemented.");
  }
};
