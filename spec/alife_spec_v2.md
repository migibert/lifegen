# Specification: Rust + ECS Autonomous A-Life Simulator
## Version 2.0 — All Systems Fully Defined

---

## 1. System Overview & Constraints

- **Language:** Rust
- **Engine/Framework:** Bevy ECS (handles automatic multithreading, rendering, and game loop).
- **Math/Matrix Library:** `nalgebra` or `ndarray` (for vectorized neural network matrix multiplication).
- **Serialization:** `serde` + `serde_json` for world state snapshots. The simulation must be pausable and resumable at any tick by serializing the full ECS world state to disk.
- **Target Hardware:** Apple Silicon (M2), 16GB RAM.
- **Performance Paradigm:** Strict Data-Oriented Design (DOD) using an Entity Component System (ECS) to maximize L1/L2 cache hits and avoid object-oriented bottlenecks.

---

## 2. The Environment (The World)

- **The Grid:** A 1D flattened vector `Vec<Tile>` representing a 2D map of size **1024×1024 tiles**. Indexing is calculated via $index = y \cdot \text{GRID\_WIDTH} + x$.
- **Biomes:** Each tile has a **static, immutable** biome type (Water, Forest, Plains, Mountain, Desert) assigned at world generation and never changed at runtime. Biomes define baseline Temperature, movement energy cost, and base plant growth limits. Only `Biomass` and `Soil_Nutrients` values on a tile change over time.
- **Tile Struct Fields:**

  ```
  Tile {
    biome:          BiomeType   // static, set at genesis
    biomass:        f32         // [0.0, MAX_BIOMASS] — plant matter available
    soil_nutrients: f32         // [0.0, 1.0] — accelerates biomass regen
    corpse_meat:    f32         // [0.0, N] — meat from decayed entities
  }
  ```

- **Hard Walls:** Water and Mountain tiles cannot be entered unless an entity's `Aquatic_Adaptation` gene exceeds `0.7` (Water) or `Mountain_Traversal` gene exceeds `0.7` (Mountain). Attempting to enter a blocked tile costs `0.005` energy and the move fails.
- **Global Cycles:**
  - **Day/Night:** Cycles every 300 ticks. Night reduces all entity `Sensory_Radius` by 30% and halves solar energy input to plant growth.
  - **Seasons:** Cycle every 3,000 ticks (Spring → Summer → Autumn → Winter). Season modifies global temperature and plant growth rate multipliers:

    | Season | Temp Modifier | Plant Growth Multiplier |
    |--------|---------------|------------------------|
    | Spring | +0.0          | ×1.5                   |
    | Summer | +0.2          | ×1.0                   |
    | Autumn | −0.1          | ×0.6                   |
    | Winter | −0.3          | ×0.1                   |

- **Cellular Flora (Plants):** Plants are not entities. They are a `biomass: f32` value on the `Tile` struct.
  - Each tick, a tile regenerates biomass: `biomass += BASE_REGEN * biome_multiplier * season_multiplier * (1.0 + soil_nutrients)`, capped at `MAX_BIOMASS = 10.0`.
  - Baseline regen rates by biome: Forest `0.008`, Plains `0.005`, Desert `0.001`, Water `0.002`, Mountain `0.0`.
  - **Cellular Automata Spread:** If a tile's `biomass >= MAX_BIOMASS`, a dedicated system checks its 4 cardinal neighbours. For each neighbour with `biomass < 1.0`, there is a 10% chance per tick that `2.0` biomass units are copied to that neighbour, simulating plant spread.

- **The Nutrient Cycle:**
  - Dead entities spawn a `corpse_meat` float on their tile equal to `entity.size * 5.0`.
  - Corpse meat decays at `0.01` per tick, converting into `soil_nutrients` at a 1:1 ratio (capped at `1.0`).
  - Elevated `soil_nutrients` decays slowly at `0.0001` per tick to prevent permanent saturation.

