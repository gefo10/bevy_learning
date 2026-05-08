use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity);
    }
}

pub fn apply_velocity(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut transform) in &mut q {
        transform.translation.x += vel.0.x * time.delta_secs();
        transform.translation.y += vel.0.y * time.delta_secs();
    }
}
