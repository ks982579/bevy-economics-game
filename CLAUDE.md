# Claude Code Instructions

## Architecture
See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for system/component design.
See [CHANGELOG.md](CHANGELOG.md) for the backlog and ideas list.

## Platform Constraints (WSL2)
- Always keep `default-features = false` in Cargo.toml for Bevy
- Never add `wayland` or `bevy_gilrs` features — missing system libs in this environment
- X11 only; `bevy_winit` with `x11` feature is the correct window backend

## Bevy 0.18 Notes
- `Query::single_mut()` returns `Result` — use `let Ok(val) = query.single_mut() else { return };`
- Same applies to `Query::single()`
- Colored rectangles use `Mesh2d` + `MeshMaterial2d(ColorMaterial)` — not `Sprite` (which requires an image handle)
- `ColorMaterial` and `MeshMaterial2d` are in `bevy::prelude::*` but require the `bevy_sprite_render` feature
- `Rectangle::new(w, h)` — no `splat` method; use `Rectangle::new(s, s)` for a square
- Tests: use `MinimalPlugins`, `init_resource::<Assets<Mesh>>()`, `init_resource::<Assets<ColorMaterial>>()`, and `use bevy::ecs::system::RunSystemOnce`

## Controls Convention
- WASD — player movement (already implemented)
- Right-hand keys (e.g. arrow keys, IJKL, or mouse) — reserved for environment interaction
