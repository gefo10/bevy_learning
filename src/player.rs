use crate::collision::Hitbox;
use crate::game_state::GameState;
use crate::movement::{Velocity, apply_velocity};
use bevy::prelude::*;

const PLAYER_SIZE: f32 = 50.0;
const PLAYER_SPEED: f32 = 400.0;
const ACCEL: f32 = 2000.0;
const FRICTION: f32 = 4.0;
const MAX_SPEED: f32 = 400.0;
const CHARACTER_SCALE: f32 = 0.15;

const WORLD_HALF_W: f32 = 640.0;
const WORLD_HALF_H: f32 = 360.0;

pub const MAX_HEALTH: i32 = 3;

#[derive(Component)]
pub struct DirectPlayer;

#[derive(Component)]
pub struct KineticPlayer;

#[derive(Component)]
pub struct Health(pub i32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players).add_systems(
            Update,
            (
                move_direct_player,
                accelerate_kinetic_player.before(apply_velocity),
                clamp_kinetic_player.after(apply_velocity),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn character_transform(x: f32, z: f32) -> Transform {
    Transform::from_xyz(x, 0.0, z).with_scale(Vec3::splat(CHARACTER_SCALE))
}

fn spawn_players(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene: Handle<Scene> = asset_server.load("characters/characterMedium.glb#Scene0");

    commands.spawn((
        SceneRoot(scene.clone()),
        character_transform(-40.0, 0.0),
        Hitbox(Vec2::splat(PLAYER_SIZE / 2.0)),
        Health(MAX_HEALTH),
        DirectPlayer,
    ));

    commands.spawn((
        SceneRoot(scene),
        character_transform(40.0, 0.0),
        Velocity::default(),
        Hitbox(Vec2::splat(PLAYER_SIZE / 2.0)),
        Health(MAX_HEALTH),
        KineticPlayer,
    ));
}

fn move_direct_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut players: Query<&mut Transform, With<DirectPlayer>>,
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

    for mut transform in &mut players {
        transform.translation.x += dir.x * PLAYER_SPEED * time.delta_secs();
        transform.translation.z -= dir.y * PLAYER_SPEED * time.delta_secs();
        transform.translation.x = transform.translation.x.clamp(-WORLD_HALF_W, WORLD_HALF_W);
        transform.translation.z = transform.translation.z.clamp(-WORLD_HALF_H, WORLD_HALF_H);
    }
}

fn accelerate_kinetic_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q: Query<&mut Velocity, With<KineticPlayer>>,
) {
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
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

fn clamp_kinetic_player(mut q: Query<(&mut Transform, &mut Velocity), With<KineticPlayer>>) {
    for (mut transform, mut vel) in &mut q {
        let p = &mut transform.translation;
        if p.x < -WORLD_HALF_W {
            p.x = -WORLD_HALF_W;
            vel.0.x = vel.0.x.max(0.0);
        } else if p.x > WORLD_HALF_W {
            p.x = WORLD_HALF_W;
            vel.0.x = vel.0.x.min(0.0);
        }
        if p.z > WORLD_HALF_H {
            p.z = WORLD_HALF_H;
            vel.0.y = vel.0.y.max(0.0);
        } else if p.z < -WORLD_HALF_H {
            p.z = -WORLD_HALF_H;
            vel.0.y = vel.0.y.min(0.0);
        }
    }
}
