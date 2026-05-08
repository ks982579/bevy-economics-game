# Bevy 0.18 Input Handling

## Keyboard — ButtonInput<KeyCode>

`ButtonInput<KeyCode>` is a resource updated every `PreUpdate`. Access it with `Res<ButtonInput<KeyCode>>`.

```rust
fn handle_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.single_mut() else { return; };

    // .pressed()      — true every frame the key is held
    // .just_pressed() — true only the first frame the key goes down
    // .just_released()— true only the first frame the key comes up

    if keys.pressed(KeyCode::KeyW) { transform.translation.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { transform.translation.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { transform.translation.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { transform.translation.x += 1.0; }

    if keys.just_pressed(KeyCode::Escape) { /* open pause menu */ }
    if keys.just_pressed(KeyCode::Space)  { /* fire / jump */ }
}
```

> **Note:** KeyCode values are *physical* key positions (layout-independent).
> Use `Key` (logical character) when you care about the actual character typed.

### Common KeyCode values

| Key | KeyCode |
|-----|---------|
| W A S D | `KeyCode::KeyW`, `KeyCode::KeyA`, `KeyCode::KeyS`, `KeyCode::KeyD` |
| Arrow keys | `KeyCode::ArrowUp`, `KeyCode::ArrowDown`, `KeyCode::ArrowLeft`, `KeyCode::ArrowRight` |
| Space | `KeyCode::Space` |
| Escape | `KeyCode::Escape` |
| Enter | `KeyCode::Enter` |
| Shift | `KeyCode::ShiftLeft`, `KeyCode::ShiftRight` |
| Digits | `KeyCode::Digit1` … `KeyCode::Digit0` |
| F-keys | `KeyCode::F1` … `KeyCode::F12` |

### Collect all pressed keys

```rust
fn debug_keys(keys: Res<ButtonInput<KeyCode>>) {
    for key in keys.get_pressed() {
        info!("{:?}", key);
    }
}
```

---

## Mouse buttons — ButtonInput<MouseButton>

```rust
fn handle_mouse_buttons(
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left)  { /* shoot */ }
    if buttons.pressed(MouseButton::Right)       { /* aim */ }
    if buttons.just_released(MouseButton::Left)  { /* release charge */ }
}
```

---

## Mouse cursor position

```rust
fn track_cursor(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let Ok(window) = windows.single() else { return; };
    let Ok((camera, camera_transform)) = camera_q.single() else { return; };

    if let Some(cursor_pos) = window.cursor_position() {
        // Convert screen pixels → world coordinates
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            info!("Cursor in world: {:?}", world_pos);
        }
    }
}
```

---

## Mouse movement delta

```rust
use bevy::input::mouse::MouseMotion;

fn handle_mouse_move(mut events: EventReader<MouseMotion>) {
    for event in events.read() {
        info!("Mouse moved: dx={}, dy={}", event.delta.x, event.delta.y);
    }
}
```

---

## Mouse scroll wheel

```rust
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseScrollUnit;

fn handle_scroll(mut scroll_events: EventReader<MouseWheel>) {
    for event in scroll_events.read() {
        match event.unit {
            MouseScrollUnit::Line  => info!("Scroll lines: {}", event.y),
            MouseScrollUnit::Pixel => info!("Scroll pixels: {}", event.y),
        }
    }
}
```

---

## Input-derived direction vector (clean movement pattern)

Compute a direction from WASD and normalize it so diagonal movement isn't faster.

```rust
fn movement_direction(keys: &Res<ButtonInput<KeyCode>>) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { dir.x += 1.0; }
    dir.normalize_or_zero()  // safe: returns ZERO when length is 0
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    time: Res<Time>,
) {
    let Ok((mut transform, speed)) = query.single_mut() else { return; };
    let direction = movement_direction(&keys);
    transform.translation += direction.extend(0.0) * speed.0 * time.delta_secs();
}
```

---

## Testing input (see testing.md)

```rust
app.world_mut()
    .resource_mut::<ButtonInput<KeyCode>>()
    .press(KeyCode::KeyD);

app.update();  // input is consumed and systems run
```

Remember to `init_resource::<ButtonInput<KeyCode>>()` in your test app.
