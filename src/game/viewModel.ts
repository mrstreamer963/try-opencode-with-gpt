import type { BuildCommandKind, CommandRejection, WorkerCommand } from '../worker/protocol';

export interface TileCoordinate {
  x: number;
  y: number;
}

export function createBuildCommand(
  kind: BuildCommandKind,
  tile: TileCoordinate,
): WorkerCommand {
  return {
    type: 'BuildAt',
    kind,
    x: tile.x,
    y: tile.y,
  };
}

export function formatNeed(value: number): string {
  return `${Math.round(Math.min(100, Math.max(0, value)))}%`;
}

export function rejectionMessage(reason: CommandRejection): string {
  switch (reason) {
    case 'OutOfBounds':
      return 'Tile is outside the map.';
    case 'NotBuildable':
      return 'This terrain cannot be built on.';
    case 'Occupied':
      return 'This tile is already occupied.';
    case 'UnknownBuildingKind':
      return 'Unknown building type.';
  }
}

export function canReplaceHeldFeedback(now: number, holdUntil: number): boolean {
  return now >= holdUntil;
}