---

## 3. Spatial Partitioning & Chunking

- **Grid Hashing:** The world is divided into **64×64 chunks of 16×16 tiles each**, tied to the maximum possible sensory radius. This avoids $O(N^2)$ collision and vision checks.
- **Tracking:** A `SpatialHashGrid` resource maintains a `HashMap<ChunkID, Vec<EntityID>>`. It is updated each tick by the `spatial_update_system` after all movement is resolved.
- **Vision Queries:** When evaluating surroundings, an entity queries only the `EntityIDs` registered in its current chunk and the 8 immediately adjacent chunks (a 3×3 chunk window), then filters to those within `Sensory_Radius` tiles.

---

## 4. The Genome Data Dictionary (ECS Components)

Entities are unique IDs possessing flat-array Components. All genome values are normalized floats between `0.0` and `1.0`.

- **Position Component:** Stores grid coordinates `(x: u32, y: u32)`.

- **PhysicalGenome Component:**

  | Gene              | Description                                                                 |
  |-------------------|-----------------------------------------------------------------------------|
  | `Size`            | Scales max energy capacity, base health, and melee damage. Range `[0.0, 1.0]`. |
  | `Speed`           | Movement tiles per action and dodge factor in combat. Higher = more tiles, higher move cost. |
  | `Sensory_Radius`  | **Heritable gene.** Base sensory radius in tiles = `2.0 + gene * 14.0`, giving a range of 2–16 tiles. Evolves independently. Can be permanently reduced by injury. |
  | `Thermal_Tolerance` | Ideal operating temperature. Deviation from current tile temp incurs energy penalties. |
  | `Aquatic_Adaptation` | Threshold `> 0.7` permits Water tile traversal.                          |

- **MetabolicGenome Component:**

  | Gene                    | Description                                                             |
  |-------------------------|-------------------------------------------------------------------------|
  | `Plant_Digestion`       | Efficiency multiplier for converting tile biomass to energy.            |
  | `Meat_Digestion`        | Efficiency multiplier for converting corpse meat to energy.             |
  | `Reproduction_Threshold`| Fraction of max energy required before reproduction is possible.        |
  | `Cellular_Decay`        | Controls the aging degradation rate. Higher = faster stat decline with age. See Section 6. |

- **VitalSigns Component:**

  | Field              | Description                                                                  |
  |--------------------|------------------------------------------------------------------------------|
  | `energy`           | Current energy. Capped at `max_energy = 10.0 + size * 40.0`. Death at `<= 0`. |
  | `health`           | Current health `[0.0, 1.0]`. Death at `<= 0`.                               |
  | `age`              | Tick counter incremented each tick.                                          |
  | `lifetime_stress`  | Cumulative stress score. Increases from starvation, temperature extremes, and combat damage. |
  | `speed_modifier`   | Runtime multiplier on `Speed` gene. Starts at `1.0`, reduced by injury/age. |
  | `health_regen_modifier` | Runtime multiplier on passive health regen. Starts at `1.0`, reduced by age. |

- **Brain Component:** Stores the flattened neural network weight arrays and `memory_state: f32`.

- **CombatState Component (temporary):** Added to an entity when it enters combat. Stores `opponent: EntityID`, `ticks_remaining: u8`. Removed when combat concludes. An entity with this component cannot move.

---

## 5. Neural Network Matrix Topology & Math

The `brain_system` relies entirely on matrix multiplication via `nalgebra` with no per-neuron loops.

### Input Vector ($I$): 11×1

