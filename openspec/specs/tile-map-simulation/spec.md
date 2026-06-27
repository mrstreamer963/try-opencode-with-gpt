## Purpose

Defines the fixed tile-map simulation model for the idle colony MVP.

## Requirements

### Requirement: Fixed MVP map dimensions
The simulation SHALL create and maintain a fixed 30x30 tile map for the MVP.

#### Scenario: New simulation starts with fixed map
- **WHEN** the simulation is initialized
- **THEN** the authoritative map contains exactly 30 columns and 30 rows

### Requirement: Terrain types
Each map tile SHALL have exactly one terrain type from land, water, or grass.

#### Scenario: Snapshot includes terrain
- **WHEN** the UX receives a map snapshot
- **THEN** every tile includes its terrain type as land, water, or grass

### Requirement: Building occupancy
The simulation SHALL allow at most one building on a tile.

#### Scenario: Tile already has building
- **WHEN** a build command targets a tile that already contains a building
- **THEN** the simulation rejects the command and leaves the existing building unchanged

### Requirement: Water blocks construction
The simulation SHALL reject building placement on water tiles.

#### Scenario: Player targets water
- **WHEN** a build command targets a water tile
- **THEN** the simulation rejects the command and reports that the tile is not buildable

### Requirement: Map buildings in snapshots
The simulation SHALL include placed buildings in state snapshots sent to the UX.

#### Scenario: Building exists on map
- **WHEN** a state snapshot is emitted after a building is completed
- **THEN** the snapshot includes the building type and tile position
