use bevy::prelude::*;

use crate::shared::{Collider, Player, PLAYER_SIZE, PLAYER_SPEED, resolve_aabb};
use crate::state::GameState;

const BUILDING_W: f32 = 120.0;
const BUILDING_H: f32 = 80.0;
const BUILDING_X: f32 = -480.0;
const BUILDING_Y: f32 = 0.0;

/// Entrance trigger zone: a thin strip just below the building's bottom edge.
const ENTRANCE_W: f32 = BUILDING_W;
const ENTRANCE_H: f32 = 20.0;
const ENTRANCE_Y: f32 = BUILDING_Y - BUILDING_H / 2.0 - ENTRANCE_H / 2.0 - 2.0;

#[derive(Component)]
pub struct Building;

/// Marker for entities that belong to the overworld scene.
#[derive(Component)]
struct OverworldEntity;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Overworld), setup_overworld)
            .add_systems(OnExit(GameState::Overworld), cleanup_overworld)
            .add_systems(
                Update,
                (move_player_overworld, check_building_entry)
                    .chain()
                    .run_if(in_state(GameState::Overworld)),
            );
    }
}

fn setup_overworld(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        OverworldEntity,
        Collider::new(PLAYER_SIZE, PLAYER_SIZE),
        Mesh2d(meshes.add(Rectangle::new(PLAYER_SIZE, PLAYER_SIZE))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.8, 0.2)))),
        Transform::from_xyz(0.0, 0.0, 3.0),
    ));

    commands.spawn((
        Building,
        OverworldEntity,
        Collider::new(BUILDING_W, BUILDING_H),
        Mesh2d(meshes.add(Rectangle::new(BUILDING_W, BUILDING_H))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.25, 0.25, 0.25)))),
        Transform::from_xyz(BUILDING_X, BUILDING_Y, 0.0),
    ));

    // Door highlight strip at the bottom of the building
    commands.spawn((
        OverworldEntity,
        Mesh2d(meshes.add(Rectangle::new(ENTRANCE_W * 0.4, ENTRANCE_H))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.6, 0.45, 0.2)))),
        Transform::from_xyz(BUILDING_X, BUILDING_Y - BUILDING_H / 2.0 + ENTRANCE_H / 2.0, 1.0),
    ));
}

fn cleanup_overworld(mut commands: Commands, query: Query<Entity, With<OverworldEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn move_player_overworld(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Collider), With<Player>>,
    obstacle_query: Query<(&Transform, &Collider), (With<Building>, Without<Player>)>,
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

    transform.translation = new_pos.extend(transform.translation.z);
}

pub fn check_building_entry(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform, &Collider), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((player_entity, transform, player_col)) = player_query.single() else { return };

    let player_pos = transform.translation.truncate();
    let entrance_col = Collider::new(ENTRANCE_W, ENTRANCE_H);
    let entrance_pos = Vec2::new(BUILDING_X, ENTRANCE_Y);

    let dx = (player_pos.x - entrance_pos.x).abs();
    let dy = (player_pos.y - entrance_pos.y).abs();
    let overlapping = dx < player_col.half_w + entrance_col.half_w
        && dy < player_col.half_h + entrance_col.half_h;

    if overlapping {
        commands.entity(player_entity).despawn();
        next_state.set(GameState::Office);
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
            .init_resource::<ButtonInput<KeyCode>>();
        app
    }

    #[test]
    fn player_cannot_walk_through_building() {
        let mut app = test_app();
        app.add_systems(Update, move_player_overworld);

        let start_x = BUILDING_X + BUILDING_W / 2.0 + PLAYER_SIZE / 2.0 + 1.0;
        app.world_mut().spawn((
            Player,
            Collider::new(PLAYER_SIZE, PLAYER_SIZE),
            Transform::from_xyz(start_x, BUILDING_Y, 3.0),
        ));
        app.world_mut().spawn((
            Building,
            Collider::new(BUILDING_W, BUILDING_H),
            Transform::from_xyz(BUILDING_X, BUILDING_Y, 0.0),
        ));
        app.update();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyA);

        for _ in 0..120 {
            app.update();
        }

        let mut q = app.world_mut().query_filtered::<&Transform, With<Player>>();
        let x = q.single(app.world()).unwrap().translation.x;
        let min_x = BUILDING_X + BUILDING_W / 2.0 + PLAYER_SIZE / 2.0;
        assert!(x >= min_x - 0.1, "player x={x} should not be inside building (min: {min_x})");
    }

    #[test]
    fn entry_trigger_fires_when_player_in_entrance_zone() {
        let mut app = test_app();
        app.init_state::<GameState>()
            .add_systems(Update, check_building_entry.run_if(in_state(GameState::Overworld)));

        app.world_mut().spawn((
            Player,
            Collider::new(PLAYER_SIZE, PLAYER_SIZE),
            Transform::from_xyz(BUILDING_X, ENTRANCE_Y, 3.0),
        ));
        app.update(); // system runs, schedules transition
        app.update(); // transition applied

        let state = app.world().resource::<State<GameState>>();
        assert_eq!(*state.get(), GameState::Office, "should have transitioned to Office");
    }

    #[test]
    fn entry_trigger_does_not_fire_far_from_building() {
        let mut app = test_app();
        app.init_state::<GameState>()
            .add_systems(Update, check_building_entry.run_if(in_state(GameState::Overworld)));

        app.world_mut().spawn((
            Player,
            Collider::new(PLAYER_SIZE, PLAYER_SIZE),
            Transform::from_xyz(0.0, 0.0, 3.0),
        ));
        app.update();

        let state = app.world().resource::<State<GameState>>();
        assert_eq!(*state.get(), GameState::Overworld, "should stay in Overworld");
    }

    #[test]
    fn building_spawns_in_setup() {
        let mut app = test_app();
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Overworld), setup_overworld);

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Overworld);
        app.update();
        app.update();

        let count = app
            .world_mut()
            .query_filtered::<Entity, With<Building>>()
            .iter(app.world())
            .count();
        assert_eq!(count, 1);
    }
}
