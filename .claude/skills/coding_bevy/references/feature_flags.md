# Bevy 0.18 Feature Flags

All features used with `default-features = false`.

## Currently enabled in this project

| Feature | Purpose |
|---------|---------|
| `bevy_asset` | Asset loading infrastructure |
| `bevy_state` | App state machine |
| `bevy_window` | Window abstraction |
| `bevy_winit` | OS window creation |
| `bevy_log` | Logging |
| `default_font` | Built-in font for text rendering |
| `multi_threaded` | Parallel system execution |
| `async_executor` | Async task support |
| `reflect_auto_register` | Auto-registers reflected types |
| `bevy_core_pipeline` | Core 2D/3D render pipeline |
| `bevy_render` | GPU rendering backend |
| `bevy_sprite` | Sprite component and `SpritePlugin` |
| `bevy_mesh` | Mesh asset type |
| `bevy_sprite_render` | `ColorMaterial`, `MeshMaterial2d`, colored mesh rendering |
| `bevy_ui` | UI layout and widgets |
| `bevy_text` | Text rendering |
| `x11` | X11 window backend (WSL2) |

## Never add (WSL2 incompatible)

| Feature | Why |
|---------|-----|
| `wayland` | Requires `libwayland-client` — not present in WSL2 |
| `bevy_gilrs` | Requires `libudev` — not present in WSL2 |

## Common additions (not yet needed)

| Feature | Purpose |
|---------|---------|
| `bevy_audio` | Audio playback |
| `bevy_scene` | Scene serialization |
| `bevy_gltf` | GLTF 3D model loading |
| `bevy_pbr` | 3D PBR rendering |
| `dynamic_linking` | Faster iterative compile times in dev |
