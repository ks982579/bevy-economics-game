use bevy::prelude::*;

mod email;
mod office;
mod overworld;
mod shared;
mod state;

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
        .init_state::<state::GameState>()
        .add_plugins((
            overworld::OverworldPlugin,
            office::OfficePlugin,
            email::EmailPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
