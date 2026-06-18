---
name: coding_bevy
description: Bevy 0.18 coding guidance for this project. Use when writing, editing, or debugging any Bevy code — systems, components, plugins, rendering, input, states, or tests.
---

# Bevy 0.18 — Project Coding Skill

## Platform constraints (WSL2)

- Always `default-features = false` in Cargo.toml
- Never add `wayland` or `bevy_gilrs` — missing system libs in this environment
- Window backend: `bevy_winit` + `x11` only
- Use the `2d` feature group for 2D games (see [feature flags](references/feature_flags.md))

## Top API gotchas — read these before writing any code

### 1. Colored shapes use Mesh2d + ColorMaterial, NOT Sprite

`Sprite` requires a real `Handle<Image>`. For plain colored shapes, always use:

```rust
commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(width, height))),
    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(r, g, b)))),
    Transform::from_xyz(x, y, 0.0),
));
```

Requires features: `bevy_sprite`, `bevy_mesh`, `bevy_sprite_render` (or the `2d` group).

### 2. Bundles are deprecated — use Required Components

`SpriteBundle`, `NodeBundle`, `PbrBundle`, etc. are all deprecated as of 0.15. Use the
`#[require]` macro instead:

```rust
#[derive(Component)]
#[require(Transform, Visibility)]   // auto-added on spawn
struct Player;
```

Spawning `Player` automatically inserts `Transform` and `Visibility` with their defaults.
Override a required component by including it explicitly in the spawn tuple.

### 3. Query::single() returns Result — always handle it

```rust
// WRONG — panics if not exactly one match
let transform = query.single();

// CORRECT
let Ok(transform) = query.single() else { return; };
// or
let Ok(mut transform) = query.single_mut() else { return; };
```

### 4. Rectangle::new — no splat, no square shorthand

```rust
// WRONG
Rectangle::splat(32.0)

// CORRECT
Rectangle::new(32.0, 32.0)
```

### 5. Systems can return Result (0.16+)

Use `?` inside systems; errors are forwarded to the global error handler (default: panic).

```rust
fn my_system(query: Query<&Transform>) -> Result {
    let transform = query.single()?;
    // ...
    Ok(())
}
```

### 6. One persistent camera — never per-scene

Spawn `Camera2d` **once** in a `Startup` system in `main.rs`. Do not spawn or despawn cameras inside scene plugins (`OnEnter` / `OnExit`). When a scene despawns its camera and the next scene spawns a replacement, there is a one-frame gap where no camera exists — the renderer produces a grey/black frame or stalls.

```rust
// main.rs — once at startup
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
app.add_systems(Startup, spawn_camera);
```

Nothing renders without a camera. Camera sits at z = 1000 looking down -Z.

### 7. Transform z controls draw order in 2D

Higher z renders on top. Keep all 2D entities between z = 0.0 and z = 999.0.

### 8. StateScoped does NOT exist in 0.18

The `StateScoped` component was not shipped in Bevy 0.18. Do not use it. The correct
pattern is a scene-marker component + `OnExit` cleanup:

```rust
#[derive(Component)]
struct SceneEntity;   // tag every entity spawned in this scene

fn cleanup(mut commands: Commands, q: Query<Entity, With<SceneEntity>>) {
    for e in &q { commands.entity(e).despawn(); }
}

app.add_systems(OnExit(GameState::Playing), cleanup);
```

### 9. despawn_recursive is gone in 0.18

`despawn_recursive()` does not exist. Use `despawn()` — it automatically despawns
children in Bevy 0.18.

```rust
// WRONG
commands.entity(e).despawn_recursive();

// CORRECT
commands.entity(e).despawn();
```

### 10. BorderColor is a struct, not a tuple

```rust
// WRONG
BorderColor(Color::WHITE)

// CORRECT
BorderColor::all(Color::WHITE)
```

### 11. Children::iter() yields Entity, not &Entity

### 12. `bevy_ui` does NOT include UI rendering — add `bevy_ui_render` separately

`bevy_ui` only provides the layout engine (Taffy/flexbox), component types (`Node`,
`BackgroundColor`, etc.), and the ECS plumbing. The actual GPU draw calls are in
`bevy_ui_render`, which is a **separate crate and feature flag**.

Without `bevy_ui_render`, UI nodes exist, layout runs, and state transitions work — but
**nothing is visible on screen** (grey/blank). The symptom is identical to a missing
camera but the camera is fine.

```toml
# WRONG — UI layout only, no rendering
"bevy_ui", "bevy_text"

# CORRECT — layout + rendering
"bevy_ui", "bevy_ui_render", "bevy_text"
```

This was discovered when the email minigame showed a grey screen. 13 tests passed,
the state transitioned correctly, but the `Node` tree rendered nothing because
`bevy_ui_render` was missing from Cargo.toml.

```rust
// WRONG
for &child in children.iter() { ... }

// CORRECT
for child in children.iter() { ... }
// child is Entity (Copy), so you can pass it directly
```

## Controls convention

- WASD — player movement
- Arrow keys / IJKL / mouse — environment or camera interaction
- Escape — pause / menu

## Reference files

| Topic | File |
|-------|------|
| Cargo features & WSL2 constraints | [feature_flags.md](references/feature_flags.md) |
| Shapes, sprites, text, camera, z-layers | [rendering.md](references/rendering.md) |
| Components, resources, queries, events, observers | [ecs_patterns.md](references/ecs_patterns.md) |
| App, plugins, schedules, system-set ordering | [app_structure.md](references/app_structure.md) |
| Keyboard, mouse, gamepad input | [input.md](references/input.md) |
| Game states, OnEnter/OnExit, run conditions | [states.md](references/states.md) |
| Transform movement, velocity, clamping | [movement.md](references/movement.md) |
| MinimalPlugins, RunSystemOnce, test assertions | [testing.md](references/testing.md) |
| AABB collision, disjoint queries, depenetration | [collision.md](references/collision.md) |
| Bevy UI — Node, Text, Button, layout | [ui.md](references/ui.md) |
