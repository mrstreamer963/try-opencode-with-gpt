## 1. Project Setup

- [x] 1.1 Create the Vite + Vue + TypeScript frontend project structure.
- [x] 1.2 Add PixiJS and frontend build dependencies.
- [x] 1.3 Create the Rust simulation core crate configured for `cdylib` Wasm output.
- [x] 1.4 Add Rust dependencies for `bevy_ecs`, `serde`, `serde_wasm_bindgen`, and Wasm bindings.
- [x] 1.5 Add npm scripts or build documentation for frontend dev, Rust tests, and Wasm build.

## 2. Rust Simulation Core

- [x] 2.1 Implement fixed 30x30 map state with land, water, and grass terrain types.
- [x] 2.2 Implement building state with one building per tile and food bush/bed building types.
- [x] 2.3 Implement three starting unit entities with unique IDs, positions, food, and sleep needs.
- [x] 2.4 Implement simulation ticks that decrease food and sleep needs over time.
- [x] 2.5 Ensure units remain alive, controllable, and present when food reaches zero.
- [x] 2.6 Implement food restoration from completed food bushes.
- [x] 2.7 Implement sleep restoration from completed beds.
- [x] 2.8 Add Rust tests for map dimensions, terrain typing, unit initialization, needs decay, and zero-food non-death behavior.

## 3. Construction System

- [x] 3.1 Implement build command DTOs for food bush and bed placement.
- [x] 3.2 Validate build commands for map bounds, water tiles, and occupied tiles.
- [x] 3.3 Create construction jobs for accepted player build commands.
- [x] 3.4 Assign construction jobs to units and advance construction progress on ticks.
- [x] 3.5 Complete construction jobs by placing the requested building on the target tile.
- [x] 3.6 Add Rust tests for accepted construction, rejected invalid targets, occupancy rejection, and completed building snapshots.

## 4. Wasm Worker Bridge

- [x] 4.1 Expose simulation initialization, command handling, tick advancement, and snapshot export from Rust to Wasm.
- [x] 4.2 Implement serializable command, event, and state snapshot schemas for the worker boundary.
- [x] 4.3 Implement a Web Worker that loads the Wasm module and handles start, tick, and build messages.
- [x] 4.4 Emit state snapshots after initialization and simulation ticks.
- [x] 4.5 Emit command rejection feedback for invalid build commands.
- [x] 4.6 Add a minimal worker integration test or smoke test for start, tick, and build command flow.

## 5. Vue + PixiJS UX

- [x] 5.1 Build the main Vue app shell with game canvas area and unit needs panel.
- [x] 5.2 Initialize PixiJS rendering inside a Vue component lifecycle.
- [x] 5.3 Render 30x30 terrain tiles from worker snapshots.
- [x] 5.4 Render units, completed buildings, and active construction indicators.
- [x] 5.5 Add build mode controls for food bush and bed.
- [x] 5.6 Convert pointer and touch tile selection into worker build commands.
- [x] 5.7 Display current food and sleep values for all three units.
- [x] 5.8 Display visible feedback when a build command is rejected.
- [x] 5.9 Ensure the layout is usable on desktop and mobile viewport sizes.

## 6. Integration and Verification

- [x] 6.1 Wire the frontend tick loop to the worker and keep PixiJS rendering synced to snapshots.
- [x] 6.2 Verify the player can place food bush and bed construction jobs on buildable tiles.
- [x] 6.3 Verify units complete construction and the resulting buildings appear on the map.
- [x] 6.4 Verify food and sleep needs visibly change and can be restored by the corresponding buildings.
- [x] 6.5 Run Rust tests for the simulation core.
- [x] 6.6 Run frontend typecheck/build.
- [x] 6.7 Run the app locally and perform a manual MVP smoke test in the browser.
