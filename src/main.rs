mod audio;
mod collision;
mod enemy;
mod game_state;
mod health_ui;
mod movement;
mod player;
mod projectile;
mod score;

use audio::GameAudioPlugin;
use bevy::prelude::*;
use collision::{CollisionPlugin, PlayerHit};
use enemy::EnemyPlugin;
use game_state::GameStatePlugin;
use health_ui::HealthUiPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use score::ScorePlugin;

const CAMERA_BASE: Vec3 = Vec3::new(0.0, 900.0, 650.0);

#[derive(Resource, Default)]
struct CameraShake {
    trauma: f32,
}

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
            ProjectilePlugin,
        ))
        .init_resource::<CameraShake>()
        .add_systems(Startup, (spawn_camera, spawn_ground))
        .add_systems(Update, camera_shake)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(CAMERA_BASE).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Sun from above-front so characters are front-lit from the camera's perspective
    commands.spawn((
        DirectionalLight {
            illuminance: 15_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
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

fn camera_shake(
    time: Res<Time>,
    mut shake: ResMut<CameraShake>,
    mut events: MessageReader<PlayerHit>,
    mut camera_q: Query<&mut Transform, With<Camera3d>>,
) {
    for _ in events.read() {
        shake.trauma = (shake.trauma + 1.0).min(1.0);
    }

    let Ok(mut transform) = camera_q.single_mut() else {
        return;
    };

    if shake.trauma <= 0.0 {
        transform.translation = CAMERA_BASE;
        return;
    }

    shake.trauma = (shake.trauma - time.delta_secs() * 3.0).max(0.0);
    let intensity = shake.trauma * shake.trauma;
    let t = time.elapsed_secs();
    let offset = Vec3::new(
        (t * 31.0).sin() * 35.0 * intensity,
        (t * 19.0).cos() * 15.0 * intensity,
        (t * 23.0).sin() * 20.0 * intensity,
    );
    transform.translation = CAMERA_BASE + offset;
}
