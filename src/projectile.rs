use std::collections::HashSet;

use crate::collision::EnemyKilled;
use crate::enemy::Enemy;
use crate::game_state::GameState;
use crate::movement::Velocity;
use crate::player::KineticPlayer;
use bevy::prelude::*;

const BULLET_SPEED: f32 = 700.0;
const BULLET_RADIUS: f32 = 5.0;
const FIRE_COOLDOWN_SECS: f32 = 0.15;
const BULLET_LIFETIME_SECS: f32 = 2.0;
const ENEMY_HALF: f32 = 15.0; // ENEMY_SIZE / 2

#[derive(Component)]
struct Bullet {
    lifetime: f32,
}

#[derive(Resource)]
struct FireCooldown(Timer);

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FireCooldown(Timer::from_seconds(
            FIRE_COOLDOWN_SECS,
            TimerMode::Once,
        )))
        .add_systems(
            Update,
            (shoot, tick_bullets, bullet_enemy_collision)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn shoot(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cooldown: ResMut<FireCooldown>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_q: Query<&Transform, With<KineticPlayer>>,
) {
    cooldown.0.tick(time.delta());
    if !keys.pressed(KeyCode::Space) || !cooldown.0.is_finished() {
        return;
    }

    let Ok(player_transform) = player_q.single() else { return };

    // Player faces local +Z rotated by its Y-rotation; that gives world shoot dir.
    let forward = player_transform.rotation * Vec3::Z;
    // Velocity convention: x→+X world, y→-Z world (see movement.rs)
    let vel_dir = Vec2::new(forward.x, -forward.z);
    if vel_dir.length_squared() < 0.001 {
        return;
    }

    cooldown.0.reset();

    let origin = player_transform.translation;
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(BULLET_RADIUS))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.9, 0.1),
            unlit: true,
            ..default()
        })),
        Transform::from_translation(Vec3::new(origin.x, BULLET_RADIUS, origin.z)),
        Velocity(vel_dir * BULLET_SPEED),
        Bullet {
            lifetime: BULLET_LIFETIME_SECS,
        },
    ));
}

fn tick_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in &mut bullets {
        bullet.lifetime -= time.delta_secs();
        if bullet.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn bullet_enemy_collision(
    mut commands: Commands,
    mut kill_events: MessageWriter<EnemyKilled>,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    let mut despawned_enemies: HashSet<Entity> = HashSet::new();

    for (bullet_e, b_t) in &bullets {
        let bp = b_t.translation;
        for (enemy_e, e_t) in &enemies {
            if despawned_enemies.contains(&enemy_e) {
                continue;
            }
            let ep = e_t.translation;
            if (bp.x - ep.x).abs() < ENEMY_HALF + BULLET_RADIUS
                && (bp.z - ep.z).abs() < ENEMY_HALF + BULLET_RADIUS
            {
                commands.entity(bullet_e).despawn();
                commands.entity(enemy_e).despawn();
                despawned_enemies.insert(enemy_e);
                kill_events.write(EnemyKilled);
                break;
            }
        }
    }
}
