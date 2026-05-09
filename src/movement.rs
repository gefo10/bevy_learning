use crate::game_state::GameState;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            apply_velocity.run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn apply_velocity(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut transform) in &mut q {
        transform.translation.x += vel.0.x * time.delta_secs();
        // vel.0.y maps to -Z: positive Y (screen up) = negative Z (away from camera)
        transform.translation.z -= vel.0.y * time.delta_secs();
    }
}
