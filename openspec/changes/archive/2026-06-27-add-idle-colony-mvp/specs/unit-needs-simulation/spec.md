## ADDED Requirements

### Requirement: Three starting units
The simulation SHALL initialize exactly three controllable units for the MVP.

#### Scenario: New simulation starts units
- **WHEN** the simulation is initialized
- **THEN** the state contains three units with unique identifiers and map positions

### Requirement: Unit food need
Each unit SHALL have a food need value that decreases as simulation time advances.

#### Scenario: Food decreases over time
- **WHEN** simulation ticks advance without the unit eating
- **THEN** the unit food value decreases from its prior value

### Requirement: Unit sleep need
Each unit SHALL have a sleep need value that decreases as simulation time advances while the unit is awake.

#### Scenario: Sleep decreases while awake
- **WHEN** simulation ticks advance and the unit is not sleeping
- **THEN** the unit sleep value decreases from its prior value

### Requirement: Zero food is non-lethal
A unit SHALL NOT die or be removed from the simulation when its food need reaches zero in the MVP.

#### Scenario: Food reaches zero
- **WHEN** a unit food value reaches zero
- **THEN** the unit remains alive, controllable, and visible in snapshots

### Requirement: Food can be restored from food bushes
The simulation SHALL allow units to restore food by using a completed food bush building.

#### Scenario: Hungry unit uses food bush
- **WHEN** a unit uses an available food bush
- **THEN** the unit food value increases above its prior value

### Requirement: Sleep can be restored from beds
The simulation SHALL allow units to restore sleep by using a completed bed building.

#### Scenario: Tired unit uses bed
- **WHEN** a unit uses an available bed
- **THEN** the unit sleep value increases above its prior value
