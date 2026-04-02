# LifeGen — A Life Simulator (Rust + Bevy ECS)

A data-oriented, 2D grid-based artificial life simulator built with Bevy ECS. Entities are driven by neural networks, living in a tile world where plants are tile biomass and all behavior is expressed through ordered ECS systems.

## 🧩 Project goals

- 1024x1024 world represented as a linear tile array
- stateless, system-based entity behavior with pure ECS queries
- neural network control from genome (`W1`, `W2`, `B1`, `B2`) with fixed sizes
- life cycle including move, eat, combat, reproduce, age, stress, and die
- deterministic and testable simulation stages from specifications

## ⚙️ Prerequisites

- Rust toolchain (1.70+ recommended)
- `cargo` available in PATH
- Optional: `rustfmt`, `clippy` for formatting and linting

## ▶️ Run (development)

From project root:

```bash
cargo run --release
```

> If a main runner exists, this starts the simulation loop and prints periodic status. If not, run the test suite and develop systems first.

## 🧪 Test

- Unit + integration tests:

```bash
cargo test --all-targets
```

- Run only systems tests (per your existing tasks):

```bash
cargo test --systems:: --nocapture
```

- Always use constants from `crate::constants` in tests.

## 🧹 Lint and format

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

## 📁 Source layout

- `Cargo.toml` - crate metadata and dependencies
- `src/` - Rust source code
  - `main.rs` - app entrypoint and schedule setup
  - `constants.rs` - all simulation constants (no magic numbers)
  - `systems/` - ECS systems, one module per feature
- `spec/alife_spec_v2.md` - canonical functional spec; source of truth

## 🧠 System execution order (spec)

1. input_system
2. global_cycle_system
3. tile_update_system
4. spatial_update_system
5. brain_system
6. movement_system
7. eat_system
8. combat_system
9. collision_system (exclusive)
10. reproduction_system (exclusive)
11. mortality_system (exclusive)
12. aging_system
13. stress_system
14. chronicle_system
15. render_system
16. census_system (every 500 ticks)

## 🛠️ Contribution workflow

1. Fork repository and create a new branch:

```bash
git checkout -b feature/<name>
```

2. Implement one system / feature at a time, with focused tests.
3. Keep each pull request small and spec-aligned, with one dedicated branch per feature.
4. Run tests and clippy before commit:

```bash
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

5. Submit PR with:
   - Summary of behavior implemented
   - Spec references (from `spec/alife_spec_v2.md`)
   - Tests added
   - Mention this as a dedicated feature branch for review/merge strategy

6. Merge through GitHub PR after thorough review.

## � Commit strategy (mandatory)

- Each isolated feature or bug fix must be one dedicated commit at minimum.
- Each such commit must include:
  - code changes for feature/bug
  - corresponding tests (happy path + boundary/edge cases)
  - documentation updates (README, inline comments, or spec mapping)
- Use clear commit message format:
  - `feat(system): implement <brief description>`
  - `fix(system): correct <brief description>`
  - `test(system): add <brief description>`
  - `docs: update <section> for <system>`

## �🧪 Testing expectations

- Each system should include a `#[cfg(test)]` module in the same file.
- Minimal config: `App::new()` with `MinimalPlugins`.
- Naming convention: `test_<system_name>_<scenario>`.
- Include both happy path and edge case tests.

## 🤝 Code conventions

- No OOP; everything in systems
- Explicit `Commands` for spawn/despawn
- No `unwrap()` in system logic
- Include `crate::constants::*` and import numeric constants by name
- `CombatState` is temporary; clear before end-of-tick

## 📜 License

Published under the MIT License (or adapt to your project policy).
