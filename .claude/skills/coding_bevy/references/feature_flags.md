# Bevy 0.18 Feature Flags

All features used with `default-features = false` in Cargo.toml.

## 0.18 high-level feature groups (NEW — prefer these)

Bevy 0.18 introduced cargo feature *collections* that bundle the right set of
sub-features for common app types. Use these instead of listing individual features.

| Group | Enables |
|-------|---------|
| `2d` | Everything needed for 2D games: rendering, sprites, mesh, materials, UI, text, window, asset loading |
| `3d` | Everything for 3D games: PBR, lighting, shadows, cameras, mesh, etc. |
| `ui` | UI-only subset (panels, buttons, text — no full game loop) |

### Minimal 2D Cargo.toml (WSL2)

```toml
[dependencies]
bevy = { version = "0.18", default-features = false, features = [
    "2d",
    "bevy_winit",
    "x11",
    "bevy_log",
    "multi_threaded",
] }
```

This is the preferred starting point for any 2D game on WSL2.

---

## Individual features (when you need fine control)

Use these when the group adds too much compile weight, or when you need a
specific capability not covered by a group.

| Feature | Purpose |
|---------|---------|
| `bevy_asset` | Asset loading infrastructure |
| `bevy_state` | App state machine (`derive(States)`, `OnEnter`, `OnExit`) |
| `bevy_window` | Window abstraction |
| `bevy_winit` | OS window creation (always required on desktop) |
| `bevy_log` | Logging (`info!`, `warn!`, `error!`) |
| `default_font` | Built-in font for text rendering (no font file needed) |
| `multi_threaded` | Parallel system execution |
| `async_executor` | Async task support |
| `reflect_auto_register` | Auto-registers reflected types (no manual `.register::<T>()`) |
| `bevy_core_pipeline` | Core 2D/3D render pipeline |
| `bevy_render` | GPU rendering backend |
| `bevy_sprite` | `Sprite`, `SpritePlugin` |
| `bevy_mesh` | `Mesh` asset type, primitive shapes |
| `bevy_sprite_render` | `ColorMaterial`, `MeshMaterial2d`, colored mesh rendering |
| `bevy_ui` | UI layout and widgets |
| `bevy_text` | Text rendering |
| `x11` | X11 window backend (required on WSL2) |
| `dynamic_linking` | Faster iterative compile times in dev (never in release builds) |
| `bevy_audio` | Audio playback |
| `bevy_scene` | Scene serialization / `.scn.ron` files |
| `bevy_gltf` | GLTF 3D model loading |
| `bevy_pbr` | 3D PBR rendering |

---

## Never add (WSL2 incompatible)

| Feature | Why blocked |
|---------|-------------|
| `wayland` | Requires `libwayland-client` — not present in WSL2 |
| `bevy_gilrs` | Requires `libudev` — not present in WSL2 |

These will cause linker errors at compile time. Do not add them even if an
example or tutorial lists them.

---

## Key feature dependencies for colored shapes

To spawn a colored `Mesh2d` rectangle (the standard way to draw a square):

```
bevy_mesh          ← Rectangle, Circle, etc.
bevy_sprite        ← Mesh2d component
bevy_sprite_render ← ColorMaterial, MeshMaterial2d
bevy_core_pipeline ← render pipeline
bevy_render        ← GPU backend
```

The `2d` group includes all of the above.
