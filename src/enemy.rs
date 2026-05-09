use crate::collision::Hitbox;
use crate::game_state::GameState;
use crate::movement::Velocity;
use crate::player::KineticPlayer;
use bevy::prelude::*;
use rand::Rng;

const ENEMY_SIZE: f32 = 30.0;
const ENEMY_SPEED: f32 = 200.0;
const SPAWN_INTERVAL_SECS: f32 = 1.0;
const SPAWN_DIST: f32 = 850.0;
const DESPAWN_DIST: f32 = 1800.0;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
struct SpawnTimer(Timer);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(
            SPAWN_INTERVAL_SECS,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (spawn_enemies, track_player, despawn_far_enemies)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_q: Query<&Transform, With<KineticPlayer>>,
) {
    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        return;
    }

    let Ok(player_transform) = player_q.single() else { return };
    let pp = player_transform.translation;

    let mut rng = rand::rng();
    let angle = rng.random_range(0.0..std::f32::consts::TAU);
    let x = pp.x + angle.cos() * SPAWN_DIST;
    let z = pp.z + angle.sin() * SPAWN_DIST;
    let half = ENEMY_SIZE / 2.0;

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(ENEMY_SIZE, ENEMY_SIZE, ENEMY_SIZE))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(
                rng.random_range(0.5..1.0),
                rng.random_range(0.2..0.6),
                rng.random_range(0.1..0.4),
            ),
            ..default()
        })),
        Transform::from_translation(Vec3::new(x, half, z)),
        Velocity(Vec2::ZERO),
        Hitbox(Vec2::splat(ENEMY_SIZE / 2.0)),
        Enemy,
    ));
}

fn track_player(
    player_q: Query<&Transform, With<KineticPlayer>>,
    mut enemies: Query<(&Transform, &mut Velocity), With<Enemy>>,
) {
    let Ok(player) = player_q.single() else { return };
    let pp = player.translation;

    for (e_transform, mut vel) in &mut enemies {
        let ep = e_transform.translation;
        let delta = Vec2::new(pp.x - ep.x, pp.z - ep.z);
        let dir = delta.normalize_or_zero();
        // Velocity convention: x→+X world, y→-Z world (see movement.rs)
        vel.0 = Vec2::new(dir.x, -dir.y) * ENEMY_SPEED;
    }
}

fn despawn_far_enemies(
    mut commands: Commands,
    player_q: Query<&Transform, With<KineticPlayer>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    let Ok(player) = player_q.single() else { return };
    let pp = player.translation;

    for (entity, transform) in &enemies {
        let ep = transform.translation;
        let dist_sq = (ep.x - pp.x).powi(2) + (ep.z - pp.z).powi(2);
        if dist_sq > DESPAWN_DIST * DESPAWN_DIST {
            commands.entity(entity).despawn();
        }
    }
}
