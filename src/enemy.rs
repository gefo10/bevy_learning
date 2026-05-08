use crate::movement::Velocity;
use bevy::prelude::*;
use rand::Rng;

const ENEMY_SIZE: f32 = 30.0;
const ENEMY_SPEED: f32 = 200.0;
const SPAWN_INTERVAL_SECS: f32 = 1.0;

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
        .add_systems(Update, (spawn_enemies, despawn_offscreen_enemies));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    windows: Query<&Window>,
) {
    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        return;
    }

    let Ok(window) = windows.single() else { return };
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    let mut rng = rand::rng();

    // Pick an edge: 0=top, 1=bottom, 2=left, 3=right.
    let (pos, vel) = match rng.random_range(0..4) {
        0 => (
            Vec2::new(rng.random_range(-half_w..half_w), half_h + ENEMY_SIZE),
            Vec2::new(0.0, -ENEMY_SPEED),
        ),
        1 => (
            Vec2::new(rng.random_range(-half_w..half_w), -half_h - ENEMY_SIZE),
            Vec2::new(0.0, ENEMY_SPEED),
        ),
        2 => (
            Vec2::new(-half_w - ENEMY_SIZE, rng.random_range(-half_h..half_h)),
            Vec2::new(ENEMY_SPEED, 0.0),
        ),
        _ => (
            Vec2::new(half_w + ENEMY_SIZE, rng.random_range(-half_h..half_h)),
            Vec2::new(-ENEMY_SPEED, 0.0),
        ),
    };

    commands.spawn((
        Sprite {
            color: Color::srgb(
                rng.random_range(0.5..1.0),
                rng.random_range(0.2..0.6),
                rng.random_range(0.1..0.4),
            ),
            custom_size: Some(Vec2::splat(ENEMY_SIZE)),
            ..default()
        },
        Transform::from_xyz(pos.x, pos.y, 0.0),
        Velocity(vel),
        Enemy,
    ));
}

fn despawn_offscreen_enemies(
    mut commands: Commands,
    windows: Query<&Window>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    let Ok(window) = windows.single() else { return };
    let limit_x = window.width() / 2.0 + ENEMY_SIZE * 2.0;
    let limit_y = window.height() / 2.0 + ENEMY_SIZE * 2.0;

    for (entity, transform) in &enemies {
        let p = transform.translation;
        if p.x.abs() > limit_x || p.y.abs() > limit_y {
            commands.entity(entity).despawn();
        }
    }
}
