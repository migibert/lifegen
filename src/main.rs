use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod constants;
mod components;
mod resources;
mod systems;

use constants::*;
use resources::{TileGrid, SpatialHashGrid, GlobalClock};
use systems::{
    input_system,
    global_cycle_system,
    tile_update_system,
    spatial_update_system,
    brain_system,
    movement_system,
    eat_system,
    combat_system,
    collision_system,
    reproduction_system,
    mortality_system,
    aging_system,
    stress_system,
    chronicle_system,
    render_system,
    census_system,
};

fn main() {
    let mut app = App::new();

    // ── Plugins ───────────────────────────────────────────────────────────────
    #[cfg(not(feature = "headless"))]
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "A-Life Simulator".to_string(),
            resolution: (1400.0, 900.0).into(),
            ..default()
        }),
        ..default()
    }));

    #[cfg(feature = "headless")]
    app.add_plugins(MinimalPlugins);

    #[cfg(not(feature = "headless"))]
    app.add_plugins(EguiPlugin);

    // ── Resources (shared mutable world state) ────────────────────────────────
    app.insert_resource(TileGrid::new(GRID_WIDTH, GRID_HEIGHT));
    app.insert_resource(SpatialHashGrid::new(CHUNKS_PER_ROW));
    app.insert_resource(GlobalClock::default());

    // ── Startup: genesis phase ────────────────────────────────────────────────
    app.add_systems(Startup, genesis_system);

    // ── Update: systems run in spec-defined order each tick ───────────────────
    //
    // Parallel systems: declared with .chain() only where strict ordering
    // within a stage is required. Bevy parallelises the rest automatically.
    //
    // Exclusive systems: run alone, declared separately with .after() anchors
    // to preserve the §11 execution order.
    app.add_systems(Update, (
        // Stage 1–4: world state
        input_system,
        global_cycle_system.after(input_system),
        tile_update_system.after(global_cycle_system),
        spatial_update_system.after(tile_update_system),

        // Stage 5–8: entity actions (parallel within stage, after spatial)
        brain_system.after(spatial_update_system),
        movement_system.after(brain_system),
        eat_system.after(brain_system),
        combat_system.after(spatial_update_system),

        // Stage 12–15: bookkeeping (parallel, after exclusive stages below)
        aging_system,
        stress_system,
        chronicle_system,

        #[cfg(not(feature = "headless"))]
        render_system,
    ));

    // Exclusive systems — run sequentially, after all parallel systems above.
    // Bevy guarantees exclusivity when scheduled with .after() on a parallel stage.
    app.add_systems(Update, (
        collision_system,
        reproduction_system.after(collision_system),
        mortality_system.after(reproduction_system),
    ).after(combat_system));

    // Census runs every 500 ticks — gated inside the system itself via GlobalClock.
    app.add_systems(Update, census_system.after(mortality_system));

    app.run();
}

/// Genesis phase — §10.
/// Populates the TileGrid with biome/biomass data and spawns the primordial entities.
fn genesis_system(
    mut commands:     Commands,
    mut tile_grid:    ResMut<TileGrid>,
    mut spatial_hash: ResMut<SpatialHashGrid>,
) {
    // 1. Generate biomes via Perlin noise and seed baseline biomass/nutrients
    tile_grid.generate_biomes();

    // 2. Spawn INITIAL_POPULATION entities inside the central primordial zone
    for _ in 0..INITIAL_POPULATION {
        let x = PRIMORDIAL_ZONE_ORIGIN
            + rand::random::<u32>() % PRIMORDIAL_ZONE_SIZE;
        let y = PRIMORDIAL_ZONE_ORIGIN
            + rand::random::<u32>() % PRIMORDIAL_ZONE_SIZE;

        let entity = commands.spawn(components::primordial_bundle(x, y)).id();
        spatial_hash.insert(x, y, entity);
    }

    info!(
        "Genesis complete — {} entities spawned in {}×{} primordial zone",
        INITIAL_POPULATION, PRIMORDIAL_ZONE_SIZE, PRIMORDIAL_ZONE_SIZE
    );
}