| Index | Sensor                          | Range     | Edge Case (nothing in range)  |
|-------|---------------------------------|-----------|-------------------------------|
| 0     | Distance to nearest food source | `[0, 1]`  | `0.0` (no food detected)      |
| 1     | Bearing to nearest food source  | `[0, 1]`  | `0.5` (neutral/no direction)  |
| 2     | Biomass on current tile         | `[0, 1]`  | `0.0`                         |
| 3     | Distance to nearest entity      | `[0, 1]`  | `0.0` (no entity detected)    |
| 4     | Bearing to nearest entity       | `[0, 1]`  | `0.5` (neutral/no direction)  |
| 5     | Genetic similarity to nearest entity | `[0, 1]` | `0.0` (no entity / alien)  |
| 6     | Aggression output of nearest entity | `[0, 1]` | `0.0` (no entity / peaceful) |
| 7     | Current energy (normalized)     | `[0, 1]`  | Intrinsic — always valid      |
| 8     | Current health (normalized)     | `[0, 1]`  | Intrinsic — always valid      |
| 9     | Thermal delta (normalized)      | `[0, 1]`  | `0.0` = perfectly comfortable |
| 10    | Memory In (previous Memory Out) | `[-1, 1]` | `0.0` on first tick           |

**Notes on sensor computation:**
- "Nearest food source" is the closer of: (a) the highest-biomass tile in sensory range, (b) the tile with the highest `corpse_meat` in range. Distance is normalized to `Sensory_Radius`.
- Bearing is encoded as a single float via `angle / (2π)`, giving `[0, 1]`.
- **Genetic similarity** is computed as `1.0 - hamming_distance(self_genome_hash, other_genome_hash)`. This gives `1.0` for a clone, `~0.5` for a distant relative, near `0.0` for an unrelated entity. This is the mechanism that enables kin detection and emergent cannibalism.
- **Nearest entity aggression** reads the opponent's last `Output[2]` cached on their `Brain` component. This allows pre-emptive flee/attack decisions before a fight starts.

### Hidden Vector ($H$): 6×1

Scaled up from 4 to support the richer input space.

### Output Vector ($O$): 5×1

| Index | Output          | Range    | Interpretation                                             |
|-------|-----------------|----------|------------------------------------------------------------|
| 0     | Move X          | `[-1, 1]`| Negative = West, Positive = East. Magnitude scales with `Speed` gene. |
| 1     | Move Y          | `[-1, 1]`| Negative = North, Positive = South.                        |
| 2     | Aggression      | `[-1, 1]`| `> 0.5` initiates combat with nearest entity if co-located.|
| 3     | Eat Impulse     | `[-1, 1]`| `> 0.5` triggers consumption of biomass/meat on current tile. |
| 4     | Memory Out      | `[-1, 1]`| Stored as `memory_state` and fed back as Input[10] next tick.|

### Weight Matrices

| Matrix | Dimensions | Description                   |
|--------|------------|-------------------------------|
| $W_1$  | $6 \times 11$ | Input to Hidden            |
| $W_2$  | $5 \times 6$  | Hidden to Output           |
| $B_1$  | $6 \times 1$  | Hidden bias vector         |
| $B_2$  | $5 \times 1$  | Output bias vector         |

**Total weight count:** $(6 \times 11) + 6 + (5 \times 6) + 5 = 66 + 6 + 30 + 5 = \mathbf{107}$ floats per entity.

### Forward Pass Equations

$$H = \tanh(W_1 \cdot I + B_1)$$
$$O = \tanh(W_2 \cdot H + B_2)$$

The $\tanh$ activation squishes all outputs to $[-1.0, 1.0]$.

---

## 6. Interaction Mechanics

### 6.1 Eating

Triggered when `Output[3] > 0.5` and the entity is standing on a tile with `biomass > 0` or `corpse_meat > 0`. The system preferentially eats meat if both are available and `Meat_Digestion > Plant_Digestion`; otherwise eats biomass.

- **Eat amount per action:** `EAT_RATE = 0.5` biomass/meat units consumed per tick.
- **Energy gained from plants:** `EAT_RATE * Plant_Digestion * 0.3` energy. At `Plant_Digestion = 1.0` this yields `0.15` energy per tick.
- **Energy gained from meat:** `EAT_RATE * Meat_Digestion * 0.5` energy. At `Meat_Digestion = 1.0` this yields `0.25` energy per tick.

