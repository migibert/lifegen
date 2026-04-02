---
applyTo: "src/**/*.rs"
---

# Skill: Module & File Structure

## Canonical source tree

Every new file must be placed in the correct module. Do not create ad-hoc files at the
`src/` root. The full intended structure is:

```
src/
├── main.rs                     ← app entry point, system scheduling only
├── constants.rs                ← ALL numeric spec values, nothing else
├── components.rs               ← ALL ECS component structs + primordial_bundle()
├── resources.rs                ← TileGrid, SpatialHashGrid, GlobalClock
├── systems/
│   ├── mod.rs                  ← re-exports all systems (pub use)
│   ├── input_system.rs         ← §11 stage 1
│   ├── global_cycle_system.rs  ← §11 stage 2
│   ├── tile_update_system.rs   ← §11 stage 3
│   ├── spatial_update_system.rs← §11 stage 4
│   ├── brain_system.rs         ← §11 stage 5
│   ├── movement_system.rs      ← §11 stage 6
│   ├── eat_system.rs           ← §11 stage 7
│   ├── combat_system.rs        ← §11 stage 8
│   ├── collision_system.rs     ← §11 stage 9  (exclusive)
│   ├── reproduction_system.rs  ← §11 stage 10 (exclusive)
│   ├── mortality_system.rs     ← §11 stage 11 (exclusive)
│   ├── aging_system.rs         ← §11 stage 12
│   ├── stress_system.rs        ← §11 stage 13
│   ├── chronicle_system.rs     ← §11 stage 14
│   ├── render_system.rs        ← §11 stage 15
│   └── census_system.rs        ← §11 stage 16 (every 500 ticks)
├── ui/
│   ├── mod.rs
│   ├── telemetry_panel.rs      ← Global Telemetry Panel (§9)
│   ├── species_ledger.rs       ← Species Ledger + K-Means (§9)
│   ├── chronicle_panel.rs      ← Chronicle log (§9)
│   └── viewport_filters.rs     ← Heatmap toggles (§9)
└── utils/
    ├── mod.rs
    ├── math.rs                 ← tile_index(), chunk_id(), hamming_distance()
    ├── noise.rs                ← Perlin noise biome generation helpers
    └── rng.rs                  ← seeded RNG resource wrapper
```

## One system per file

Each system in `§11` gets its own file. Do not put two systems in one file.
A system file contains: the system function, its helper functions, and its `#[cfg(test)]`
module. Nothing else.

## components.rs conventions

All component structs live in one file. Group them by spec section:

```rust
// §4 — Position
#[derive(Component, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Position { pub x: u32, pub y: u32 }

// §4 — PhysicalGenome
#[derive(Component, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhysicalGenome {
    pub size:               f32,
    pub speed:              f32,
    pub sensory_radius:     f32,
    pub thermal_tolerance:  f32,
    pub aquatic_adaptation: f32,
}
// ... etc
```

All components must derive `serde::Serialize, serde::Deserialize` for snapshot support (§9).

## resources.rs conventions

Resources are structs registered with `app.insert_resource()`. They must not contain
game logic — only data and simple accessor methods:

```rust
pub struct TileGrid {
    pub tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl TileGrid {
    pub fn get(&self, x: u32, y: u32) -> &Tile { ... }
    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut Tile { ... }
}
```

## systems/mod.rs — re-export everything

```rust
pub mod brain_system;
pub mod movement_system;
// ... all systems

pub use brain_system::brain_system;
pub use movement_system::movement_system;
// ... all pub use re-exports
```

This keeps `main.rs` imports clean: `use systems::brain_system;`

## Naming rules

| Thing | Convention | Example |
|-------|-----------|---------|
| System functions | `snake_case` + `_system` suffix | `brain_system` |
| Component structs | `PascalCase` | `PhysicalGenome` |
| Resource structs | `PascalCase` | `TileGrid` |
| Constants | `SCREAMING_SNAKE_CASE` | `BASE_MOVE_COST` |
| Event structs | `PascalCase` + `Event` suffix | `EntityDiedEvent` |
| Genome gene fields | `snake_case` matching spec name | `cellular_decay` |
