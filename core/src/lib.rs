use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub const MAP_WIDTH: usize = 30;
pub const MAP_HEIGHT: usize = 30;

const CONSTRUCTION_WORK_REQUIRED: f32 = 3.0;
const FOOD_DECAY_PER_TICK: f32 = 0.4;
const SLEEP_DECAY_PER_TICK: f32 = 0.3;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Terrain {
    Land,
    Water,
    Grass,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BuildingKind {
    FoodBush,
    Bed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CommandRejection {
    OutOfBounds,
    NotBuildable,
    Occupied,
    UnknownBuildingKind,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TileSnapshot {
    pub x: usize,
    pub y: usize,
    pub terrain: Terrain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapSnapshot {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileSnapshot>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnitSnapshot {
    pub id: u32,
    pub x: usize,
    pub y: usize,
    pub food: f32,
    pub sleep: f32,
    pub alive: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildingSnapshot {
    pub kind: BuildingKind,
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionJobSnapshot {
    pub id: u32,
    pub kind: BuildingKind,
    pub x: usize,
    pub y: usize,
    pub progress: f32,
    pub required: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StateSnapshot {
    pub map: MapSnapshot,
    pub units: Vec<UnitSnapshot>,
    pub buildings: Vec<BuildingSnapshot>,
    pub construction_jobs: Vec<ConstructionJobSnapshot>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildCommand {
    pub kind: BuildingKind,
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum CoreEvent {
    StateSnapshot { snapshot: StateSnapshot },
    CommandRejected { reason: CommandRejection },
    ConstructionStarted { job_id: u32 },
}

#[derive(Component)]
struct Unit {
    id: u32,
    alive: bool,
}

#[derive(Component)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct Needs {
    food: f32,
    sleep: f32,
}

#[derive(Component)]
struct Building {
    kind: BuildingKind,
}

#[derive(Component)]
struct ConstructionJob {
    id: u32,
    kind: BuildingKind,
    progress: f32,
}

#[derive(Resource)]
struct MapResource {
    tiles: Vec<Terrain>,
}

pub struct Simulation {
    world: World,
    next_job_id: u32,
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulation {
    pub fn new() -> Self {
        let mut world = World::new();
        world.insert_resource(MapResource {
            tiles: starter_tiles(),
        });

        let starts = [(2, 2), (3, 2), (2, 3)];
        for (index, (x, y)) in starts.into_iter().enumerate() {
            world.spawn((
                Unit {
                    id: index as u32 + 1,
                    alive: true,
                },
                Position { x, y },
                Needs {
                    food: 100.0,
                    sleep: 100.0,
                },
            ));
        }

        Self {
            world,
            next_job_id: 1,
        }
    }

    pub fn build_at(
        &mut self,
        kind: BuildingKind,
        x: usize,
        y: usize,
    ) -> Result<u32, CommandRejection> {
        self.validate_build_target(x, y)?;

        let job_id = self.next_job_id;
        self.next_job_id += 1;
        self.world.spawn((
            ConstructionJob {
                id: job_id,
                kind,
                progress: 0.0,
            },
            Position { x, y },
        ));

        Ok(job_id)
    }

    pub fn tick(&mut self) {
        self.advance_construction();
        self.decay_needs();
        self.restore_needs_from_buildings();
    }

    pub fn snapshot(&mut self) -> StateSnapshot {
        let map = self.map_snapshot();
        let mut units = self.unit_snapshots();
        let mut buildings = self.building_snapshots();
        let mut construction_jobs = self.construction_job_snapshots();

        units.sort_by_key(|unit| unit.id);
        buildings.sort_by_key(|building| (building.y, building.x));
        construction_jobs.sort_by_key(|job| job.id);

        StateSnapshot {
            map,
            units,
            buildings,
            construction_jobs,
        }
    }

    fn validate_build_target(&mut self, x: usize, y: usize) -> Result<(), CommandRejection> {
        if x >= MAP_WIDTH || y >= MAP_HEIGHT {
            return Err(CommandRejection::OutOfBounds);
        }

        if self.terrain_at(x, y) == Terrain::Water {
            return Err(CommandRejection::NotBuildable);
        }

        if self.has_building_at(x, y) || self.has_construction_job_at(x, y) {
            return Err(CommandRejection::Occupied);
        }

        Ok(())
    }

    fn terrain_at(&self, x: usize, y: usize) -> Terrain {
        self.world.resource::<MapResource>().tiles[tile_index(x, y)]
    }

    fn has_building_at(&mut self, x: usize, y: usize) -> bool {
        self.world
            .query_filtered::<&Position, With<Building>>()
            .iter(&self.world)
            .any(|position| position.x == x && position.y == y)
    }

    fn has_construction_job_at(&mut self, x: usize, y: usize) -> bool {
        self.world
            .query_filtered::<&Position, With<ConstructionJob>>()
            .iter(&self.world)
            .any(|position| position.x == x && position.y == y)
    }

    fn advance_construction(&mut self) {
        let mut completed = Vec::new();

        for (entity, mut job, position) in self
            .world
            .query::<(Entity, &mut ConstructionJob, &Position)>()
            .iter_mut(&mut self.world)
        {
            job.progress += 1.0;
            if job.progress >= CONSTRUCTION_WORK_REQUIRED {
                completed.push((entity, job.kind, position.x, position.y));
            }
        }

        for (entity, kind, x, y) in completed {
            self.world.entity_mut(entity).despawn();
            self.world.spawn((Building { kind }, Position { x, y }));
        }
    }

    fn decay_needs(&mut self) {
        for mut needs in self.world.query::<&mut Needs>().iter_mut(&mut self.world) {
            needs.food = (needs.food - FOOD_DECAY_PER_TICK).max(0.0);
            needs.sleep = (needs.sleep - SLEEP_DECAY_PER_TICK).max(0.0);
        }
    }

    fn restore_needs_from_buildings(&mut self) {
        let has_food_bush = self.has_building_kind(BuildingKind::FoodBush);
        let has_bed = self.has_building_kind(BuildingKind::Bed);

        for mut needs in self.world.query::<&mut Needs>().iter_mut(&mut self.world) {
            if has_food_bush && needs.food <= 40.0 {
                needs.food = (needs.food + 25.0).min(100.0);
            }
            if has_bed && needs.sleep <= 35.0 {
                needs.sleep = (needs.sleep + 30.0).min(100.0);
            }
        }
    }

    fn has_building_kind(&mut self, kind: BuildingKind) -> bool {
        self.world
            .query::<&Building>()
            .iter(&self.world)
            .any(|building| building.kind == kind)
    }

    fn map_snapshot(&self) -> MapSnapshot {
        let tiles = self
            .world
            .resource::<MapResource>()
            .tiles
            .iter()
            .enumerate()
            .map(|(index, terrain)| TileSnapshot {
                x: index % MAP_WIDTH,
                y: index / MAP_WIDTH,
                terrain: *terrain,
            })
            .collect();

        MapSnapshot {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            tiles,
        }
    }

    fn unit_snapshots(&mut self) -> Vec<UnitSnapshot> {
        self.world
            .query::<(&Unit, &Position, &Needs)>()
            .iter(&self.world)
            .map(|(unit, position, needs)| UnitSnapshot {
                id: unit.id,
                x: position.x,
                y: position.y,
                food: needs.food,
                sleep: needs.sleep,
                alive: unit.alive,
            })
            .collect()
    }

    fn building_snapshots(&mut self) -> Vec<BuildingSnapshot> {
        self.world
            .query::<(&Building, &Position)>()
            .iter(&self.world)
            .map(|(building, position)| BuildingSnapshot {
                kind: building.kind,
                x: position.x,
                y: position.y,
            })
            .collect()
    }

    fn construction_job_snapshots(&mut self) -> Vec<ConstructionJobSnapshot> {
        self.world
            .query::<(&ConstructionJob, &Position)>()
            .iter(&self.world)
            .map(|(job, position)| ConstructionJobSnapshot {
                id: job.id,
                kind: job.kind,
                x: position.x,
                y: position.y,
                progress: job.progress,
                required: CONSTRUCTION_WORK_REQUIRED,
            })
            .collect()
    }
}

#[wasm_bindgen]
pub struct WasmSimulation {
    simulation: Simulation,
}

#[wasm_bindgen]
impl WasmSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            simulation: Simulation::new(),
        }
    }

    pub fn snapshot(&mut self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&CoreEvent::StateSnapshot {
            snapshot: self.simulation.snapshot(),
        })
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    pub fn tick(&mut self) -> Result<JsValue, JsValue> {
        self.simulation.tick();
        self.snapshot()
    }

    pub fn build_at(&mut self, kind: &str, x: usize, y: usize) -> Result<JsValue, JsValue> {
        let kind = parse_building_kind(kind).ok_or(CommandRejection::UnknownBuildingKind);
        let event = match kind.and_then(|kind| self.simulation.build_at(kind, x, y)) {
            Ok(job_id) => CoreEvent::ConstructionStarted { job_id },
            Err(reason) => CoreEvent::CommandRejected { reason },
        };

        serde_wasm_bindgen::to_value(&event).map_err(|error| JsValue::from_str(&error.to_string()))
    }
}

impl Default for WasmSimulation {
    fn default() -> Self {
        Self::new()
    }
}

fn starter_tiles() -> Vec<Terrain> {
    let mut tiles = Vec::with_capacity(MAP_WIDTH * MAP_HEIGHT);
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let terrain = if x == 0 || y == 0 || x == MAP_WIDTH - 1 || y == MAP_HEIGHT - 1 {
                Terrain::Water
            } else if (x + y) % 3 == 0 {
                Terrain::Grass
            } else {
                Terrain::Land
            };
            tiles.push(terrain);
        }
    }
    tiles
}

fn tile_index(x: usize, y: usize) -> usize {
    y * MAP_WIDTH + x
}

fn parse_building_kind(kind: &str) -> Option<BuildingKind> {
    match kind {
        "food_bush" | "FoodBush" => Some(BuildingKind::FoodBush),
        "bed" | "Bed" => Some(BuildingKind::Bed),
        _ => None,
    }
}