### 6.2 Movement

Each tick, `Output[0]` and `Output[1]` are thresholded. If `|Move_X| > 0.3` or `|Move_Y| > 0.3`, the entity attempts to move. The movement direction is discretized to the 8 compass directions.

**Movement cost:** `BASE_MOVE_COST = 0.002` energy per tile traversed, scaled by:
- Biome modifier: Forest `×1.5`, Plains `×1.0`, Desert `×2.0`, Mountain `×3.0` (if traversable).
- Speed gene: faster entities cover more distance but pay proportionally more energy per action. Specifically, an entity with `Speed = 1.0` moves up to 3 tiles per action and pays `3 × BASE_MOVE_COST × biome_modifier`.

**Idle metabolic cost:** `IDLE_COST = 0.001` energy per tick, regardless of action.

### 6.3 Thermal Regulation

Each tick, `thermal_delta = |tile_temperature - Thermal_Tolerance|`. If `thermal_delta > 0.3`, the entity incurs a stress energy penalty: `THERMAL_PENALTY = thermal_delta * 0.003` energy per tick. This also increments `lifetime_stress` by `thermal_delta * 0.01`.

### 6.4 Combat

Combat is triggered when two entities occupy the same tile and at least one of them outputs `Aggression > 0.5`.

**Initiation:** The `collision_system` detects co-location. The entity with the higher Aggression output is assigned `attacker`; the other is `defender`. Both receive a `CombatState` component locking them in place for the combat duration. Combat lasts a fixed **5 ticks**.

**Per-tick damage formula:**

```
attacker_roll  = attacker.Size * rng_uniform(0.6, 1.0)
defender_dodge = defender.Speed * rng_uniform(0.0, 0.4)
damage_dealt   = max(0.0, (attacker_roll - defender_dodge) * COMBAT_DAMAGE_SCALE)
```

Where `COMBAT_DAMAGE_SCALE = 0.08`. At max Size vs. zero dodge, this yields up to `0.08` health damage per tick, meaning a full 5-tick fight can deal up to `0.4` health damage.

The defender deals damage back to the attacker using the same formula with roles reversed.

**Combat termination:**
- After 5 ticks, or when either entity's health reaches `0.0`, the `CombatState` component is removed from both entities.
- The surviving entity gains no immediate energy reward from combat; energy comes from eating the corpse afterward.
- If both survive, they are displaced 1 tile apart and may re-engage next tick if still aggressive.

**Permanent injury check:** After any combat tick that drops an entity below `30%` health, roll a `d6`:
- 1–2: Reduce `speed_modifier` by `0.15` (permanent, cumulative).
- 3–4: Reduce `Sensory_Radius` gene by `0.1` (permanent, cumulative, floored at `0.1`).
- 5–6: No permanent injury.

### 6.5 Aging & Cellular Decay (Degradation Model)

`Cellular_Decay` controls how rapidly an entity's performance degrades as it ages. The degradation model affects two runtime multipliers on `VitalSigns`: `speed_modifier` and `health_regen_modifier`.

**Decay onset tick:** `onset = (1.0 - Cellular_Decay) * MAX_LIFESPAN_TICKS`, where `MAX_LIFESPAN_TICKS = 50_000`. A high `Cellular_Decay` gene (e.g., `0.9`) begins degradation very early (at tick ~5,000); a low value (e.g., `0.1`) delays onset until tick ~45,000.

**Per-tick degradation (applied only after onset):**

```
decay_rate         = Cellular_Decay * 0.000_02
speed_modifier    -= decay_rate
health_regen      -= decay_rate * 0.5
```

Both modifiers are floored at `0.1` (entities retain minimal function but become progressively ineffective). Death occurs when `energy <= 0` or `health <= 0` — aging itself does not kill directly, but degraded stats make starvation, thermal stress, and combat increasingly lethal.

