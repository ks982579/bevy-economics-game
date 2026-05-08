# Bevy 0.18 Movement Patterns

## The golden rule: use FixedUpdate for all movement

Put movement systems in `FixedUpdate`, not `Update`. This makes behaviour
frame-rate independent. Default fixed timestep is 64 Hz.

```rust
app.add_systems(FixedUpdate, move_player);
```

Inside a `FixedUpdate` system, use `Time<Fixed>` for the delta:

```rust
fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    time: Res<Time<Fixed>>,          // <-- Fixed, not Time
) {
    let Ok((mut transform, speed)) = query.single_mut() else { return; };
    // ...
    transform.translation.x += dx * speed.0 * time.delta_secs();
}
```

If you use `Res<Time>` (generic) in a `FixedUpdate` system it compiles fine but
gives the wrong delta — it returns the variable frame delta instead of the fixed one.

---

## Velocity component pattern

Store velocity as a component rather than computing it from input every frame.
This enables acceleration, friction, knockback, etc.

```rust
#[derive(Component, Default)]
struct Velocity(Vec2);

// In FixedUpdate — integrate velocity into position
fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time<Fixed>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

// In Update — set velocity from input (instant response, no lag)
fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Speed), With<Player>>,
) {
    let Ok((mut velocity, speed)) = query.single_mut() else { return; };
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { dir.x += 1.0; }
    velocity.0 = dir.normalize_or_zero() * speed.0;
}
```

---

## Screen-edge clamping

Keep an entity inside a fixed play area (e.g., 800 × 600 window).

```rust
const HALF_W: f32 = 400.0;
const HALF_H: f32 = 300.0;
const HALF_SPRITE: f32 = 16.0; // half the entity's visual width/height

fn clamp_to_bounds(mut query: Query<&mut Transform, With<Player>>) {
    let Ok(mut transform) = query.single_mut() else { return; };
    transform.translation.x = transform.translation.x
        .clamp(-HALF_W + HALF_SPRITE, HALF_W - HALF_SPRITE);
    transform.translation.y = transform.translation.y
        .clamp(-HALF_H + HALF_SPRITE, HALF_H - HALF_SPRITE);
}
```

Or derive bounds dynamically from the window:

```rust
fn clamp_to_window(
    windows: Query<&Window>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(window) = windows.single() else { return; };
    let Ok(mut transform) = query.single_mut() else { return; };
    let hw = window.width()  / 2.0 - 16.0;
    let hh = window.height() / 2.0 - 16.0;
    transform.translation.x = transform.translation.x.clamp(-hw, hw);
    transform.translation.y = transform.translation.y.clamp(-hh, hh);
}
```

---

## Simple AABB collision (no physics engine)

Axis-aligned bounding box overlap for two rectangular entities.

```rust
fn aabb_overlap(
    a_pos: Vec2, a_half: Vec2,
    b_pos: Vec2, b_half: Vec2,
) -> bool {
    (a_pos.x - b_pos.x).abs() < a_half.x + b_half.x
        && (a_pos.y - b_pos.y).abs() < a_half.y + b_half.y
}

fn check_player_enemy_collision(
    player_q: Query<&Transform, With<Player>>,
    enemy_q:  Query<&Transform, With<Enemy>>,
    mut events: EventWriter<PlayerHit>,
) {
    let Ok(player_tf) = player_q.single() else { return; };
    let player_pos = player_tf.translation.truncate();

    for enemy_tf in &enemy_q {
        let enemy_pos = enemy_tf.translation.truncate();
        if aabb_overlap(player_pos, Vec2::splat(16.0), enemy_pos, Vec2::splat(16.0)) {
            events.send(PlayerHit);
        }
    }
}
```

---

## Smooth follow camera

Move the camera toward the player each frame (lerp).

```rust
fn camera_follow(
    player_q: Query<&Transform, With<Player>>,
    mut camera_q: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_q.single() else { return; };
    let Ok(mut camera_tf) = camera_q.single_mut() else { return; };

    let target = player_tf.translation;
    camera_tf.translation = camera_tf.translation.lerp(target, 5.0 * time.delta_secs());
    camera_tf.translation.z = 1000.0; // keep camera z fixed
}
```

---

## Projectile movement

Spawn a bullet with a direction baked into a `Velocity`, then let `apply_velocity` move it.

```rust
fn fire_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_q: Query<&Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::Space) { return; }
    let Ok(player_tf) = player_q.single() else { return; };

    commands.spawn((
        Bullet,
        Mesh2d(meshes.add(Rectangle::new(8.0, 8.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(1.0, 0.8, 0.0)))),
        Transform::from_translation(player_tf.translation + Vec3::new(0.0, 20.0, 0.0)),
        Velocity(Vec2::new(0.0, 400.0)),  // upward at 400 units/s
    ));
}

// Despawn bullets that leave the screen
fn despawn_out_of_bounds(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, transform) in &query {
        if transform.translation.y > 400.0 || transform.translation.y < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}
```

---

## Rotation toward a target

```rust
fn face_player(
    player_q: Query<&Transform, With<Player>>,
    mut enemy_q: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    let Ok(player_tf) = player_q.single() else { return; };
    for mut enemy_tf in &mut enemy_q {
        let dir = (player_tf.translation - enemy_tf.translation).truncate();
        let angle = dir.y.atan2(dir.x) - std::f32::consts::FRAC_PI_2;
        enemy_tf.rotation = Quat::from_rotation_z(angle);
    }
}
```
