use crate::collision::Hitbox;
use crate::game_state::GameState;
use crate::movement::Velocity;
use bevy::prelude::*;
use rand::Rng;

const ENEMY_SIZE: f32 = 30.0;
const ENEMY_SPEED: f32 = 200.0;
const SPAWN_INTERVAL_SECS: f32 = 1.0;

const WORLD_HALF_W: f32 = 640.0;
const WORLD_HALF_H: f32 = 360.0;

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
            (spawn_enemies, despawn_offscreen_enemies).run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        return;
    }

    let mut rng = rand::rng();
    let mesh = meshes.add(Rectangle::new(ENEMY_SIZE, ENEMY_SIZE));
    let flat_rot = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);

    // Pick an edge: 0=top(-Z), 1=bottom(+Z), 2=left(-X), 3=right(+X)
    let (pos, vel) = match rng.random_range(0..4) {
        0 => (
            Vec3::new(rng.random_range(-WORLD_HALF_W..WORLD_HALF_W), 0.5, -WORLD_HALF_H - ENEMY_SIZE),
            Vec2::new(0.0, -ENEMY_SPEED),
        ),
        1 => (
            Vec3::new(rng.random_range(-WORLD_HALF_W..WORLD_HALF_W), 0.5, WORLD_HALF_H + ENEMY_SIZE),
            Vec2::new(0.0, ENEMY_SPEED),
        ),
        2 => (
            Vec3::new(-WORLD_HALF_W - ENEMY_SIZE, 0.5, rng.random_range(-WORLD_HALF_H..WORLD_HALF_H)),
            Vec2::new(ENEMY_SPEED, 0.0),
        ),
        _ => (
            Vec3::new(WORLD_HALF_W + ENEMY_SIZE, 0.5, rng.random_range(-WORLD_HALF_H..WORLD_HALF_H)),
            Vec2::new(-ENEMY_SPEED, 0.0),
        ),
    };

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(
                rng.random_range(0.5..1.0),
                rng.random_range(0.2..0.6),
                rng.random_range(0.1..0.4),
            ),
            unlit: true,
            ..default()
        })),
        Transform::from_translation(pos).with_rotation(flat_rot),
        Velocity(vel),
        Hitbox(Vec2::splat(ENEMY_SIZE / 2.0)),
        Enemy,
    ));
}

fn despawn_offscreen_enemies(
    mut commands: Commands,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    let limit_x = WORLD_HALF_W + ENEMY_SIZE * 2.0;
    let limit_z = WORLD_HALF_H + ENEMY_SIZE * 2.0;

    for (entity, transform) in &enemies {
        let p = transform.translation;
        if p.x.abs() > limit_x || p.z.abs() > limit_z {
            commands.entity(entity).despawn();
        }
    }
}
