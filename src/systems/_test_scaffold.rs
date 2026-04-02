/// # Test Scaffold — Copy this pattern into every system file
///
/// Convention: tests live in a `#[cfg(test)]` module at the bottom of the same
/// file as the system they test. Never put system tests in a separate file.
///
/// Naming: `test_<system_name>_<scenario>`
/// Minimum coverage per system: one happy-path + one boundary/edge case.
///
/// This file shows the scaffold for `mortality_system` as a worked example.
/// Replace all `mortality`-specific content with your system's logic.

#[cfg(test)]
use bevy::prelude::*;

#[cfg(test)]
use crate::components::{PhysicalGenome, Position, VitalSigns};
#[cfg(test)]
use crate::constants::*;
#[cfg(test)]
use crate::resources::TileGrid;

// ── System code lives above ────────────────────────────────────────────────

// This scaffold file intentionally leaves actual system implementation to `crate::systems`.

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::mortality_system;

    /// Helper: build the minimal Bevy App needed to run a single system.
    /// Always use MinimalPlugins — never DefaultPlugins in tests.
    fn make_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        // Register any Resources your system needs
        app.insert_resource(TileGrid::new(GRID_WIDTH, GRID_HEIGHT));
        app
    }

    /// Helper: spawn a test entity with controlled VitalSigns.
    fn spawn_entity(app: &mut App, energy: f32, health: f32) -> Entity {
        app.world_mut().spawn((
            Position { x: 512, y: 512 },
            VitalSigns {
                energy,
                health,
                age: 0,
                lifetime_stress: 0.0,
                speed_modifier: 1.0,
                health_regen_modifier: 1.0,
            },
            PhysicalGenome {
                size: 0.5,
                speed: 0.5,
                sensory_radius: 0.5,
                thermal_tolerance: 0.5,
                aquatic_adaptation: 0.0,
            },
        )).id()
    }

    // ── Happy path ─────────────────────────────────────────────────────────

    /// A living entity (energy > 0, health > 0) must survive the mortality system.
    #[test]
    fn test_mortality_system_healthy_entity_survives() {
        let mut app = make_test_app();
        app.add_systems(Update, mortality_system);

        let entity = spawn_entity(&mut app, 5.0, 1.0); // healthy

        app.update();

        // Entity must still exist
        assert!(app.world().get_entity(entity).is_some(), "Healthy entity was incorrectly despawned");
    }

    // ── Boundary: energy at exactly zero ──────────────────────────────────

    /// An entity with energy == 0.0 must be despawned and leave corpse_meat on its tile.
    #[test]
    fn test_mortality_system_energy_zero_triggers_death() {
        let mut app = make_test_app();
        app.add_systems(Update, mortality_system);

        let entity = spawn_entity(&mut app, 0.0, 1.0); // dead from starvation

        app.update();

        assert!(
            app.world().get_entity(entity).is_none(),
            "Entity with energy=0 was not despawned"
        );

        // Verify corpse_meat was written to the tile
        let tile_grid = app.world().resource::<TileGrid>();
        let tile = tile_grid.get(512, 512);
        // size=0.5 → corpse_meat should be 0.5 * CORPSE_MEAT_PER_SIZE = 2.5
        let expected_meat = 0.5 * CORPSE_MEAT_PER_SIZE;
        assert!(
            (tile.corpse_meat - expected_meat).abs() < 1e-5,
            "Expected corpse_meat={}, got={}", expected_meat, tile.corpse_meat
        );
    }

    // ── Boundary: health at exactly zero ──────────────────────────────────

    /// An entity killed in combat (health == 0) must also be despawned.
    #[test]
    fn test_mortality_system_health_zero_triggers_death() {
        let mut app = make_test_app();
        app.add_systems(Update, mortality_system);

        let entity = spawn_entity(&mut app, 5.0, 0.0); // dead from combat

        app.update();

        assert!(
            app.world().get_entity(entity).is_none(),
            "Entity with health=0 was not despawned"
        );
    }

    // ── Add more tests below as implementation expands ─────────────────────
    //
    // Suggested next tests:
    //   test_mortality_system_chronicle_event_emitted_for_old_entity
    //   test_mortality_system_entity_removed_from_spatial_hash
    //   test_mortality_system_two_deaths_same_tile_accumulates_meat
}
