# Bevy 0.18 Rendering Patterns

## Colored rectangle (no texture)

```rust
commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(width, height))),
    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(r, g, b)))),
    Transform::from_xyz(x, y, z),
));
```

Requires: `bevy_mesh`, `bevy_sprite_render`

## Colored circle

```rust
commands.spawn((
    Mesh2d(meshes.add(Circle::new(radius))),
    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(r, g, b)))),
    Transform::from_xyz(x, y, z),
));
```

## Sprite with texture

```rust
// In a system with asset_server: Res<AssetServer>
commands.spawn((
    Sprite::from_image(asset_server.load("path/to/image.png")),
    Transform::from_xyz(x, y, z),
));
```

Requires: `bevy_sprite`, `bevy_asset`

## Z-ordering (draw layers)

The `z` value of `Transform::from_xyz(x, y, z)` controls draw order in 2D — higher z renders on top.

Suggested conventions for this project:

| Layer | Z value | Contents |
|-------|---------|---------|
| Ground / tiles | `0.0` | Background, floor |
| Items / pickups | `1.0` | World objects |
| NPCs | `2.0` | Blue squares |
| Player | `3.0` | Green square |
| UI | `10.0+` | HUD overlays |

## Entity color conventions

| Entity | Color | Size |
|--------|-------|------|
| Player | `Color::srgb(0.2, 0.8, 0.2)` green | 32×32 |
| NPC | `Color::srgb(0.2, 0.4, 0.9)` blue | 32×32 |

## 2D camera

```rust
commands.spawn(Camera2d);
```

Camera sits at `z = 1000` by default and looks down the -Z axis. All 2D entities should have `z` between `0.0` and `999.0`.
