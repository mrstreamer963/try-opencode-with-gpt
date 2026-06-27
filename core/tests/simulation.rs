use idle_core::{BuildingKind, CommandRejection, Simulation, Terrain, MAP_HEIGHT, MAP_WIDTH};

#[test]
fn simulation_starts_with_fixed_30_by_30_map_and_supported_terrain() {
    let mut simulation = Simulation::new();
    let snapshot = simulation.snapshot();

    assert_eq!(snapshot.map.width, MAP_WIDTH);
    assert_eq!(snapshot.map.height, MAP_HEIGHT);
    assert_eq!(snapshot.map.tiles.len(), MAP_WIDTH * MAP_HEIGHT);
    assert!(snapshot
        .map
        .tiles
        .iter()
        .all(|tile| matches!(tile.terrain, Terrain::Land | Terrain::Water | Terrain::Grass)));
}

#[test]
fn simulation_starts_with_three_living_units_with_needs() {
    let mut simulation = Simulation::new();
    let snapshot = simulation.snapshot();

    assert_eq!(snapshot.units.len(), 3);
    assert!(snapshot.units.iter().all(|unit| unit.alive));
    assert!(snapshot.units.iter().all(|unit| unit.food > 0.0));
    assert!(snapshot.units.iter().all(|unit| unit.sleep > 0.0));
}

#[test]
fn ticks_decay_needs_without_killing_units_at_zero_food() {
    let mut simulation = Simulation::new();
    let initial = simulation.snapshot().units[0].clone();

    for _ in 0..400 {
        simulation.tick();
    }

    let unit = simulation.snapshot().units[0].clone();
    assert!(unit.food < initial.food);
    assert!(unit.sleep < initial.sleep);
    assert_eq!(unit.food, 0.0);
    assert!(unit.alive);
}

#[test]
fn accepted_construction_job_completes_and_appears_in_snapshot() {
    let mut simulation = Simulation::new();

    let job_id = simulation
        .build_at(BuildingKind::FoodBush, 4, 4)
        .expect("build command should be accepted");
    assert!(simulation.snapshot().construction_jobs.iter().any(|job| job.id == job_id));

    for _ in 0..5 {
        simulation.tick();
    }

    let snapshot = simulation.snapshot();
    assert!(snapshot.construction_jobs.is_empty());
    assert!(snapshot
        .buildings
        .iter()
        .any(|building| building.kind == BuildingKind::FoodBush && building.x == 4 && building.y == 4));
}

#[test]
fn build_commands_reject_out_of_bounds_water_and_occupied_tiles() {
    let mut simulation = Simulation::new();

    assert_eq!(
        simulation.build_at(BuildingKind::Bed, MAP_WIDTH, 2),
        Err(CommandRejection::OutOfBounds)
    );
    assert_eq!(
        simulation.build_at(BuildingKind::Bed, 0, 0),
        Err(CommandRejection::NotBuildable)
    );

    simulation
        .build_at(BuildingKind::Bed, 5, 5)
        .expect("first build should be accepted");
    for _ in 0..5 {
        simulation.tick();
    }

    assert_eq!(
        simulation.build_at(BuildingKind::FoodBush, 5, 5),
        Err(CommandRejection::Occupied)
    );
}

#[test]
fn food_bush_and_bed_restore_matching_unit_needs() {
    let mut simulation = Simulation::new();

    simulation
        .build_at(BuildingKind::FoodBush, 6, 6)
        .expect("food bush should be accepted");
    simulation
        .build_at(BuildingKind::Bed, 7, 7)
        .expect("bed should be accepted");

    for _ in 0..5 {
        simulation.tick();
    }

    for _ in 0..220 {
        simulation.tick();
    }

    let unit = simulation.snapshot().units[0].clone();
    assert!(unit.food > 0.0);
    assert!(unit.sleep > 0.0);
}
