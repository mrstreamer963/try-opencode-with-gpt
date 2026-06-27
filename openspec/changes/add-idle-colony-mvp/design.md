## Context

This change starts a new MVP project for an idle colony simulation inspired by StarDeus. The repository currently contains planning artifacts only, so the implementation can establish the initial structure without migration constraints.

The target architecture has two runtime parts: a browser UX built with Vite, Vue, and PixiJS, and a Rust simulation core compiled to WebAssembly. The Wasm module runs inside a Web Worker so simulation ticks do not block rendering or input handling. The Rust core owns authoritative game state and exposes state snapshots and command/event handling to the UX.

## Goals / Non-Goals

**Goals:**

- Build a first playable idle-colony MVP with a 30x30 tile map, three units, basic needs, and player-directed construction.
- Keep simulation state authoritative in Rust using `bevy_ecs`.
- Compile the Rust core to Wasm and run it from a Web Worker.
- Use an explicit message/event boundary between Vue/Pixi UX and the core.
- Render terrain, units, buildings, selected build mode, build targets, and unit need indicators.

**Non-Goals:**

- No procedural world generation beyond a deterministic starter map.
- No combat, death, diseases, resource economy, hauling inventory, pathfinding optimization, saves, or multiplayer.
- No autonomous building placement; the player chooses construction locations.
- No advanced art pipeline; MVP visuals can use colored primitives or simple generated sprites.

## Decisions

### Rust core owns simulation state

The Rust core will store the map, units, buildings, needs, jobs, and event queue. The UX will only send commands and render snapshots.

Rationale: this keeps gameplay deterministic, testable, and isolated from rendering concerns.

Alternative considered: keeping state in Vue and using Rust only for computations. That would make early integration simpler but would blur authority and make later simulation growth harder.

### Use `bevy_ecs` without full Bevy engine

The core will depend on `bevy_ecs` for entities, components, resources, schedules, and events, but not on Bevy rendering or app runtime.

Rationale: ECS fits units, jobs, needs, and map entities while keeping the Wasm payload and browser integration smaller than a full engine stack.

Alternative considered: a custom struct-based simulation. That would reduce dependencies but would make job/state extensibility weaker.

### Worker message boundary uses serializable command and snapshot DTOs

The UX will post commands such as `Start`, `Tick`, and `BuildAt`, and the worker will post snapshots and events such as `StateSnapshot`, `CommandRejected`, and `ConstructionCompleted`.

Rationale: serializable DTOs make the worker boundary explicit and allow the UX to remain framework-agnostic around simulation state.

Alternative considered: directly exposing Wasm objects to the main thread. That conflicts with the worker requirement and makes memory ownership harder to reason about.

### MVP uses command-driven ticks

The worker will run simulation advancement through an explicit tick loop controlled by the worker wrapper. Each tick advances needs and jobs, then emits a compact state snapshot.

Rationale: explicit ticks are straightforward to test in Rust and easy to throttle from the browser.

Alternative considered: running a continuous loop inside Rust. That can work later, but command-driven ticks are easier for MVP debugging and deterministic tests.

### Simple tile occupancy and construction rules

The map will be a fixed 30x30 grid. Land and grass are traversable/buildable for MVP; water is blocked and not buildable. One building can occupy one tile.

Rationale: these rules support clear UX feedback and avoid building/pathing complexity before the core loop is proven.

Alternative considered: multi-tile buildings and richer terrain modifiers. That adds complexity without helping the first playable validation.

## Risks / Trade-offs

- Wasm + Worker + Vue integration may slow initial setup -> Mitigation: create a minimal worker bridge first with start/tick/snapshot before adding gameplay detail.
- `bevy_ecs` may increase Wasm bundle size -> Mitigation: use only `bevy_ecs`, avoid full Bevy, and keep feature usage minimal.
- Pathfinding can become complex even on a 30x30 map -> Mitigation: use a simple grid pathfinder or nearest-job movement suitable for MVP scale.
- Frequent full-state snapshots may be inefficient -> Mitigation: 30x30 and three units are small enough for MVP; add delta updates later only if profiling shows a problem.
- Ambiguous StarDeus-like expectations may expand scope -> Mitigation: keep MVP limited to needs, construction, and map interaction listed in specs.

## Migration Plan

No existing runtime code needs migration. Implementation can be introduced as a new project structure with frontend, Rust core, Wasm build, and worker bridge. Rollback is removing the new app/core files and the associated package/tooling configuration.

## Open Questions

- Exact visual style and UI language are not specified; MVP will use functional visuals unless a separate design direction is provided.
- The production build pipeline for Rust Wasm can use `wasm-pack` or a Vite Wasm plugin; implementation should choose the smallest reliable setup for this repository.
