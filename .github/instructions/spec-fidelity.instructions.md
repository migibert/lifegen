---
applyTo: "src/**/*.rs"
---

# Skill: Spec Fidelity

All numeric values, formulas, and data shapes must match `spec/alife_spec_v2.md` exactly.
When in doubt, read the spec before writing code.

## Constants — zero tolerance for magic numbers

Every literal number that corresponds to a spec value is a bug. Import and use the constant:

```rust
use crate::constants::*;

// ✅ Correct
let corpse_meat = vitals.size * CORPSE_MEAT_PER_SIZE;
let onset_tick  = (1.0 - genome.cellular_decay) * MAX_LIFESPAN_TICKS as f32;

// ❌ Wrong — magic numbers that will silently diverge from the spec
let corpse_meat = vitals.size * 5.0;
let onset_tick  = (1.0 - genome.cellular_decay) * 50_000.0;
```

If a value you need is not yet in `constants.rs`, add it there first with a `§` section
reference comment, then use it. Never inline it at the call site.

## Neural network — shape is fixed, do not change without a spec update

```rust
use nalgebra::{SMatrix, SVector};
use crate::constants::{NN_INPUT_SIZE, NN_HIDDEN_SIZE, NN_OUTPUT_SIZE};

// Types must match exactly
type InputVec  = SVector<f32, 11>;   // NN_INPUT_SIZE
type HiddenVec = SVector<f32, 6>;    // NN_HIDDEN_SIZE
type OutputVec = SVector<f32, 5>;    // NN_OUTPUT_SIZE
type W1 = SMatrix<f32, 6, 11>;
type W2 = SMatrix<f32, 5, 6>;

// Forward pass — exactly as written in §5, no deviations
fn forward(brain: &Brain, input: &InputVec) -> OutputVec {
    let h: HiddenVec = (brain.w1 * input + brain.b1).map(f32::tanh);
    let o: OutputVec = (brain.w2 * h     + brain.b2).map(f32::tanh);
    o
}
```

Total weight count must remain **107 floats** (`NN_TOTAL_WEIGHTS`).

## Output vector indices — never use raw indices, use named constants

```rust
// ✅ Correct
const OUT_MOVE_X:    usize = 0;
const OUT_MOVE_Y:    usize = 1;
const OUT_AGGRESSION: usize = 2;
const OUT_EAT:       usize = 3;
const OUT_MEMORY:    usize = 4;

let aggression = output[OUT_AGGRESSION];

// ❌ Wrong — a raw index with no indication of what it means
let aggression = output[2];
```

## Input vector edge cases — must be handled before forward pass (§5)

| Input index | No-target value | Rationale |
|-------------|-----------------|-----------|
| 0 — food distance   | `0.0` | No food = no signal |
| 1 — food bearing    | `0.5` | Neutral direction |
| 3 — entity distance | `0.0` | No entity = no signal |
| 4 — entity bearing  | `0.5` | Neutral direction |
| 5 — genetic similarity | `0.0` | No entity = unrelated |
| 6 — entity aggression  | `0.0` | No entity = peaceful |

```rust
// ✅ Correct edge case handling
let food_distance = nearest_food
    .map(|f| f.distance / sensory_radius)
    .unwrap_or(0.0);
let food_bearing = nearest_food
    .map(|f| f.angle / std::f32::consts::TAU)
    .unwrap_or(0.5);
```

## Key formulas — copy these exactly, do not paraphrase

**Max energy** (§4):
```rust
let max_energy = BASE_MAX_ENERGY + genome.size * MAX_ENERGY_SIZE_SCALE;
```

**Sensory radius** (§4):
```rust
let radius_tiles = SENSORY_RADIUS_MIN + genome.sensory_radius * (SENSORY_RADIUS_MAX - SENSORY_RADIUS_MIN);
```

**Combat damage per tick** (§6.4):
```rust
let attacker_roll  = attacker.size * rng.gen_range(COMBAT_ROLL_MIN..=COMBAT_ROLL_MAX);
let defender_dodge = defender.speed * rng.gen_range(0.0..=COMBAT_DODGE_MAX);
let damage = (attacker_roll - defender_dodge).max(0.0) * COMBAT_DAMAGE_SCALE;
```

**Mutation rate** (§8.3):
```rust
let stress_bonus  = (parent.lifetime_stress / 100.0).clamp(0.0, 1.0) * (MUTATION_MAX_RATE - MUTATION_BASE_RATE);
let mutation_rate = MUTATION_BASE_RATE + stress_bonus;
```

**Aging decay onset** (§6.5):
```rust
let onset = (1.0 - genome.cellular_decay) * MAX_LIFESPAN_TICKS as f32;
if vitals.age as f32 > onset {
    vitals.speed_modifier        = (vitals.speed_modifier        - genome.cellular_decay * DECAY_RATE_SPEED).max(MODIFIER_FLOOR);
    vitals.health_regen_modifier = (vitals.health_regen_modifier - genome.cellular_decay * DECAY_RATE_HEALTH_REGEN).max(MODIFIER_FLOOR);
}
```

**Energy gained from eating** (§6.1):
```rust
let plant_gain = EAT_RATE * genome.plant_digestion * PLANT_ENERGY_SCALE;
let meat_gain  = EAT_RATE * genome.meat_digestion  * MEAT_ENERGY_SCALE;
```

**Genetic similarity** (§5 — input index 5):
```rust
let similarity = 1.0 - hamming_distance_normalised(&self_genome_hash, &other_genome_hash);
```
