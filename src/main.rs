mod collision;
mod enemy;
mod movement;
mod player;

use bevy::prelude::*;
use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((MovementPlugin, PlayerPlugin, EnemyPlugin, CollisionPlugin))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
