#![allow(dead_code)]

/// # Simulation Constants
///
/// **Single source of truth for all numeric values defined in the spec.**
///
/// Rules:
/// - Never hardcode a spec value anywhere else in the codebase.
/// - If the spec changes, change it here — one edit propagates everywhere.
/// - All values are annotated with their spec section reference.
/// - Derived constants (computed from primitives) are marked with `// derived`.

// ── World Geometry ─────────────────────────────────────────────────────────── §2

pub const GRID_WIDTH: u32  = 1024;
pub const GRID_HEIGHT: u32 = 1024;
pub const GRID_SIZE: usize = (GRID_WIDTH * GRID_HEIGHT) as usize; // derived

pub const CHUNK_SIZE: u32        = 16;   // tiles per chunk side
pub const CHUNKS_PER_ROW: u32    = GRID_WIDTH / CHUNK_SIZE;       // derived: 64
pub const TOTAL_CHUNKS: usize    = (CHUNKS_PER_ROW * CHUNKS_PER_ROW) as usize; // derived: 4096

// ── Biome Plant Regen (biomass/tick baseline) ──────────────────────────────── §2

pub const BASE_REGEN_FOREST:  f32 = 0.008;
pub const BASE_REGEN_PLAINS:  f32 = 0.005;
pub const BASE_REGEN_DESERT:  f32 = 0.001;
pub const BASE_REGEN_WATER:   f32 = 0.002;
pub const BASE_REGEN_MOUNTAIN: f32 = 0.0;

pub const MAX_BIOMASS: f32 = 10.0;

/// Chance per tick that a full tile spreads biomass to an adjacent empty neighbour. §2
pub const PLANT_SPREAD_CHANCE: f32 = 0.10;
/// Biomass units copied during a spread event. §2
pub const PLANT_SPREAD_AMOUNT: f32 = 2.0;
/// Biomass threshold below which a tile is considered "empty" for spread. §2
pub const PLANT_SPREAD_TARGET_THRESHOLD: f32 = 1.0;

// ── Tile Nutrient Cycle ────────────────────────────────────────────────────── §2, §7

/// Meat units added to a tile on entity death, per unit of entity Size. §7
pub const CORPSE_MEAT_PER_SIZE: f32 = 5.0;
/// Meat decayed per tick; equal amount added to soil_nutrients. §7
pub const CORPSE_DECAY_RATE: f32 = 0.01;
/// Passive soil nutrient decay per tick (prevents permanent saturation). §2
pub const SOIL_NUTRIENT_DECAY: f32 = 0.0001;

// ── Day / Night Cycle ──────────────────────────────────────────────────────── §2

/// Duration of one full day/night cycle in ticks. §2
pub const DAY_NIGHT_PERIOD_TICKS: u32 = 300;
/// Fraction by which Sensory_Radius is reduced at night. §2
pub const NIGHT_VISION_PENALTY: f32 = 0.30;

// ── Season Cycle ──────────────────────────────────────────────────────────────  §2

/// Ticks per season (one full year = 4× this). §2
pub const SEASON_DURATION_TICKS: u32 = 3_000;

pub const SEASON_TEMP_SPRING: f32 = 0.0;
pub const SEASON_TEMP_SUMMER: f32 = 0.2;
pub const SEASON_TEMP_AUTUMN: f32 = -0.1;
pub const SEASON_TEMP_WINTER: f32 = -0.3;

pub const SEASON_GROWTH_SPRING: f32 = 1.5;
pub const SEASON_GROWTH_SUMMER: f32 = 1.0;
pub const SEASON_GROWTH_AUTUMN: f32 = 0.6;
pub const SEASON_GROWTH_WINTER: f32 = 0.1;

// ── Hard Wall Traversal Thresholds ────────────────────────────────────────── §2

pub const AQUATIC_TRAVERSAL_THRESHOLD: f32  = 0.7;
pub const MOUNTAIN_TRAVERSAL_THRESHOLD: f32 = 0.7;
/// Energy cost for attempting to enter a blocked tile. §2
pub const BLOCKED_TILE_PENALTY: f32 = 0.005;

// ── Sensory Radius ─────────────────────────────────────────────────────────── §4

