# Bevy 0.18 Game States

## Defining a state type

```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}
```

All five derives are required: `States`, `Default`, `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`.
The `#[default]` attribute marks the initial state.

---

## Registering state in the app

```rust
app.init_state::<GameState>()
```

This replaces the older `add_state::<GameState>()` API.

---

## Reacting to state transitions

### OnEnter — runs once when entering a state

```rust
app.add_systems(OnEnter(GameState::Playing), (
    spawn_player,
    spawn_level,
    reset_score,
));
```

### OnExit — runs once when leaving a state

```rust
app.add_systems(OnExit(GameState::Playing), cleanup_playing_entities);
```

### Update systems scoped to a state

```rust
app.add_systems(Update,
    (handle_input, move_player, check_collision)
        .run_if(in_state(GameState::Playing))
);

app.add_systems(Update,
    update_menu.run_if(in_state(GameState::MainMenu))
);
```

### Full pattern example

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .init_state::<GameState>()
    // --- Main Menu ---
    .add_systems(OnEnter(GameState::MainMenu),  spawn_menu)
    .add_systems(Update, menu_input.run_if(in_state(GameState::MainMenu)))
    .add_systems(OnExit(GameState::MainMenu),   despawn_menu)
    // --- Playing ---
    .add_systems(OnEnter(GameState::Playing),   (spawn_player, spawn_level))
    .add_systems(Update, (handle_input, move_player, check_collision)
        .chain()
        .run_if(in_state(GameState::Playing)))
    .add_systems(FixedUpdate, apply_velocity.run_if(in_state(GameState::Playing)))
    .add_systems(OnExit(GameState::Playing),    cleanup_level)
    // --- Paused ---
    .add_systems(OnEnter(GameState::Paused),    show_pause_overlay)
    .add_systems(Update, unpause_input.run_if(in_state(GameState::Paused)))
    .add_systems(OnExit(GameState::Paused),     hide_pause_overlay)
    .run();
```

---

## Changing state from a system

```rust
fn check_game_over(
    query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(health) = query.single() else { return; };
    if health.0 <= 0.0 {
        next_state.set(GameState::GameOver);
    }
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused  => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}
```

`NextState::set()` schedules the transition. It takes effect at the end of the frame,
triggering `OnExit` (old state) then `OnEnter` (new state) in the next frame.

---

## State-scoped entity cleanup

**`StateScoped` does NOT exist in Bevy 0.18.** Use a scene-marker component and an
`OnExit` cleanup system instead. This is the correct pattern for this project:

```rust
/// Tag every entity spawned in this scene.
#[derive(Component)]
struct PlayingEntity;

fn setup(mut commands: Commands, ...) {
    commands.spawn((Player, PlayingEntity, ...));
    commands.spawn((Enemy, PlayingEntity, ...));
}

fn cleanup(mut commands: Commands, q: Query<Entity, With<PlayingEntity>>) {
    for entity in &q {
        commands.entity(entity).despawn();
    }
}

app.add_systems(OnEnter(GameState::Playing), setup)
   .add_systems(OnExit(GameState::Playing),  cleanup);
```

For UI trees spawned with `with_children`, `despawn()` in 0.18 automatically
despawns the full hierarchy — no `despawn_recursive()` needed (it doesn't exist).

---

## Reading current state

```rust
fn debug_state(state: Res<State<GameState>>) {
    info!("Current state: {:?}", state.get());
}
```

---

## Sub-states (nested states)

For fine-grained control within a top-level state (e.g., `Playing::Combat` vs `Playing::Dialogue`):

```rust
#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(GameState = GameState::Playing)]
enum PlayingState {
    #[default]
    Normal,
    Combat,
    Dialogue,
}

app.add_sub_state::<PlayingState>();
```

Sub-states are only active when the parent state is active.
