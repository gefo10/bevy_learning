mod audio;
mod collision;
mod enemy;
mod game_state;
mod health_ui;
mod movement;
mod player;
mod projectile;
mod props;
mod score;

use audio::GameAudioPlugin;
use bevy::prelude::*;
use collision::{CollisionPlugin, PlayerHit};
use enemy::EnemyPlugin;
use game_state::GameStatePlugin;
use health_ui::HealthUiPlugin;
use movement::MovementPlugin;
use player::{KineticPlayer, PlayerPlugin};
use projectile::ProjectilePlugin;
use props::PropsPlugin;
use score::ScorePlugin;

// Fixed offset from player to camera in world space
const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 900.0, 650.0);

#[derive(Resource)]
struct CameraState {
    trauma: f32,
    smooth_pos: Vec3,
}

impl Default for CameraState {
    fn default() -> Self {
        CameraState {
            trauma: 0.0,
            smooth_pos: CAMERA_OFFSET,
        }
    }
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
            PropsPlugin,
        ))
        .init_resource::<CameraState>()
        .add_systems(Startup, (spawn_camera, spawn_ground))
        .add_systems(Update, camera_system)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
    ));
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
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20_000.0, 20_000.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.12, 0.15),
            unlit: true,
            ..default()
        })),
    ));
}

fn camera_system(
    time: Res<Time>,
    mut state: ResMut<CameraState>,
    mut hit_events: MessageReader<PlayerHit>,
    player_q: Query<&Transform, With<KineticPlayer>>,
    mut camera_q: Query<&mut Transform, (With<Camera3d>, Without<KineticPlayer>)>,
) {
    for _ in hit_events.read() {
        state.trauma = (state.trauma + 1.0).min(1.0);
    }

    let Ok(player) = player_q.single() else { return };
    let Ok(mut cam) = camera_q.single_mut() else { return };

    // Exponential smoothing toward player — framerate-independent lag
    let alpha = 1.0 - (-5.0 * time.delta_secs()).exp();
    let target = player.translation + CAMERA_OFFSET;
    state.smooth_pos = state.smooth_pos.lerp(target, alpha);

    // Trauma-squared shake decaying over ~0.33s
    state.trauma = (state.trauma - time.delta_secs() * 3.0).max(0.0);
    let intensity = state.trauma * state.trauma;
    let t = time.elapsed_secs();
    let shake = Vec3::new(
        (t * 31.0).sin() * 35.0 * intensity,
        (t * 19.0).cos() * 15.0 * intensity,
        (t * 23.0).sin() * 20.0 * intensity,
    );

    cam.translation = state.smooth_pos + shake;
    cam.look_at(player.translation, Vec3::Y);
}
