## ADDED Requirements

### Requirement: Vite Vue application
The UX SHALL be implemented as a Vite application using Vue.

#### Scenario: Development app starts
- **WHEN** the frontend development server runs
- **THEN** the browser loads a Vue-based game UI

### Requirement: PixiJS map rendering
The UX SHALL use PixiJS to render the tile map, units, buildings, and construction indicators.

#### Scenario: Snapshot is rendered
- **WHEN** the UX receives a state snapshot
- **THEN** PixiJS renders terrain tiles, unit markers, building markers, and active construction markers

### Requirement: Build interaction UI
The UX SHALL allow the player to select food bush or bed build mode and choose a target tile on the map.

#### Scenario: Player selects build target
- **WHEN** the player selects a building type and clicks a tile
- **THEN** the UX sends a build command to the worker with the selected type and tile coordinates

### Requirement: Unit needs display
The UX SHALL display food and sleep values for each of the three units.

#### Scenario: Needs change after tick
- **WHEN** a new snapshot contains changed food or sleep values
- **THEN** the displayed unit need indicators update to match the snapshot

### Requirement: Command rejection feedback
The UX SHALL show visible feedback when the simulation rejects a build command.

#### Scenario: Invalid build attempt rejected
- **WHEN** the worker reports a rejected build command
- **THEN** the UX displays a short message or tile feedback explaining that the build target is invalid

### Requirement: Desktop and mobile usability
The UX SHALL support both desktop pointer interaction and mobile/touch interaction for selecting build modes and map tiles.

#### Scenario: Mobile player places building
- **WHEN** a touch user selects a build type and taps a buildable tile
- **THEN** the UX sends the same build command as a desktop click interaction