**Passive health regen:** Each tick, an entity regenerates `BASE_HEALTH_REGEN * health_regen_modifier` health, where `BASE_HEALTH_REGEN = 0.0005`. This is halted entirely if `energy < 0.1`.

---

## 7. Death & Entity Cleanup System

The `mortality_system` runs each tick after all interaction systems. It checks every entity's `VitalSigns`.

**Death condition:** `energy <= 0.0` OR `health <= 0.0`.

**Death sequence (executed in order within a command buffer):**

1. Read `entity.position` and `entity.size`.
2. Add `corpse_meat` to the tile at that position: `tile.corpse_meat += entity.size * 5.0`.
3. Remove the entity from the `SpatialHashGrid`.
4. Despawn all of the entity's components.
5. If the entity's `age > 10_000` ticks or `lifetime_stress > 50.0`, emit a Chronicle event: `"[Tick X] Lineage member died — age: Y, stress: Z"`.
6. Decrement global population counter.

**Corpse decay (in `tile_update_system`):** Each tick on tiles with `corpse_meat > 0`:

```
decay_amount        = 0.01
tile.corpse_meat   -= decay_amount
tile.soil_nutrients = min(1.0, tile.soil_nutrients + decay_amount)
```

---

## 8. Evolution & Genetics

### 8.1 Reproduction

The `reproduction_system` runs as a **single-threaded exclusive system** (not parallelized) to eliminate race conditions from simultaneous pair detection.

**Trigger conditions:**
- Two entities occupy the same tile.
- Genetic similarity `>= 0.85` (same species).
- Both entities have `energy >= Reproduction_Threshold * max_energy`.

**Race condition prevention:** Within a single tick, the system processes pairs using a `HashSet<EntityID>` of "already-reproducing" entities. The pair is resolved by sorting entity IDs and processing the lower ID as the initiator. Once an entity is added to the set it cannot participate in another pairing that tick.

**Offspring instantiation:**
- A new Entity is spawned with blended genome (see Section 8.2).
- Placed on the parent tile or the nearest free tile if occupied.
- Starting energy: `REPRODUCTION_COST = 0.40` of max_energy, split from the initiating parent's energy pool.
- Starting health: `1.0`. Starting age: `0`.

**Asexual reproduction:** If an entity has no partner within sensory range and `energy >= Reproduction_Threshold * max_energy * 1.5` (higher threshold), it reproduces via splitting. The offspring receives a copy of the parent genome, subject to mutation.

### 8.2 DNA Splicing

- **PhysicalGenome & MetabolicGenome genes:** Each gene is selected uniformly — 50% chance from Parent A, 50% from Parent B.
- **Neural Network weights ($W_1$, $W_2$, $B_1$, $B_2$):** All weights are averaged between both parents (`weight = (A + B) / 2.0`). This prevents non-functional brains from random weight splicing.
- **Memory state:** Initialized to `0.0` in all offspring.

### 8.3 Stress-Induced Mutation

Mutation is applied gene-by-gene to the offspring at birth.

**Mutation rate calculation:**

```
base_rate    = 0.005
stress_bonus = clamp(parent.lifetime_stress / 100.0, 0.0, 1.0) * 0.145
mutation_rate = base_rate + stress_bonus
```

This gives `0.5%` baseline, scaling up to `15%` under maximum stress.

**Mutation application:** For each gene float and each weight float, if `rng() < mutation_rate`, the value is perturbed by `rng_gaussian(mean=0.0, std=0.05)`, then clamped to `[-1.0, 1.0]` for weights and `[0.0, 1.0]` for genome genes.

---

## 9. Observer Interface (The God-View)

The UI is strictly observational, built using Bevy's `egui` integration.

- **Visual Speciation:** Entity sprites are colored dynamically based on their genome. Red Channel maps to `Meat_Digestion`, Green Channel maps to `Plant_Digestion`, Blue Channel maps to `Speed + Aquatic_Adaptation`. Sprite radius maps to `Size`.

