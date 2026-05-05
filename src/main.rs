use bevy::prelude::*;
use rand::Rng;

const PLAYER_SIZE: f32 = 50.0;
const PLAYER_SPEED: f32 = 400.0;
const ACCEL: f32 = 2000.0;
const FRICTION: f32 = 4.0;
const MAX_SPEED: f32 = 400.0;

const ENEMY_SIZE: f32 = 30.0;
const ENEMY_SPEED: f32 = 200.0;
const SPAWN_INTERVAL_SECS: f32 = 1.0;

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerV;

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct SpawnTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SpawnTimer(Timer::from_seconds(
            SPAWN_INTERVAL_SECS,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                (accelerate_player, apply_velocity, clamp_to_window).chain(),
                spawn_enemies,
                despawn_offscreen_enemies,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.7, 0.9),
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.5),
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        },
        Transform::default(),
        Velocity::default(),
        PlayerV,
    ));
}

fn accelerate_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q: Query<&mut Velocity, With<PlayerV>>,
) {
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    };
    if keys.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }
    let dir = dir.normalize_or_zero();

    for mut vel in &mut q {
        vel.0 += dir * ACCEL * time.delta_secs();
        vel.0 *= 1.0 - FRICTION * time.delta_secs();
        vel.0 = vel.0.clamp_length_max(MAX_SPEED);
    }
}

// Velocity moves position.
fn apply_velocity(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut transform) in &mut q {
        transform.translation.x += vel.0.x * time.delta_secs();
        transform.translation.y += vel.0.y * time.delta_secs();
    }
}

fn clamp_to_window(
    windows: Query<&Window>,
    mut q: Query<(&mut Transform, &mut Velocity), With<PlayerV>>,
) {
    let Ok(window) = windows.single() else { return };
    let half_w = window.width() / 2.0 - PLAYER_SIZE / 2.0;
    let half_h = window.height() / 2.0 - PLAYER_SIZE / 2.0;

    for (mut transform, mut vel) in &mut q {
        let p = &mut transform.translation;
        if p.x < -half_w {
            p.x = -half_w;
            vel.0.x = vel.0.x.max(0.0);
        } else if p.x > half_w {
            p.x = half_w;
            vel.0.x = vel.0.x.min(0.0);
        }
        if p.y < -half_h {
            p.y = -half_h;
            vel.0.y = vel.0.y.max(0.0);
        } else if p.y > half_h {
            p.y = half_h;
            vel.0.y = vel.0.y.min(0.0);
        }
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

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    windows: Query<&Window>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    let dir = dir.normalize_or_zero();

    let Ok(window) = windows.single() else { return };
    let half_w = window.width() / 2.0 - PLAYER_SIZE / 2.0;
    let half_h = window.height() / 2.0 - PLAYER_SIZE / 2.0;

    for mut transform in &mut players {
        transform.translation.x += dir.x * PLAYER_SPEED * time.delta_secs();
        transform.translation.y += dir.y * PLAYER_SPEED * time.delta_secs();
        transform.translation.x = transform.translation.x.clamp(-half_w, half_w);
        transform.translation.y = transform.translation.y.clamp(-half_h, half_h);
    }
}
