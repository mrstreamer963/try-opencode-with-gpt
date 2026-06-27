import { Application, Container, Graphics } from 'pixi.js';
import type { StateSnapshot, Terrain } from '../worker/protocol';

const TILE_SIZE = 18;

export interface PixiMapRenderer {
  render(snapshot: StateSnapshot): void;
  destroy(): void;
}

export async function createPixiMapRenderer(
  host: HTMLElement,
  onTileSelected: (x: number, y: number) => void,
): Promise<PixiMapRenderer> {
  const app = new Application();
  await app.init({
    width: 30 * TILE_SIZE,
    height: 30 * TILE_SIZE,
    background: '#111827',
    antialias: false,
  });

  host.replaceChildren(app.canvas);
  app.canvas.classList.add('game-canvas');

  const layer = new Container();
  app.stage.addChild(layer);

  app.stage.eventMode = 'static';
  app.stage.hitArea = app.screen;
  app.stage.on('pointertap', (event) => {
    const point = event.global;
    onTileSelected(Math.floor(point.x / TILE_SIZE), Math.floor(point.y / TILE_SIZE));
  });

  return {
    render(snapshot) {
      layer.removeChildren();
      renderTerrain(layer, snapshot);
      renderConstruction(layer, snapshot);
      renderBuildings(layer, snapshot);
      renderUnits(layer, snapshot);
    },
    destroy() {
      app.destroy(true, { children: true });
    },
  };
}

function renderTerrain(layer: Container, snapshot: StateSnapshot) {
  for (const tile of snapshot.map.tiles) {
    const graphic = new Graphics();
    graphic
      .rect(tile.x * TILE_SIZE, tile.y * TILE_SIZE, TILE_SIZE - 1, TILE_SIZE - 1)
      .fill(terrainColor(tile.terrain));
    layer.addChild(graphic);
  }
}

function renderConstruction(layer: Container, snapshot: StateSnapshot) {
  for (const job of snapshot.construction_jobs) {
    const progress = Math.min(1, job.progress / job.required);
    const graphic = new Graphics();
    graphic
      .rect(job.x * TILE_SIZE + 3, job.y * TILE_SIZE + 3, (TILE_SIZE - 6) * progress, 4)
      .fill(0xfacc15);
    graphic.rect(job.x * TILE_SIZE + 4, job.y * TILE_SIZE + 8, TILE_SIZE - 8, TILE_SIZE - 8).stroke({
      color: 0xfacc15,
      width: 1,
    });
    layer.addChild(graphic);
  }
}

function renderBuildings(layer: Container, snapshot: StateSnapshot) {
  for (const building of snapshot.buildings) {
    const graphic = new Graphics();
    if (building.kind === 'FoodBush') {
      graphic.circle(building.x * TILE_SIZE + 9, building.y * TILE_SIZE + 9, 6).fill(0x22c55e);
      graphic.circle(building.x * TILE_SIZE + 11, building.y * TILE_SIZE + 7, 2).fill(0xef4444);
    } else {
      graphic.roundRect(building.x * TILE_SIZE + 3, building.y * TILE_SIZE + 5, 12, 9, 2).fill(0x8b5cf6);
      graphic.rect(building.x * TILE_SIZE + 4, building.y * TILE_SIZE + 6, 10, 3).fill(0xc4b5fd);
    }
    layer.addChild(graphic);
  }
}

function renderUnits(layer: Container, snapshot: StateSnapshot) {
  for (const unit of snapshot.units) {
    const graphic = new Graphics();
    graphic.circle(unit.x * TILE_SIZE + 9, unit.y * TILE_SIZE + 9, 5).fill(0xf8fafc);
    graphic.circle(unit.x * TILE_SIZE + 7, unit.y * TILE_SIZE + 7, 1).fill(0x111827);
    graphic.circle(unit.x * TILE_SIZE + 11, unit.y * TILE_SIZE + 7, 1).fill(0x111827);
    layer.addChild(graphic);
  }
}

function terrainColor(terrain: Terrain): number {
  switch (terrain) {
    case 'Land':
      return 0x8b6f47;
    case 'Water':
      return 0x2563eb;
    case 'Grass':
      return 0x3f7d3b;
  }
}
