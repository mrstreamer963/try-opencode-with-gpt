export type Terrain = 'Land' | 'Water' | 'Grass';
export type BuildingKind = 'FoodBush' | 'Bed';
export type BuildCommandKind = 'food_bush' | 'bed';
export type CommandRejection = 'OutOfBounds' | 'NotBuildable' | 'Occupied' | 'UnknownBuildingKind';

export interface TileSnapshot {
  x: number;
  y: number;
  terrain: Terrain;
}

export interface MapSnapshot {
  width: number;
  height: number;
  tiles: TileSnapshot[];
}

export interface UnitSnapshot {
  id: number;
  x: number;
  y: number;
  food: number;
  sleep: number;
  alive: boolean;
}

export interface BuildingSnapshot {
  kind: BuildingKind;
  x: number;
  y: number;
}

export interface ConstructionJobSnapshot {
  id: number;
  kind: BuildingKind;
  x: number;
  y: number;
  progress: number;
  required: number;
}

export interface StateSnapshot {
  map: MapSnapshot;
  units: UnitSnapshot[];
  buildings: BuildingSnapshot[];
  construction_jobs: ConstructionJobSnapshot[];
}

export type CoreEvent =
  | { type: 'StateSnapshot'; snapshot: StateSnapshot }
  | { type: 'CommandRejected'; reason: CommandRejection }
  | { type: 'ConstructionStarted'; job_id: number };

export type WorkerCommand =
  | { type: 'Start' }
  | { type: 'Tick' }
  | { type: 'BuildAt'; kind: BuildCommandKind; x: number; y: number };

export interface SimulationCore {
  snapshot(): CoreEvent;
  tick(): CoreEvent;
  build_at(kind: BuildCommandKind, x: number, y: number): CoreEvent;
}
