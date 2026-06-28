## Purpose

Defines the Rust Wasm simulation core and Web Worker boundary for the idle colony MVP.

## Requirements

### Requirement: Rust core compiled to Wasm
The simulation core SHALL be implemented in Rust and compiled to WebAssembly for browser execution.

#### Scenario: Frontend loads simulation core
- **WHEN** the frontend starts the game
- **THEN** the Web Worker loads and initializes the Wasm simulation module

### Requirement: Core runs inside Web Worker
The Wasm simulation core SHALL run behind a Web Worker interface rather than directly on the browser main thread.

#### Scenario: Simulation advances
- **WHEN** simulation ticks advance
- **THEN** tick processing occurs through the worker without blocking the main UI thread API

### Requirement: ECS simulation model
The Rust core SHALL use `bevy_ecs` for simulation entities, components, resources, schedules, or events.

#### Scenario: Unit state is simulated
- **WHEN** the core advances units, needs, jobs, or buildings
- **THEN** those simulation concepts are represented through ECS-managed state

### Requirement: Event bus boundary
The worker bridge SHALL communicate with the simulation core using explicit command and event messages.

#### Scenario: Player issues build command
- **WHEN** the UX sends a build command to the worker
- **THEN** the command is passed to the core event handling path and produces either a rejection event or updated state

### Requirement: State snapshots
The worker SHALL emit serializable state snapshots containing map, units, buildings, needs, and construction jobs.

#### Scenario: Tick completes
- **WHEN** a simulation tick completes
- **THEN** the worker posts a state snapshot that the UX can render without direct access to Rust memory
