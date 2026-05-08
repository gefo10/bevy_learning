mod enemy;
mod movement;
mod player;

use bevy::prelude::*;
use enemy::EnemyPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((MovementPlugin, PlayerPlugin, EnemyPlugin))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
