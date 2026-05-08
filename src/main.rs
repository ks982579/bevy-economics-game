use bevy::prelude::*;

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 32.0;

const BUILDING_W: f32 = 120.0;
const BUILDING_H: f32 = 80.0;
const BUILDING_X: f32 = -480.0;
const BUILDING_Y: f32 = 0.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Building;

/// Axis-aligned bounding box for solid collision. Stored as half-extents.
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Economy Sim".into(),
                resolution: (1280_u32, 720_u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    spawn_player(&mut commands, &mut meshes, &mut materials);
    spawn_building(&mut commands, &mut meshes, &mut materials);
}

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    commands.spawn((
        Player,
        Collider::new(PLAYER_SIZE, PLAYER_SIZE),
        Mesh2d(meshes.add(Rectangle::new(PLAYER_SIZE, PLAYER_SIZE))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.8, 0.2)))),
        Transform::from_xyz(0.0, 0.0, 3.0),
    ));
}

pub fn spawn_building(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    commands.spawn((
        Building,
        Collider::new(BUILDING_W, BUILDING_H),
        Mesh2d(meshes.add(Rectangle::new(BUILDING_W, BUILDING_H))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.25, 0.25, 0.25)))),
        Transform::from_xyz(BUILDING_X, BUILDING_Y, 0.0),
    ));
}

fn move_player(
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
        let obs_pos = obs_transform.translation.truncate();
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
    }

    transform.translation = new_pos.extend(transform.translation.z);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<ColorMaterial>>()
            .init_resource::<ButtonInput<KeyCode>>();
        app
    }

    #[test]
    fn player_spawns() {
        let mut app = test_app();

        let _ = app.world_mut().run_system_once(
            |mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<ColorMaterial>>| {
                spawn_player(&mut commands, &mut meshes, &mut materials);
            },
        );
        app.update();

        let mut query = app.world_mut().query_filtered::<Entity, With<Player>>();
        assert_eq!(query.iter(app.world()).count(), 1);
    }

    #[test]
    fn building_spawns() {
        let mut app = test_app();

        let _ = app.world_mut().run_system_once(
            |mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<ColorMaterial>>| {
                spawn_building(&mut commands, &mut meshes, &mut materials);
            },
        );
        app.update();

        let mut query = app.world_mut().query_filtered::<Entity, With<Building>>();
        assert_eq!(query.iter(app.world()).count(), 1);
    }

    #[test]
    fn player_moves_right_when_d_pressed() {
        let mut app = test_app();
        app.add_systems(Update, move_player);

        let _ = app.world_mut().run_system_once(
            |mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<ColorMaterial>>| {
                spawn_player(&mut commands, &mut meshes, &mut materials);
            },
        );
        app.update();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);

        let x_before = {
            let mut q = app.world_mut().query_filtered::<&Transform, With<Player>>();
            q.single(app.world()).unwrap().translation.x
        };

        app.update();

        let x_after = {
            let mut q = app.world_mut().query_filtered::<&Transform, With<Player>>();
            q.single(app.world()).unwrap().translation.x
        };

        assert!(x_after > x_before, "player should move right when D pressed");
    }

    #[test]
    fn player_cannot_walk_through_building() {
        let mut app = test_app();
        app.add_systems(Update, move_player);

        // Spawn player just to the right of the building edge
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

        // Drive player left into the building for many frames
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyA);

        for _ in 0..120 {
            app.update();
        }

        let x = {
            let mut q = app.world_mut().query_filtered::<&Transform, With<Player>>();
            q.single(app.world()).unwrap().translation.x
        };

        let min_x = BUILDING_X + BUILDING_W / 2.0 + PLAYER_SIZE / 2.0;
        assert!(
            x >= min_x - 0.1,
            "player x={x} should not be inside building (min allowed: {min_x})"
        );
    }
}
