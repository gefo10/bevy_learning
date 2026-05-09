use crate::collision::Hitbox;
use crate::game_state::GameState;
use crate::movement::{Velocity, apply_velocity};
use bevy::prelude::*;

const PLAYER_SIZE: f32 = 50.0;
const ACCEL: f32 = 2000.0;
const FRICTION: f32 = 4.0;
const MAX_SPEED: f32 = 400.0;
const CHARACTER_SCALE: f32 = 0.15;

pub const MAX_HEALTH: i32 = 3;

#[derive(Component)]
pub struct KineticPlayer;

#[derive(Component)]
pub struct Health(pub i32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                accelerate_player.before(apply_velocity),
                rotate_player,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene: Handle<Scene> = asset_server.load("characters/characterMedium.glb#Scene0");
    commands.spawn((
        SceneRoot(scene),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(CHARACTER_SCALE)),
        Velocity::default(),
        Hitbox(Vec2::splat(PLAYER_SIZE / 2.0)),
        Health(MAX_HEALTH),
        KineticPlayer,
    ));
}

fn accelerate_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q: Query<&mut Velocity, With<KineticPlayer>>,
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

    for mut vel in &mut q {
        vel.0 += dir * ACCEL * time.delta_secs();
        vel.0 *= 1.0 - FRICTION * time.delta_secs();
        vel.0 = vel.0.clamp_length_max(MAX_SPEED);
    }
}

fn rotate_player(
    time: Res<Time>,
    mut q: Query<(&Velocity, &mut Transform), With<KineticPlayer>>,
) {
    for (vel, mut transform) in &mut q {
        if vel.0.length_squared() < 100.0 {
            continue;
        }
        // vel.x → +X in world; vel.y → -Z in world (see movement.rs)
        let angle = vel.0.x.atan2(-vel.0.y);
        let target = Quat::from_rotation_y(angle);
        transform.rotation = transform.rotation.slerp(target, 12.0 * time.delta_secs());
    }
}