/// Minimum sensory radius in tiles (gene = 0.0). §4
pub const SENSORY_RADIUS_MIN: f32 = 2.0;
/// Maximum sensory radius in tiles (gene = 1.0). §4
pub const SENSORY_RADIUS_MAX: f32 = 16.0;
/// Derived: `radius = SENSORY_RADIUS_MIN + gene * (SENSORY_RADIUS_MAX - SENSORY_RADIUS_MIN)`

// ── Energy & Metabolism ────────────────────────────────────────────────────── §6.1–6.3

/// Base energy capacity before Size scaling. §4
pub const BASE_MAX_ENERGY: f32 = 10.0;
/// Energy capacity added per unit of Size gene. §4
pub const MAX_ENERGY_SIZE_SCALE: f32 = 40.0;

/// Base energy cost per tile of movement, before biome and speed scaling. §6.2
pub const BASE_MOVE_COST: f32 = 0.002;
/// Energy cost per tick regardless of action. §6.2
pub const IDLE_COST: f32 = 0.001;

/// Biome movement cost multipliers. §6.2
pub const MOVE_COST_FOREST:   f32 = 1.5;
pub const MOVE_COST_PLAINS:   f32 = 1.0;
pub const MOVE_COST_DESERT:   f32 = 2.0;
pub const MOVE_COST_MOUNTAIN: f32 = 3.0;
pub const MOVE_COST_WATER:    f32 = 2.5;

/// Max tiles moved per action at Speed = 1.0. §6.2
pub const MAX_MOVE_TILES: u32 = 3;
/// Neural output threshold below which no movement is attempted. §6.2
pub const MOVE_OUTPUT_THRESHOLD: f32 = 0.3;

/// Biomass/meat consumed per eating action. §6.1
pub const EAT_RATE: f32 = 0.5;
/// Energy per biomass unit at Plant_Digestion = 1.0. §6.1
pub const PLANT_ENERGY_SCALE: f32 = 0.30;
/// Energy per meat unit at Meat_Digestion = 1.0. §6.1
pub const MEAT_ENERGY_SCALE: f32 = 0.50;
/// Neural output threshold for triggering eat action. §6.1
pub const EAT_OUTPUT_THRESHOLD: f32 = 0.5;

/// Thermal delta above which energy penalty is applied. §6.3
pub const THERMAL_COMFORT_THRESHOLD: f32 = 0.3;
/// Energy penalty per tick per unit of thermal delta above threshold. §6.3
pub const THERMAL_PENALTY_SCALE: f32 = 0.003;
/// Stress increment per unit of thermal delta per tick. §6.3
pub const THERMAL_STRESS_SCALE: f32 = 0.01;

// ── Passive Health Regen ───────────────────────────────────────────────────── §6.5

pub const BASE_HEALTH_REGEN: f32 = 0.0005;
/// Energy level below which health regen halts entirely. §6.5
pub const HEALTH_REGEN_MIN_ENERGY: f32 = 0.1;

// ── Combat ─────────────────────────────────────────────────────────────────── §6.4

/// Neural output threshold for initiating combat when co-located. §6.4
pub const AGGRESSION_OUTPUT_THRESHOLD: f32 = 0.5;
/// Number of ticks a combat engagement lasts. §6.4
pub const COMBAT_DURATION_TICKS: u8 = 5;
/// Scales the raw combat roll into health damage per tick. §6.4
pub const COMBAT_DAMAGE_SCALE: f32 = 0.08;
/// Min and max of the attacker's random roll multiplier. §6.4
pub const COMBAT_ROLL_MIN: f32 = 0.6;
pub const COMBAT_ROLL_MAX: f32 = 1.0;
/// Max of the defender's random dodge multiplier. §6.4
pub const COMBAT_DODGE_MAX: f32 = 0.4;
/// Health fraction below which a permanent injury roll is triggered. §6.4
pub const INJURY_HEALTH_THRESHOLD: f32 = 0.30;
/// Permanent speed_modifier reduction on injury (rolls 1–2 on d6). §6.4
pub const INJURY_SPEED_PENALTY: f32 = 0.15;
/// Permanent Sensory_Radius gene reduction on injury (rolls 3–4 on d6). §6.4
pub const INJURY_SENSORY_PENALTY: f32 = 0.10;
/// Minimum Sensory_Radius gene value after repeated injury. §6.4
pub const SENSORY_RADIUS_FLOOR: f32 = 0.10;

// ── Aging & Decay ─────────────────────────────────────────────────────────── §6.5

