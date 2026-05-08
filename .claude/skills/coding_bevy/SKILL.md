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

### 6. 2D camera is required — always spawn one

```rust
commands.spawn(Camera2d);
```

Nothing renders without a camera. Camera sits at z = 1000 looking down -Z.

### 7. Transform z controls draw order in 2D

Higher z renders on top. Keep all 2D entities between z = 0.0 and z = 999.0.

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
