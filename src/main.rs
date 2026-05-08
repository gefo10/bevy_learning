mod collision;
mod enemy;
mod game_state;
mod movement;
mod player;
mod score;

use bevy::prelude::*;
use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use game_state::GameStatePlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            GameStatePlugin,
            MovementPlugin,
            PlayerPlugin,
            EnemyPlugin,
            CollisionPlugin,
            ScorePlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
