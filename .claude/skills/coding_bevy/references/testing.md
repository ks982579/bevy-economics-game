# Bevy 0.18 Testing Patterns

## Minimal app setup

```rust
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<ColorMaterial>>()
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<ButtonInput<MouseButton>>();
    app
}
```

- `MinimalPlugins` gives you the scheduler and time — no window, no renderer
- Always `init_resource` for every `Assets<T>` your systems touch
- Always `init_resource::<ButtonInput<KeyCode>>()` if testing input systems

---

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

---

## Querying entities in tests

```rust
let mut q = app.world_mut().query_filtered::<&Transform, With<Player>>();
let translation = q.single(app.world()).unwrap().translation;
assert!(translation.x > 0.0);
```

For mutable access:

```rust
let mut q = app.world_mut().query_filtered::<&mut Transform, With<Player>>();
let mut transform = q.single_mut(app.world_mut()).unwrap();
transform.translation.x = 50.0;
```

---

## Simulating keyboard input

```rust
// Press a key
app.world_mut()
    .resource_mut::<ButtonInput<KeyCode>>()
    .press(KeyCode::KeyD);

app.update(); // runs systems with that input active

// The key is still "pressed" — simulate release
app.world_mut()
    .resource_mut::<ButtonInput<KeyCode>>()
    .release(KeyCode::KeyD);

app.update(); // first update after release: just_released() is true
app.update(); // second update: neither pressed nor just_released
```

---

## Testing game states

`MinimalPlugins` does NOT include state support. Add `bevy::state::app::StatesPlugin` explicitly:

```rust
fn test_app_with_states() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
       .init_resource::<ButtonInput<KeyCode>>();
    app
}
```

State transitions scheduled with `NextState::set()` take effect at the **end** of the frame —
you need **two** `app.update()` calls to observe the new state:

```rust
fn test_state_transition() {
    let mut app = test_app_with_states();
    app.init_state::<GameState>()
       .add_systems(OnEnter(GameState::Playing), spawn_player)
       .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));

    // Transition to Playing
    app.world_mut()
       .resource_mut::<NextState<GameState>>()
       .set(GameState::Playing);

    app.update(); // transition applied, OnEnter(Playing) fires
    app.update(); // first Update in Playing state
}

// When testing that a system CAUSES a transition:
fn test_system_triggers_transition() {
    // ... spawn entities, set state ...
    app.update(); // system runs, calls next_state.set(...)
    app.update(); // transition now applied — assert new state here
    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::Playing);
}
```

---

## Full pattern: spawn → act → assert

```rust
#[test]
fn player_moves_right_when_d_pressed() {
    let mut app = test_app();
    app.add_systems(Update, move_player);

    // 1. spawn
    let _ = app.world_mut().run_system_once(
        |mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>| {
            spawn_player(&mut commands, &mut meshes, &mut materials);
        },
    );
    app.update(); // flush spawn

    // 2. act
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyD);
    app.update();

    // 3. assert
    let mut q = app.world_mut().query_filtered::<&Transform, With<Player>>();
    let x = q.single(app.world()).unwrap().translation.x;
    assert!(x > 0.0, "Player should have moved right, got x={x}");
}
```

---

## Testing with resources

```rust
#[test]
fn score_increments_on_pickup() {
    let mut app = test_app();
    app.insert_resource(Score(0));
    app.add_systems(Update, collect_pickup);

    // spawn a pickup near origin
    app.world_mut().spawn(Pickup);
    app.update();

    let score = app.world().resource::<Score>();
    assert_eq!(score.0, 10);
}
```

---

## Checking entity count

```rust
let count = app.world_mut()
    .query_filtered::<Entity, With<Enemy>>()
    .iter(app.world())
    .count();
assert_eq!(count, 5);
```
