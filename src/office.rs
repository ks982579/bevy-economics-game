use bevy::prelude::*;

use crate::shared::{Collider, OfficeContext, Player, PLAYER_SIZE, PLAYER_SPEED, resolve_aabb};
use crate::state::GameState;

const OFFICE_W: f32 = 800.0;
const OFFICE_H: f32 = 500.0;

const DESK_W: f32 = 80.0;
const DESK_H: f32 = 50.0;
const COMPUTER_W: f32 = 30.0;
const COMPUTER_H: f32 = 20.0;

const INTERACT_RANGE: f32 = 60.0;

pub const DESK_POSITIONS: [(f32, f32); 4] = [
    (-200.0, 80.0), // player's desk
    (0.0, 80.0),
    (200.0, 80.0),
    (0.0, -80.0),
];
pub const PLAYER_DESK_IDX: usize = 0;

#[derive(Component)]
pub struct OfficeFurniture;

/// Marks the player's own computer terminal.
#[derive(Component)]
pub struct PlayerComputer;

/// Marker for all entities belonging to the office scene.
#[derive(Component)]
struct OfficeEntity;

pub struct OfficePlugin;

impl Plugin for OfficePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Office), setup_office)
            .add_systems(OnExit(GameState::Office), cleanup_office)
            .add_systems(
                Update,
                (move_player_office, check_computer_interact)
                    .chain()
                    .run_if(in_state(GameState::Office)),
            );
    }
}

fn setup_office(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ctx: Res<OfficeContext>,
) {
    let pos = ctx.player_pos;
    commands.spawn((
        Player,
        OfficeEntity,
        Collider::new(PLAYER_SIZE, PLAYER_SIZE),
        Mesh2d(meshes.add(Rectangle::new(PLAYER_SIZE, PLAYER_SIZE))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.8, 0.2)))),
        Transform::from_xyz(pos.x, pos.y, 3.0),
    ));

    // Office floor
    commands.spawn((
        OfficeEntity,
        Mesh2d(meshes.add(Rectangle::new(OFFICE_W, OFFICE_H))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.85, 0.82, 0.78)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Solid walls — top, left, right
    for (x, y, w, h) in [
        (0.0_f32, OFFICE_H / 2.0, OFFICE_W, 10.0_f32),
        (-OFFICE_W / 2.0, 0.0, 10.0, OFFICE_H),
        (OFFICE_W / 2.0, 0.0, 10.0, OFFICE_H),
    ] {
        commands.spawn((
            OfficeEntity,
            OfficeFurniture,
            Collider::new(w, h),
            Mesh2d(meshes.add(Rectangle::new(w, h))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.4, 0.4, 0.4)))),
            Transform::from_xyz(x, y, 1.0),
        ));
    }

    // Bottom wall — visual only, no collider (player walks through it to exit)
    commands.spawn((
        OfficeEntity,
        Mesh2d(meshes.add(Rectangle::new(OFFICE_W, 10.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.4, 0.4, 0.4)))),
        Transform::from_xyz(0.0, -OFFICE_H / 2.0, 1.0),
    ));

    // Desks + computers
    for (idx, (dx, dy)) in DESK_POSITIONS.iter().enumerate() {
        let is_player_desk = idx == PLAYER_DESK_IDX;

        commands.spawn((
            OfficeEntity,
            OfficeFurniture,
            Collider::new(DESK_W, DESK_H),
            Mesh2d(meshes.add(Rectangle::new(DESK_W, DESK_H))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.55, 0.38, 0.18)))),
            Transform::from_xyz(*dx, *dy, 1.0),
        ));

        let monitor_color = if is_player_desk {
            Color::srgb(0.2, 0.6, 1.0) // bright blue = player's computer
        } else {
            Color::srgb(0.15, 0.15, 0.15)
        };

        let mut ecmds = commands.spawn((
            OfficeEntity,
            Mesh2d(meshes.add(Rectangle::new(COMPUTER_W, COMPUTER_H))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(monitor_color))),
            Transform::from_xyz(*dx, *dy + DESK_H / 2.0 + COMPUTER_H / 2.0, 2.0),
        ));
        if is_player_desk {
            ecmds.insert(PlayerComputer);
        }
    }

    // Hint text
    commands.spawn((
        OfficeEntity,
        Text2d::new("[ E ] use computer  |  walk south to exit"),
        TextFont { font_size: 14.0, ..default() },
        TextColor(Color::srgb(0.4, 0.4, 0.4)),
        Transform::from_xyz(0.0, -OFFICE_H / 2.0 + 20.0, 10.0),
    ));
}

