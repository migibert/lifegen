# GitHub Copilot — Custom Instructions
# Rust + Bevy ECS A-Life Simulator

## Project context

> **Full specification:** `spec/alife_spec_v2.md` — this is the canonical source of truth for all
> system behaviour, constants, and architecture decisions. When in doubt, defer to the spec.


This is a Data-Oriented Design (DOD) autonomous A-Life simulator written in Rust using Bevy ECS.
Entities have no methods — all logic lives in systems that operate on flat component arrays.
The world is a 1D `Vec<Tile>` representing a 1024×1024 grid. Plants are NOT entities; they are
`biomass: f32` fields on `Tile`. Neural networks drive entity behaviour using nalgebra matrix math.

## Architecture rules — always follow these

- **Never suggest OOP patterns.** No `impl Entity { fn update(&self) }`. Logic goes in Bevy systems.
- **Never suggest `entity.get_component()`** inside a hot loop. Use ECS queries with `Query<>`.
- **Parallel by default.** All systems should be `fn my_system(query: Query<...>)` unless they
  mutate shared resources, in which case use `ResMut<>` and note the exclusivity requirement.
- **Exclusive systems** (those that spawn/despawn entities or need full World access) must use
  `fn my_system(world: &mut World)` and be scheduled with `.add_systems(Update, my_system.exclusive_system())`.
- **Commands for structural changes.** Spawning, despawning, and adding/removing components must
  use `Commands`, never direct World mutation inside a parallel system.
- **No `unwrap()` in system code.** Use `if let Some(x) = ...` or `? ` with proper error types.
  Panics in a Bevy system crash the entire sim.

## The canonical system execution order

Systems MUST be ordered according to the spec. When suggesting new systems, place them in the
correct stage relative to this sequence:

```
1.  input_system
2.  global_cycle_system
3.  tile_update_system
4.  spatial_update_system
5.  brain_system          ← neural net forward pass (parallel)
6.  movement_system       ← apply energy costs inline (parallel)
7.  eat_system            (parallel)
8.  combat_system         (parallel, operates on CombatState component)
9.  collision_system      ← EXCLUSIVE — initiates CombatState
10. reproduction_system   ← EXCLUSIVE — spawns offspring
11. mortality_system      ← EXCLUSIVE — despawns entities, writes corpse to tile
12. aging_system          (parallel)
13. stress_system         (parallel)
14. chronicle_system
15. render_system
16. census_system         ← runs every 500 ticks only
```

## Constants — always import from `crate::constants`

Never hardcode a magic number. Every numeric constant from the spec lives in `src/constants.rs`.
Always import and use them by name. If a value is not yet in `constants.rs`, add it there first.

Key constants (do not inline these values):
```rust
use crate::constants::*;

GRID_WIDTH, GRID_HEIGHT, GRID_SIZE
CHUNK_SIZE, MAX_BIOMASS
BASE_REGEN_FOREST, BASE_REGEN_PLAINS, BASE_REGEN_DESERT
MAX_LIFESPAN_TICKS
BASE_MOVE_COST, IDLE_COST, THERMAL_PENALTY_SCALE
EAT_RATE, PLANT_ENERGY_SCALE, MEAT_ENERGY_SCALE
COMBAT_DAMAGE_SCALE, COMBAT_DURATION_TICKS, INJURY_HEALTH_THRESHOLD
BASE_HEALTH_REGEN, REPRODUCTION_COST
MUTATION_BASE_RATE, MUTATION_MAX_RATE
INITIAL_POPULATION, PRIMORDIAL_ZONE_SIZE
CENSUS_INTERVAL_TICKS, SPECIATION_MIN_POPULATION, SPECIATION_MIN_DISTANCE
```

## Neural network shape — do not change without updating the spec

- Input vector:  11 × 1  (`SVector<f32, 11>`)
- Hidden vector:  6 × 1  (`SVector<f32, 6>`)
- Output vector:  5 × 1  (`SVector<f32, 5>`)
- W1: `SMatrix<f32, 6, 11>`, W2: `SMatrix<f32, 5, 6>`
- B1: `SVector<f32, 6>`,     B2: `SVector<f32, 5>`
- Forward pass: `H = (W1 * I + B1).map(f32::tanh); O = (W2 * H + B2).map(f32::tanh)`

## Component conventions

- Component structs use `#[derive(Component, Debug, Clone)]`.
- All genome values are `f32` in `[0.0, 1.0]`.
- Runtime modifiers (e.g. `speed_modifier`) live in `VitalSigns`, not genome components.
- `CombatState` is a temporary marker component — always clean it up in `mortality_system` or
  when combat ends.

## Test conventions

- Every system must have a corresponding `#[cfg(test)]` module in the same file.
- Use `bevy::app::App::new()` with `MinimalPlugins` for system tests — never `DefaultPlugins`.
- Test names follow: `test_<system_name>_<scenario>` e.g. `test_mortality_system_energy_zero_death`.
- All constants used in tests must come from `crate::constants`, never be hardcoded in the test.
- At minimum, each system needs: one happy-path test, one edge-case/boundary test.

## Genome and tile indexing

```rust
// Tile indexing — always use this helper, never inline the formula
fn tile_index(x: u32, y: u32) -> usize {
    (y * GRID_WIDTH + x) as usize
}

// Chunk indexing
fn chunk_id(x: u32, y: u32) -> u32 {
    (y / CHUNK_SIZE) * (GRID_WIDTH / CHUNK_SIZE) + (x / CHUNK_SIZE)
}
```
