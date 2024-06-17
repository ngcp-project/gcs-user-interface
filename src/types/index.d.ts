// these are types that make it easier for us to update target coordinate and search area coordinates
// for each obj

export interface Coordinate {
  latitude: string | null;
  longitude: string | null;
}

export interface Vehicle {
  vehicleName: string;
  target: Coordinate | null;
  searchArea: Coordinate[] | null;
}

export interface Stage {
  stageName: string;
  vehicleKeys: Vehicle[];
}