fn cleanup_office(mut commands: Commands, query: Query<Entity, With<OfficeEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn move_player_office(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Collider), With<Player>>,
    obstacle_query: Query<(&Transform, &Collider), (With<OfficeFurniture>, Without<Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ctx: ResMut<OfficeContext>,
) {
    let Ok((mut transform, player_col)) = player_query.single_mut() else { return };

    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) { direction.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { direction.x += 1.0; }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    let delta = direction * PLAYER_SPEED * time.delta_secs();
    let mut new_pos = transform.translation.truncate() + delta;

    for (obs_transform, obs_col) in &obstacle_query {
        new_pos = resolve_aabb(new_pos, player_col, obs_transform.translation.truncate(), obs_col);
    }

    // Walking through the open bottom wall returns to the overworld
    if new_pos.y < -OFFICE_H / 2.0 + PLAYER_SIZE {
        ctx.player_pos = new_pos;
        next_state.set(GameState::Overworld);
        return;
    }

    transform.translation = new_pos.extend(transform.translation.z);
}

pub fn check_computer_interact(
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    computer_query: Query<&Transform, With<PlayerComputer>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ctx: ResMut<OfficeContext>,
) {
    if !keys.just_pressed(KeyCode::KeyE) { return; }

    let Ok(player_t) = player_query.single() else { return };
    let Ok(computer_t) = computer_query.single() else { return };

    let dist = player_t.translation.truncate().distance(computer_t.translation.truncate());
    if dist <= INTERACT_RANGE {
        ctx.player_pos = player_t.translation.truncate();
        next_state.set(GameState::EmailMinigame);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<ColorMaterial>>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<OfficeContext>();
        app
    }

    #[test]
    fn office_setup_spawns_desks_and_computer() {
        let mut app = test_app();
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Office), setup_office);

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Office);
        app.update();
        app.update();

        let desk_count = app
            .world_mut()
            .query_filtered::<Entity, With<OfficeFurniture>>()
            .iter(app.world())
            .count();
        assert!(desk_count >= DESK_POSITIONS.len(), "should have at least one entity per desk (walls + desks)");

        let computer_count = app
            .world_mut()
            .query_filtered::<Entity, With<PlayerComputer>>()
            .iter(app.world())
            .count();
        assert_eq!(computer_count, 1, "exactly one player computer");
    }

    #[test]
    fn e_key_near_computer_transitions_to_email() {
        let mut app = test_app();
        app.init_state::<GameState>()
            .add_systems(Update, check_computer_interact.run_if(in_state(GameState::Office)));

        let (desk_x, desk_y) = DESK_POSITIONS[PLAYER_DESK_IDX];
        let computer_pos = Vec3::new(desk_x, desk_y + DESK_H / 2.0 + COMPUTER_H / 2.0, 2.0);

        app.world_mut().spawn((
            Player,
            Transform::from_translation(computer_pos + Vec3::new(20.0, 0.0, 1.0)),
        ));
        app.world_mut().spawn((
            PlayerComputer,
            Transform::from_translation(computer_pos),
        ));

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Office);
        app.update();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        app.update(); // system runs, schedules transition
        app.update(); // transition applied

        let state = app.world().resource::<State<GameState>>();
        assert_eq!(*state.get(), GameState::EmailMinigame);
    }

    #[test]
    fn setup_office_spawns_player_at_context_position() {
        let mut app = test_app();
        let saved_pos = Vec2::new(-50.0, -100.0);
        app.world_mut().resource_mut::<OfficeContext>().player_pos = saved_pos;

        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Office), setup_office);

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Office);
        app.update();
        app.update();

        let mut q = app.world_mut().query_filtered::<&Transform, With<Player>>();
        let t = q.single(app.world()).unwrap();
        let actual = t.translation.truncate();
        assert!(
            actual.distance(saved_pos) < 1.0,
            "player should spawn at saved position {saved_pos:?}, got {actual:?}"
        );
    }

    #[test]
    fn e_key_near_computer_saves_position_and_transitions() {
        let mut app = test_app();
        app.init_state::<GameState>()
            .add_systems(Update, check_computer_interact.run_if(in_state(GameState::Office)));

        let (desk_x, desk_y) = DESK_POSITIONS[PLAYER_DESK_IDX];
        let computer_pos = Vec3::new(desk_x, desk_y + DESK_H / 2.0 + COMPUTER_H / 2.0, 2.0);
        let player_pos = computer_pos + Vec3::new(20.0, 0.0, 1.0);

        app.world_mut().spawn((Player, Transform::from_translation(player_pos)));
        app.world_mut().spawn((PlayerComputer, Transform::from_translation(computer_pos)));

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Office);
        app.update();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        app.update();
        app.update();

        let saved = app.world().resource::<OfficeContext>().player_pos;
        let expected = player_pos.truncate();
        assert!(
            saved.distance(expected) < 1.0,
            "OfficeContext should record player pos before entering email, got {saved:?}"
        );
    }

    #[test]
    fn e_key_far_from_computer_does_not_transition() {
        let mut app = test_app();
        app.init_state::<GameState>()
            .add_systems(Update, check_computer_interact.run_if(in_state(GameState::Office)));

        let (desk_x, desk_y) = DESK_POSITIONS[PLAYER_DESK_IDX];
        let computer_pos = Vec3::new(desk_x, desk_y + DESK_H / 2.0 + COMPUTER_H / 2.0, 2.0);

        app.world_mut().spawn((Player, Transform::from_xyz(300.0, 300.0, 3.0)));
        app.world_mut().spawn((PlayerComputer, Transform::from_translation(computer_pos)));

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Office);
        app.update();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        app.update();

        let state = app.world().resource::<State<GameState>>();
        assert_eq!(*state.get(), GameState::Office, "should stay in Office when far from computer");
    }
}