- **Global Telemetry Panel:** Displays the current Epoch/Tick, Global Season, Global Temperature, Total Biomass (summed across all tiles), and Total Population.

- **Species Ledger Panel:** Clusters entities via K-Means on the **6 physical and metabolic genome genes only** (not neural weights). K-Means runs every **500 ticks** (a "census tick"), not every frame. K is initialized at `6` and updated using a simple gap heuristic capped at `12`. Displays dominant lineages, population counts, and radar charts of their average traits. A "speciation event" is flagged in the Chronicle when a new cluster exceeds `20` members and has a centroid distance `> 0.3` from all existing cluster centroids.

- **The Chronicle Panel:** A scrolling text log pushing notifications for emergent milestones: extinctions, speciation events, global overgrazing (`total_biomass < 5% of theoretical max`), population bottlenecks, and notable individual deaths.

- **Viewport Filters:** Toggles between Standard Vision, Nutrient Heatmap (tile `soil_nutrients`), Biomass Heatmap, and Danger Heatmap (clusters of entities with last `Aggression > 0.5`).

- **Snapshot Controls:** A "Save State" button serializes the full ECS world to `snapshot_tick_N.json` via `serde`. A "Load State" button restores it. Snapshots include the full `SpatialHashGrid`, tile data, all entity components, and the current tick/season/cycle state.

---

## 10. Genesis Phase (Initialization)

**Year 0 sequence:**

1. Allocate `Vec<Tile>` of size `1024 × 1024 = 1,048,576` tiles.
2. Assign biomes via layered Perlin noise. Water bodies cluster near map edges; Plains occupy the central region; Forest, Desert, and Mountain patches are scattered via secondary noise octaves.
3. Seed baseline `biomass` values: Plains `3.0`, Forest `5.0`, Desert `1.0`, Water `1.0`, Mountain `0.0`.
4. Seed baseline `soil_nutrients`: uniformly `0.2` across all non-water tiles.

**The Primordial Soup:** Spawn **500 entities** with all starting positions restricted to a **128×128 central zone** of the map (tiles `[448..576, 448..576]`). This ensures entities can find each other and interact immediately, seeding early selection pressure.

Each entity is initialized with:
- All `PhysicalGenome` and `MetabolicGenome` genes set to `0.5` (neutral baseline).
- `Brain` component weight arrays filled with `rng_uniform(-0.3, 0.3)` (small random values to break symmetry without wild initial behavior).
- `energy` set to `50%` of `max_energy`. `health = 1.0`. `age = 0`.
- `memory_state = 0.0`.

---

## 11. System Execution Order (Bevy Schedule)

Each tick executes the following systems in order. Systems within a stage may run in parallel unless marked `[exclusive]`.

```
1. input_system                  — read egui snapshot/load events
2. global_cycle_system           — advance day/night, season, temperature
3. tile_update_system            — biomass regen, plant spread, corpse decay
4. spatial_update_system         — rebuild SpatialHashGrid from current positions
5. brain_system                  — run neural net forward pass for all entities (parallel)
6. movement_system               — resolve movement intents, apply energy costs (parallel)
7. eat_system                    — resolve eat intents on current tile (parallel)
8. combat_system                 — resolve combat ticks for all CombatState entities (parallel)
9. collision_system              — detect co-location, initiate new CombatState (exclusive)
10. reproduction_system          — detect reproduction pairs, spawn offspring (exclusive)
11. mortality_system             — detect deaths, spawn corpses, despawn entities (exclusive)
12. aging_system                 — increment age, apply degradation model (parallel)
13. stress_system                — update lifetime_stress accumulators (parallel)
14. chronicle_system             — emit milestone events to the log
15. render_system                — update sprites, UI panels, heatmaps
16. [every 500 ticks] census_system — run K-Means speciation clustering
```

---
