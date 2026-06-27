## Why

The project needs a first playable MVP for an idle colony game inspired by StarDeus, with a clear split between a browser UX layer and a deterministic simulation core. This change establishes the initial gameplay loop, technical architecture, and implementation path for validating unit needs, player-directed construction, and a tile-based world.

## What Changes

- Add a Vite + Vue + PixiJS UX application for rendering and interacting with a 30x30 tile map.
- Add a Rust simulation core compiled to WebAssembly and hosted in a Web Worker.
- Use `bevy_ecs` in the Rust core for units, needs, map entities, constructions, and simulation events.
- Introduce an event bus between the UX worker bridge and the core simulation.
- Add a 30x30 map with terrain tile types: land, water, and grass.
- Add support for map buildings, including food bushes and beds.
- Add three controllable units with food and sleep needs.
- Add player-directed build commands so the player chooses where units build food bushes and beds.
- Ensure units do not die when food reaches zero during MVP; zero food remains a visible unmet need/state.

## Capabilities

### New Capabilities
- `tile-map-simulation`: Defines the 30x30 tile world, terrain types, buildings on map tiles, and map validity rules.
- `unit-needs-simulation`: Defines three units, food and sleep need changes over time, and non-lethal zero-food behavior.
- `player-directed-construction`: Defines player-issued build commands, unit construction behavior, food bushes, and beds.
- `wasm-worker-core`: Defines the Rust `bevy_ecs` simulation core compiled to Wasm, hosted in a Web Worker, and controlled through an event bus.
- `pixi-vue-ux`: Defines the Vite + Vue + PixiJS UX for rendering the map, units, buildings, needs, and player build interactions.

### Modified Capabilities

None.

## Impact

- Creates a new frontend application structure for Vite, Vue, and PixiJS.
- Creates a new Rust crate for the simulation core and Wasm build target.
- Adds JavaScript/TypeScript worker bridge code for communicating with the Wasm core.
- Adds dependency requirements for Vue, PixiJS, Vite, Rust Wasm tooling, `wasm-bindgen`, and `bevy_ecs`.
- Establishes initial message/event schemas between UX and core, including simulation ticks, state snapshots, and player commands.
