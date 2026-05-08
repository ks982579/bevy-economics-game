# Bevy 0.18 Rendering Patterns

## 2D Camera (always required)

```rust
commands.spawn(Camera2d);
```

- Sits at z = 1000 by default, looks down the -Z axis
- Without a camera, nothing renders — spawn it in your `Startup` system
- One camera per scene is typical for 2D

---

## Colored shapes (no texture)

### Rectangle / Square

```rust
commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(width, height))),
    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(r, g, b)))),
    Transform::from_xyz(x, y, z),
));
```

For a square: `Rectangle::new(32.0, 32.0)` — there is no `splat` method.

### Circle

```rust
commands.spawn((
    Mesh2d(meshes.add(Circle::new(radius))),
    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(r, g, b)))),
    Transform::from_xyz(x, y, z),
));
```

### Triangle

```rust
commands.spawn((
    Mesh2d(meshes.add(Triangle2d::new(
        Vec2::new(0.0, 16.0),
        Vec2::new(-16.0, -16.0),
        Vec2::new(16.0, -16.0),
    ))),
    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(r, g, b)))),
    Transform::from_xyz(x, y, z),
));
```

### Capsule (rounded rectangle / pill)

```rust
commands.spawn((
    Mesh2d(meshes.add(Capsule2d::new(radius, length))),
    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(r, g, b)))),
    Transform::from_xyz(x, y, z),
));
```

### Required features

All colored-mesh shapes need: `bevy_mesh`, `bevy_sprite`, `bevy_sprite_render`, `bevy_core_pipeline`, `bevy_render`. The `2d` feature group includes all of these.

---

## Sprite with texture

```rust
// System parameter: asset_server: Res<AssetServer>
commands.spawn((
    Sprite::from_image(asset_server.load("sprites/player.png")),
    Transform::from_xyz(x, y, z),
));
```

Requires: `bevy_sprite`, `bevy_asset`. Assets are loaded from the `assets/` folder next to the executable.

### Tinted sprite (color overlay)

```rust
commands.spawn((
    Sprite {
        image: asset_server.load("sprites/player.png"),
        color: Color::srgba(1.0, 0.5, 0.5, 1.0), // reddish tint
        ..default()
    },
    Transform::from_xyz(x, y, z),
));
```

---

## Text

```rust
commands.spawn((
    Text2d::new("Hello, Bevy!"),
    TextFont {
        font_size: 32.0,
        ..default()
    },
    TextColor(Color::WHITE),
    Transform::from_xyz(x, y, z),
));
```

Requires: `bevy_text`, `default_font` (for the built-in font). Text is centered at its transform by default.

---

## Z-ordering (draw layers)

Higher z renders on top. All 2D entities should be between z = 0.0 and z = 999.0.

| Layer | z value | Contents |
|-------|---------|---------|
| Background / tiles | `0.0` | Floor, scenery |
| Items / pickups | `1.0` | World objects |
| Enemies / NPCs | `2.0` | Blue squares |
| Player | `3.0` | Green square |
| Projectiles | `4.0` | Bullets, effects |
| UI overlays | `10.0+` | HUD, score text |

---

## Entity color conventions (this project)

| Entity | Color | Size |
|--------|-------|------|
| Player | `Color::srgb(0.2, 0.8, 0.2)` green | 32 × 32 |
| Enemy / NPC | `Color::srgb(0.2, 0.4, 0.9)` blue | 32 × 32 |
| Projectile | `Color::srgb(1.0, 0.8, 0.0)` yellow | 8 × 8 |

---

## Visibility

Entities are visible by default. To hide/show at runtime:

```rust
// Hide
*visibility = Visibility::Hidden;

// Show
*visibility = Visibility::Visible;

// Let parent decide (default for children)
*visibility = Visibility::Inherited;
```

`Visibility` is automatically required by `Mesh2d` and `Sprite`, so you don't need to add it manually.

---

## Scaling and rotation

```rust
// Scale uniformly
Transform::from_xyz(x, y, z).with_scale(Vec3::splat(2.0))

// Rotate (2D = around Z axis)
Transform::from_xyz(x, y, z).with_rotation(Quat::from_rotation_z(angle_radians))

// Combined
commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::GREEN))),
    Transform {
        translation: Vec3::new(x, y, z),
        rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0), // 45°
        scale: Vec3::splat(1.5),
    },
));
```
