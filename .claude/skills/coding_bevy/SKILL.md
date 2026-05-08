---
name: coding_bevy
description: Bevy 0.18 coding guidance for this project. Use when writing, editing, or debugging any Bevy code — systems, components, plugins, rendering, input, or tests.
---

# Bevy 0.18 — Project Coding Skill

## Platform constraints (WSL2)

- Always `default-features = false` in Cargo.toml
- Never add `wayland` or `bevy_gilrs` — missing system libs in this environment
- Window backend: `bevy_winit` + `x11` only

## Core API gotchas

- `Query::single()` and `Query::single_mut()` return `Result` — always use `let Ok(val) = query.single_mut() else { return };`
- `Sprite` requires a real `Handle<Image>` — use `Mesh2d` + `MeshMaterial2d(ColorMaterial)` for plain colored shapes instead
- `Rectangle::new(w, h)` — there is no `splat` method; use `Rectangle::new(s, s)` for a square

## Colored shape spawning (the correct pattern)

```rust
commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(r, g, b)))),
    Transform::from_xyz(x, y, 0.0),
));
```

Requires features: `bevy_sprite`, `bevy_mesh`, `bevy_sprite_render`

## Controls convention

- WASD — player movement
- Right-hand keys (arrows, IJKL, mouse) — reserved for environment interaction

## References

- [Feature flags](references/feature_flags.md) — which Cargo features enable which functionality
- [Testing patterns](references/testing.md) — MinimalPlugins setup, RunSystemOnce, resource init
- [Rendering patterns](references/rendering.md) — shapes, sprites, text, layers
