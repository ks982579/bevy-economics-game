# Bevy 0.18 Collision Patterns

## Disjoint query requirement

When one query borrows a component mutably and another borrows the same component
type immutably, Bevy requires the queries to be provably disjoint. Use `Without<T>`
on the read-only query to satisfy the borrow checker:

```rust
fn move_player(
    mut player_query: Query<(&mut Transform, &Collider), With<Player>>,
    obstacle_query: Query<(&Transform, &Collider), (With<Building>, Without<Player>)>,
) { ... }
```

Without `Without<Player>` on `obstacle_query`, Bevy will panic at runtime because
both queries could match the same entity.

---

## AABB Collider component

Store collision bounds as half-extents so overlap math stays symmetric:

```rust
#[derive(Component, Clone, Copy)]
pub struct Collider {
    pub half_w: f32,
    pub half_h: f32,
}

impl Collider {
    fn new(w: f32, h: f32) -> Self {
        Self { half_w: w / 2.0, half_h: h / 2.0 }
    }
}
```

Attach to every solid entity alongside its `Transform`.

---

## AABB overlap test and depenetration

Compute the proposed new position first, then push it out if it overlaps any obstacle.
This keeps movement and collision in one system with no extra events or resources.

```rust
let combined_hw = player_col.half_w + obs_col.half_w;
let combined_hh = player_col.half_h + obs_col.half_h;

let diff = new_pos - obs_pos;
let overlap_x = combined_hw - diff.x.abs();
let overlap_y = combined_hh - diff.y.abs();

if overlap_x > 0.0 && overlap_y > 0.0 {
    // Push out along the axis of least overlap
    if overlap_x < overlap_y {
        new_pos.x += overlap_x * diff.x.signum();
    } else {
        new_pos.y += overlap_y * diff.y.signum();
    }
}
```

Apply the corrected position back, preserving the z value:

```rust
transform.translation = new_pos.extend(transform.translation.z);
```

---

## Testing collision

Spawn entities directly (no `run_system_once` needed for static world state),
drive movement for many frames, then assert position never penetrates:

```rust
let start_x = BUILDING_X + BUILDING_W / 2.0 + PLAYER_SIZE / 2.0 + 1.0;
app.world_mut().spawn((Player, Collider::new(PLAYER_SIZE, PLAYER_SIZE), Transform::from_xyz(start_x, 0.0, 0.0)));
app.world_mut().spawn((Building, Collider::new(BUILDING_W, BUILDING_H), Transform::from_xyz(BUILDING_X, 0.0, 0.0)));

app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::KeyA);
for _ in 0..120 { app.update(); }

let x = { /* query player x */ };
let min_x = BUILDING_X + BUILDING_W / 2.0 + PLAYER_SIZE / 2.0;
assert!(x >= min_x - 0.1);
```

The `-0.1` tolerance accounts for floating-point drift over many frames.

---

## Limitations of this approach

- Works correctly for a single moving entity against static obstacles
- For multiple moving entities colliding with each other, each mover needs its own
  pass or you need a dedicated collision system that processes all pairs
- Does not handle high-speed tunnelling (entity moves faster than its own size in
  one frame) — at 200 px/s and 60 fps the player moves ~3 px/frame, well within
  the 16 px half-width safety margin
