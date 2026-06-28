## Purpose

Defines player-directed construction behavior for MVP buildings.

## Requirements

### Requirement: Player selects construction locations
The system SHALL require the player to choose the map tile where a food bush or bed will be built.

#### Scenario: Player places build order
- **WHEN** the player selects a build type and clicks a buildable tile
- **THEN** the UX sends a build command for that building type and tile

### Requirement: Supported MVP building types
The construction system SHALL support food bush and bed building types in the MVP.

#### Scenario: Player opens build options
- **WHEN** the player views available build actions
- **THEN** food bush and bed are available as construction options

### Requirement: Build command validation
The simulation SHALL validate build commands against map bounds, terrain buildability, and tile occupancy.

#### Scenario: Invalid build target
- **WHEN** the player attempts to build outside the map, on water, or on an occupied tile
- **THEN** the command is rejected and no construction job is created

### Requirement: Unit performs construction
The simulation SHALL assign accepted construction jobs to units and complete the requested building after required work advances.

#### Scenario: Accepted construction completes
- **WHEN** an accepted construction job receives enough unit work over simulation ticks
- **THEN** the requested building is added to the target tile

### Requirement: Construction feedback
The system SHALL expose pending and completed construction state to the UX.

#### Scenario: Construction job is active
- **WHEN** a construction job is pending or in progress
- **THEN** snapshots include the target tile, building type, and progress state
