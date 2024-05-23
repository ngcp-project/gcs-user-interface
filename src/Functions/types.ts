// these are types that make it easier for us to update target coordinate and search area coordinates
// for each obj

export interface Coordinate {
    latitude: string;
    longitude: string;
  }
  
export interface Vehicle {
vehicleName: string;
target: Coordinate;
searchArea: Coordinate[];
}

export interface Stage {
stageName: string;
vehicleKeys: Vehicle[];
}
