# Bevy 0.18 ECS Patterns

## Components

Components are plain Rust structs or enums that derive `Component`.

```rust
#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Player;   // marker component — zero-size, used as a filter tag
```

### Required Components (`#[require]`)

Declare components that are automatically inserted when this component is spawned.
The required component must implement `Default` (or you provide a constructor).

```rust
#[derive(Component, Default)]
#[require(Transform, Visibility, Health)]
struct Player;

// Spawning Player also inserts Transform::default(), Visibility::default(), Health::default()
commands.spawn(Player);

// Override a required component by including it in the spawn tuple
commands.spawn((Player, Transform::from_xyz(100.0, 0.0, 3.0)));
```

Required components replace the old Bundle pattern. Do not create new Bundles.

---

## Resources

Resources are global singletons not attached to any entity.

```rust
#[derive(Resource)]
struct Score(u32);

#[derive(Resource, Default)]
struct GameConfig {
    player_speed: f32,
    enemy_count: u32,
}

// Insert at startup
app.insert_resource(Score(0));
app.init_resource::<GameConfig>();   // uses Default::default()

// Access in systems
fn update_score(mut score: ResMut<Score>) {
    score.0 += 10;
}

fn read_config(config: Res<GameConfig>) {
    info!("speed = {}", config.player_speed);
}
```

`Res<T>` is read-only, `ResMut<T>` is mutable. Missing a required resource panics at startup.

---

## Queries

Queries iterate over entities that have a matching set of components.

```rust
// Read-only
fn read_positions(query: Query<&Transform>) {
    for transform in &query {
        info!("{:?}", transform.translation);
    }
}

// Mutable
fn move_players(mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut query {
        transform.translation.x += 1.0;
    }
}

// Multiple components
fn update_entities(query: Query<(&Transform, &Health), With<Player>>) {
    for (transform, health) in &query {
        // ...
    }
}

// Mutable + read-only mix
fn update_with_config(
    mut q: Query<(&mut Transform, &Speed)>,
    time: Res<Time>,
) {
    for (mut transform, speed) in &mut q {
        transform.translation.x += speed.0 * time.delta_secs();
    }
}
```

### Query filters

```rust
With<T>      // entity must have T
Without<T>   // entity must NOT have T
Changed<T>   // T was mutated this frame
Added<T>     // T was just inserted this frame
Or<(A, B)>  // either A or B filter matches
```

### single() — exactly one match

Always use the Result pattern:

```rust
let Ok(mut transform) = query.single_mut() else { return; };
// or in a Result-returning system:
let transform = query.single()?;
```

### Entity IDs

```rust
fn get_entity_ids(query: Query<Entity, With<Player>>) {
    for entity in &query {
        info!("entity = {:?}", entity);
    }
}
```

---

## Events

### Define and send

```rust
#[derive(Event)]
struct PlayerDied {
    position: Vec2,
}

// Register in plugin/app
app.add_event::<PlayerDied>();

// Send from a system
fn detect_death(
    query: Query<(&Transform, &Health), With<Player>>,
    mut events: EventWriter<PlayerDied>,
) {
    for (transform, health) in &query {
        if health.0 <= 0.0 {
            events.send(PlayerDied {
                position: transform.translation.truncate(),
            });
        }
    }
}

// Read in another system
fn on_player_died(mut events: EventReader<PlayerDied>) {
    for event in events.read() {
        info!("Player died at {:?}", event.position);
    }
}
```

---

## Observers (0.14+)

Observers are the modern alternative to one-shot event readers for
entity-scoped or trigger-based reactions.

```rust
#[derive(Event)]
struct Explode;

// Trigger on a specific entity
commands.entity(bomb_entity).trigger(Explode);

// Observe on a specific entity
commands.entity(bomb_entity).observe(|trigger: Trigger<Explode>| {
    info!("Bomb {:?} exploded!", trigger.entity());
});

// Global observer (watches all entities)
app.observe(|trigger: Trigger<Explode>| {
    info!("Something exploded!");
});
```

Use observers for reactions that are tightly coupled to an entity's lifecycle
(e.g., on_death, on_hit). Use EventWriter/EventReader for decoupled system communication.

---

## Commands

Commands defer structural changes (spawn, despawn, insert, remove) to end of frame.

```rust
fn spawn_enemy(mut commands: Commands) {
    commands.spawn(Enemy);
}

fn despawn_dead(
    query: Query<(Entity, &Health)>,
    mut commands: Commands,
) {
    for (entity, health) in &query {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// Insert a component onto an existing entity
commands.entity(entity).insert(Stunned { duration: 2.0 });

// Remove a component
commands.entity(entity).remove::<Stunned>();
```

`despawn()` removes the entity and all its components.
Use `despawn_recursive()` (requires `HierarchyCommandExt`) to also despawn children.

---

## Change detection

```rust
fn react_to_health_change(query: Query<&Health, Changed<Health>>) {
    for health in &query {
        info!("Health changed to {}", health.0);
    }
}
```

`Changed<T>` is true the first frame a component is inserted AND any frame it is mutated.
`Added<T>` is only true the frame the component is first inserted.
