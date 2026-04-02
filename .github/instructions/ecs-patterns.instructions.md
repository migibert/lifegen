---
applyTo: "src/systems/**/*.rs"
---

# Skill: Bevy ECS Patterns

## System signatures

Always prefer the most specific query possible. Never query more components than the
system actually reads or writes.

```rust
// ✅ Correct — reads Position, mutates VitalSigns, no extras
fn movement_system(
    mut query: Query<(&Position, &mut VitalSigns, &PhysicalGenome)>,
    tile_grid: Res<TileGrid>,
) { ... }

// ❌ Wrong — querying the whole entity, or With<> filters missing
fn movement_system(query: Query<Entity>) { ... }
```

## Mutation rules

- Use `&Component` for read-only access, `&mut Component` for writes.
- Never take `&mut` on a component you only read — it prevents parallelism.
- Split queries if you need to read component A and write component B on the same entity:
  use `Query<(&A, &mut B)>`, not two separate queries.

## Structural changes (spawn / despawn / add / remove component)

Never mutate world structure inside a parallel system. Always use `Commands`:

```rust
// ✅ Correct
fn mortality_system(
    query: Query<(Entity, &VitalSigns, &Position)>,
    mut commands: Commands,
    mut tile_grid: ResMut<TileGrid>,
) {
    for (entity, vitals, pos) in &query {
        if vitals.energy <= 0.0 || vitals.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// ❌ Wrong — direct world mutation inside a parallel system
fn mortality_system(world: &mut World) { ... } // use exclusive system instead
```

## Exclusive systems

Only use `fn my_system(world: &mut World)` when you genuinely need full World access
(e.g. spawning entities while also iterating others). Mark the intent with a comment:

```rust
/// EXCLUSIVE SYSTEM — spawns offspring, requires full World access.
/// Scheduled after collision_system. See §11 execution order.
fn reproduction_system(world: &mut World) { ... }
```

## Resource access

- `Res<T>` for read-only shared resources (TileGrid reads, GlobalClock reads).
- `ResMut<T>` for mutable resources (SpatialHashGrid updates, TileGrid writes).
- Never take `ResMut<>` if the system only reads — it serialises unnecessarily.

## Filters

Use query filters to narrow iteration, never filter inside the loop body:

```rust
// ✅ Correct — ECS filters the set before iteration
Query<(&mut VitalSigns, &Position), With<CombatState>>

// ❌ Wrong — wastes iteration over all entities
for (vitals, pos, maybe_combat) in &query {
    if maybe_combat.is_some() { ... }
}
```

## Event pattern for cross-system communication

Use Bevy `EventWriter<T>` / `EventReader<T>` for signals between systems
(e.g. death events to Chronicle). Never use a global `Vec` or `Mutex`.

```rust
#[derive(Event)]
struct EntityDiedEvent { entity: Entity, cause: DeathCause }

// Writer side (mortality_system)
fn mortality_system(..., mut death_events: EventWriter<EntityDiedEvent>) {
    death_events.send(EntityDiedEvent { entity, cause: DeathCause::Starvation });
}

// Reader side (chronicle_system)
fn chronicle_system(mut events: EventReader<EntityDiedEvent>) {
    for ev in events.read() { ... }
}
```
