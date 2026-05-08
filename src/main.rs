use bevy::prelude::*;

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Player;

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
}

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Rectangle::new(PLAYER_SIZE, PLAYER_SIZE))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.8, 0.2)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.single_mut() else { return };
    let mut direction = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    transform.translation += direction.extend(0.0) * PLAYER_SPEED * time.delta_secs();
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
}
