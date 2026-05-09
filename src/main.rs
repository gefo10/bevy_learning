mod audio;
mod collision;
mod enemy;
mod game_state;
mod health_ui;
mod movement;
mod player;
mod score;

use audio::GameAudioPlugin;
use bevy::prelude::*;
use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use game_state::GameStatePlugin;
use health_ui::HealthUiPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            GameStatePlugin,
            MovementPlugin,
            PlayerPlugin,
            EnemyPlugin,
            CollisionPlugin,
            ScorePlugin,
            HealthUiPlugin,
            GameAudioPlugin,
        ))
        .add_systems(Startup, (spawn_camera, spawn_ground))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 900.0, 650.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2000.0, 2000.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.12, 0.15),
            unlit: true,
            ..default()
        })),
    ));
}