/// Maximum possible lifespan in ticks (for Cellular_Decay onset calculation). §6.5
pub const MAX_LIFESPAN_TICKS: u32 = 50_000;
/// Per-tick degradation rate multiplier for speed_modifier (× Cellular_Decay gene). §6.5
pub const DECAY_RATE_SPEED: f32 = 0.000_02;
/// Per-tick degradation rate multiplier for health_regen_modifier. §6.5
pub const DECAY_RATE_HEALTH_REGEN: f32 = 0.000_01; // half of speed decay rate
/// Minimum value for runtime degradation modifiers (speed, health regen). §6.5
pub const MODIFIER_FLOOR: f32 = 0.1;

// ── Reproduction ───────────────────────────────────────────────────────────── §8.1

/// Genetic similarity threshold to enable sexual reproduction. §8.1
pub const REPRODUCTION_SIMILARITY_THRESHOLD: f32 = 0.85;
/// Fraction of max_energy spent by initiating parent to create offspring. §8.1
pub const REPRODUCTION_COST: f32 = 0.40;
/// Energy multiplier for asexual reproduction threshold (higher bar). §8.1
pub const ASEXUAL_REPRODUCTION_ENERGY_MULTIPLIER: f32 = 1.5;

// ── Mutation ──────────────────────────────────────────────────────────────── §8.3

pub const MUTATION_BASE_RATE: f32 = 0.005;
pub const MUTATION_MAX_RATE: f32  = 0.15;
/// Gaussian std dev for mutation perturbation. §8.3
pub const MUTATION_STD_DEV: f32   = 0.05;

// ── Genesis / Primordial Soup ─────────────────────────────────────────────── §10

pub const INITIAL_POPULATION: u32   = 500;
/// Side length of the central spawn zone in tiles. §10
pub const PRIMORDIAL_ZONE_SIZE: u32 = 128;
/// Top-left corner of spawn zone (centred on 1024×1024 grid). §10 — derived
pub const PRIMORDIAL_ZONE_ORIGIN: u32 = (GRID_WIDTH - PRIMORDIAL_ZONE_SIZE) / 2; // = 448

pub const INITIAL_BIOMASS_FOREST:  f32 = 5.0;
pub const INITIAL_BIOMASS_PLAINS:  f32 = 3.0;
pub const INITIAL_BIOMASS_DESERT:  f32 = 1.0;
pub const INITIAL_BIOMASS_WATER:   f32 = 1.0;
pub const INITIAL_BIOMASS_MOUNTAIN: f32 = 0.0;

pub const INITIAL_SOIL_NUTRIENTS: f32     = 0.2;
pub const INITIAL_ENTITY_ENERGY_FRAC: f32 = 0.5;  // fraction of max_energy at spawn
pub const BRAIN_INIT_WEIGHT_RANGE: f32    = 0.3;   // uniform(-range, range)

// ── Neural Network Shape ──────────────────────────────────────────────────── §5

pub const NN_INPUT_SIZE:  usize = 11;
pub const NN_HIDDEN_SIZE: usize = 6;
pub const NN_OUTPUT_SIZE: usize = 5;

/// Total weight count per brain: W1 + B1 + W2 + B2. §5 — derived
pub const NN_TOTAL_WEIGHTS: usize =
    (NN_HIDDEN_SIZE * NN_INPUT_SIZE)   // W1: 66
    + NN_HIDDEN_SIZE                   // B1: 6
    + (NN_OUTPUT_SIZE * NN_HIDDEN_SIZE) // W2: 30
    + NN_OUTPUT_SIZE;                  // B2: 5  → total: 107

// ── Observer / UI ─────────────────────────────────────────────────────────── §9

/// How many ticks between K-Means speciation census runs. §9
pub const CENSUS_INTERVAL_TICKS: u32 = 500;
/// Minimum cluster population to flag a speciation event. §9
pub const SPECIATION_MIN_POPULATION: usize = 20;
/// Minimum centroid distance from all existing clusters to flag speciation. §9
pub const SPECIATION_MIN_DISTANCE: f32 = 0.3;
/// Maximum number of K-Means clusters. §9
pub const CENSUS_MAX_K: usize = 12;
/// Chronicle overgrazing threshold (fraction of theoretical max biomass). §9
pub const OVERGRAZING_BIOMASS_FRACTION: f32 = 0.05;
