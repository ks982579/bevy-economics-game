use bevy::prelude::*;

/// Axis-aligned bounding box for solid collision. Stored as half-extents.
#[derive(Component, Clone, Copy)]
pub struct Collider {
    pub half_w: f32,
    pub half_h: f32,
}

impl Collider {
    pub fn new(w: f32, h: f32) -> Self {
        Self { half_w: w / 2.0, half_h: h / 2.0 }
    }
}

#[derive(Component)]
pub struct Player;

pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_SIZE: f32 = 32.0;

/// Saved overworld player position. Persists across state transitions so the
/// player reappears where they left, not at the spawn default.
#[derive(Resource)]
pub struct OverworldContext {
    pub player_pos: Vec2,
}

impl Default for OverworldContext {
    fn default() -> Self {
        Self { player_pos: Vec2::ZERO }
    }
}

/// Saved office player position. Default puts the player just inside the entrance.
#[derive(Resource)]
pub struct OfficeContext {
    pub player_pos: Vec2,
}

impl Default for OfficeContext {
    fn default() -> Self {
        Self { player_pos: Vec2::new(0.0, -200.0) }
    }
}

/// Resolves AABB overlap between `new_pos` and an obstacle. Returns the pushed-out position.
pub fn resolve_aabb(
    new_pos: Vec2,
    player_col: &Collider,
    obs_pos: Vec2,
    obs_col: &Collider,
) -> Vec2 {
    let combined_hw = player_col.half_w + obs_col.half_w;
    let combined_hh = player_col.half_h + obs_col.half_h;
    let diff = new_pos - obs_pos;
    let overlap_x = combined_hw - diff.x.abs();
    let overlap_y = combined_hh - diff.y.abs();

    if overlap_x > 0.0 && overlap_y > 0.0 {
        if overlap_x < overlap_y {
            Vec2::new(new_pos.x + overlap_x * diff.x.signum(), new_pos.y)
        } else {
            Vec2::new(new_pos.x, new_pos.y + overlap_y * diff.y.signum())
        }
    } else {
        new_pos
    }
}
