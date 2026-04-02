---
applyTo: "src/systems/**/*.rs"
---

# Skill: Test Coverage Rules

## Location

Tests live in a `#[cfg(test)]` module **at the bottom of the same file** as the system
they test. Never put system tests in a separate `_test.rs` file.

## Minimum coverage per system

Every system file must have at minimum:

| Test type | What it checks |
|-----------|---------------|
| **Happy path** | Normal inputs produce correct outputs |
| **Boundary / edge case** | Values at the exact threshold (0.0, 1.0, threshold constants) |
| **No-op / empty world** | System does not panic when no entities match the query |

Suggested additional tests per system are listed in comments at the bottom of each test
module (see scaffold in `src/systems/_test_scaffold.rs`).

## App setup — always MinimalPlugins

```rust
fn make_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    // Insert only the resources your system touches
    app.insert_resource(TileGrid::new_empty());
    app.insert_resource(SpatialHashGrid::new(CHUNKS_PER_ROW));
    app.insert_resource(GlobalClock::default());
    app
}
```

Never use `DefaultPlugins` in tests — it opens a window and fails in CI.

## Naming convention

```
test_<system_name>_<scenario>
```

Examples:
```rust
test_brain_system_no_entities_does_not_panic
test_brain_system_all_inputs_produce_bounded_output
test_movement_system_blocked_tile_costs_energy_penalty
test_movement_system_below_threshold_does_not_move
test_combat_system_damage_formula_at_max_size
test_combat_system_duration_exactly_five_ticks
test_mortality_system_energy_zero_triggers_death
test_mortality_system_health_zero_triggers_death
test_mortality_system_corpse_meat_written_to_tile
test_reproduction_system_same_tick_pair_deduplication
```

## Constants in tests — never hardcode

Every threshold, rate, or count used in a test assertion must come from `crate::constants`:

```rust
use crate::constants::*;

// ✅ Correct — assertion updates automatically when the spec changes
assert!((tile.corpse_meat - 0.5 * CORPSE_MEAT_PER_SIZE).abs() < 1e-5);

// ❌ Wrong — will silently pass even if the spec constant changes
assert!((tile.corpse_meat - 2.5).abs() < 1e-5);
```

## Testing formulas — use the spec formula, not the result

When testing a computed value, drive the expected result from the same formula
the spec defines, not from a pre-calculated literal:

```rust
// ✅ Correct — derives expected from the formula in §6.1
let expected_gain = EAT_RATE * genome.plant_digestion * PLANT_ENERGY_SCALE;
assert!((vitals.energy - initial_energy - expected_gain).abs() < 1e-5);

// ❌ Wrong — breaks silently if EAT_RATE or PLANT_ENERGY_SCALE change
assert!((vitals.energy - initial_energy - 0.15).abs() < 1e-5);
```

## Testing exclusive systems

Exclusive systems (`collision_system`, `reproduction_system`, `mortality_system`)
cannot be tested with a standard parallel query. Use `world.run_system_once()`:

```rust
#[test]
fn test_mortality_system_energy_zero_triggers_death() {
    let mut world = World::new();
    // setup world...
    world.run_system_once(mortality_system);
    // assertions...
}
```

## Tolerance for float comparisons

Always use an epsilon, never `==` on floats:

```rust
const EPS: f32 = 1e-5;
assert!((result - expected).abs() < EPS, "got {result}, expected {expected}");
```

## What not to test

- **Rendering** (`render_system`, UI panels) — skip, test visually.
- **Perlin noise output** — test that biomes were assigned (non-zero), not exact values.
- **RNG-dependent combat outcomes** — test the formula bounds, not a specific roll.
  Use seeded RNG (`rand::SeedableRng`) to make probabilistic tests deterministic:

```rust
use rand::SeedableRng;
use rand::rngs::SmallRng;

let mut rng = SmallRng::seed_from_u64(42); // deterministic seed
let damage = compute_combat_damage(&attacker, &defender, &mut rng);
assert!(damage >= 0.0);
assert!(damage <= attacker.size * COMBAT_DAMAGE_SCALE);
```
