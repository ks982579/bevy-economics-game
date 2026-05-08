# Bevy 0.18 App Structure

## Minimal 2D app (WSL2)

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)  // window, renderer, input, audio, etc.
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.8, 0.2)))),
        Transform::from_xyz(0.0, 0.0, 3.0),
    ));
}

fn update() { /* runs every frame */ }
```

For WSL2, use `default-features = false` with the `2d` feature group (see feature_flags.md).

---

## Schedules

Systems are attached to a schedule that controls when they run.

| Schedule | When it runs |
|----------|-------------|
| `Startup` | Once, before the first `Update` |
| `Update` | Every frame |
| `FixedUpdate` | At a fixed timestep (default 64 Hz) — use for physics/movement |
| `PostUpdate` | After `Update`, after transform propagation |
| `PreUpdate` | Before `Update` (input is updated here) |
| `Last` | Very end of frame |

```rust
app.add_systems(Startup, spawn_player)
   .add_systems(Update, handle_input)
   .add_systems(FixedUpdate, apply_velocity)
   .add_systems(PostUpdate, check_bounds);
```

Always use `FixedUpdate` for movement and physics so behaviour is frame-rate independent.
Access `Time<Fixed>` (not `Time`) inside `FixedUpdate` systems.

---

## Plugins

A `Plugin` is the idiomatic way to group related systems, resources, and events.

```rust
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerConfig>()
            .add_event::<PlayerDied>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (handle_input, animate_player).chain())
            .add_systems(FixedUpdate, move_player);
    }
}

// Register in main
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PlayerPlugin)
    .run();
```

### Multiple plugins

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins((PlayerPlugin, EnemyPlugin, UIPlugin))
    .run();
```

---

## System sets and ordering

Use `before` / `after` to enforce ordering within the same schedule.

```rust
app.add_systems(Update,
    handle_input.before(move_player)
);

app.add_systems(Update,
    (handle_input, move_player, check_collision).chain()  // sequential
);
```

### Named system sets

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
    Collision,
    Rendering,
}

app.configure_sets(Update, (
    GameSet::Input,
    GameSet::Movement,
    GameSet::Collision,
    GameSet::Rendering,
).chain());  // Input → Movement → Collision → Rendering

app.add_systems(Update, handle_input.in_set(GameSet::Input));
app.add_systems(Update, move_player.in_set(GameSet::Movement));
```

---

## WindowPlugin customization

```rust
App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "My Game".into(),
            resolution: (800.0, 600.0).into(),
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    .run();
```

---

## App state integration

See [states.md](states.md) for the full pattern. Quick reference:

```rust
app.init_state::<GameState>()
   .add_systems(OnEnter(GameState::Playing), spawn_level)
   .add_systems(Update, update_gameplay.run_if(in_state(GameState::Playing)))
   .add_systems(OnExit(GameState::Playing), cleanup_level);
```

---

## Accessing the world directly (avoid in systems)

Prefer `Commands` for deferred changes. Direct `World` access is for one-off
operations or tests only.

```rust
// In main (before run):
app.world_mut().spawn(Player);

// In tests (see testing.md):
app.world_mut().run_system_once(my_system);
```
