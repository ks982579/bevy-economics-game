# Bevy 0.18 Testing Patterns

## Minimal app setup

```rust
use bevy::ecs::system::RunSystemOnce;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<ColorMaterial>>()
        .init_resource::<ButtonInput<KeyCode>>();
    app
}
```

- `MinimalPlugins` gives you the scheduler and time — no window, no renderer
- Always `init_resource` for any `Assets<T>` your systems touch
- Always `init_resource::<ButtonInput<KeyCode>>()` if testing input systems

## Running a one-shot system

```rust
let _ = app.world_mut().run_system_once(
    |mut commands: Commands,
     mut meshes: ResMut<Assets<Mesh>>,
     mut materials: ResMut<Assets<ColorMaterial>>| {
        spawn_player(&mut commands, &mut meshes, &mut materials);
    },
);
app.update(); // flush commands
```

- `run_system_once` returns `Result` — assign to `let _` to suppress the warning
- Always call `app.update()` after to flush spawned entities into the world

## Querying entities in tests

```rust
let mut q = app.world_mut().query_filtered::<&Transform, With<Player>>();
let val = q.single(app.world()).unwrap().translation.x;
```

## Simulating input

```rust
app.world_mut()
    .resource_mut::<ButtonInput<KeyCode>>()
    .press(KeyCode::KeyD);

app.update(); // runs systems with that input active
```

## General pattern: spawn → update → assert

```rust
#[test]
fn example_test() {
    let mut app = test_app();
    app.add_systems(Update, my_system);

    // 1. spawn
    let _ = app.world_mut().run_system_once(...);
    app.update();

    // 2. act
    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::KeyW);
    app.update();

    // 3. assert
    let mut q = app.world_mut().query_filtered::<&Transform, With<Player>>();
    assert!(q.single(app.world()).unwrap().translation.y > 0.0);
}
```
