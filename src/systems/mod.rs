use bevy::prelude::*;
use crate::components::{PhysicalGenome, Position, VitalSigns};
use crate::resources::TileGrid;

#[allow(unused_variables)]
pub fn input_system() {}

#[allow(unused_variables)]
pub fn global_cycle_system() {}

#[allow(unused_variables)]
pub fn tile_update_system() {}

#[allow(unused_variables)]
pub fn spatial_update_system() {}

#[allow(unused_variables)]
pub fn brain_system() {}

#[allow(unused_variables)]
pub fn movement_system() {}

#[allow(unused_variables)]
pub fn eat_system() {}

#[allow(unused_variables)]
pub fn combat_system() {}

#[allow(unused_variables)]
pub fn collision_system() {}

#[allow(unused_variables)]
pub fn reproduction_system() {}

pub fn mortality_system(
    mut commands: Commands,
    query: Query<(Entity, &VitalSigns, &PhysicalGenome, &Position)>,
    mut tile_grid: ResMut<TileGrid>,
) {
    let mut deaths = Vec::new();

    for (entity, vital_signs, genome, pos) in query.iter() {
        if vital_signs.energy <= 0.0 || vital_signs.health <= 0.0 {
            deaths.push((entity, genome.size, pos.x, pos.y));
        }
    }

    for (entity, size, x, y) in deaths {
        commands.entity(entity).despawn_recursive();
        if let Some(tile_idx) = tile_grid.index(x, y) {
            if let Some(tile) = tile_grid.tiles.get_mut(tile_idx) {
                tile.corpse_meat += size * crate::constants::CORPSE_MEAT_PER_SIZE;
            }
        }
    }
}

#[allow(unused_variables)]
pub fn aging_system() {}

#[allow(unused_variables)]
pub fn stress_system() {}

#[allow(unused_variables)]
pub fn chronicle_system() {}

#[allow(unused_variables)]
pub fn render_system() {}

#[allow(unused_variables)]
pub fn census_system() {}

// optionally include scaffold module for future tests
pub mod _test_scaffold;
