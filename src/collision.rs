use crate::enemy::Enemy;
use crate::game_state::GameState;
use crate::player::{DirectPlayer, Health, KineticPlayer};
use bevy::prelude::*;

#[derive(Component)]
pub struct Hitbox(pub Vec2); // half-extents (width/2, height/2)

#[derive(Message)]
pub struct EnemyKilled;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EnemyKilled>().add_systems(
            Update,
            detect_player_enemy_collisions.run_if(in_state(GameState::Playing)),
        );
    }
}

fn detect_player_enemy_collisions(
    mut commands: Commands,
    mut events: MessageWriter<EnemyKilled>,
    mut next_state: ResMut<NextState<GameState>>,
    mut players: Query<
        (&Transform, &Hitbox, &mut Health),
        Or<(With<DirectPlayer>, With<KineticPlayer>)>,
    >,
    enemies: Query<(Entity, &Transform, &Hitbox), With<Enemy>>,
) {
    for (p_transform, p_hitbox, mut p_health) in &mut players {
        for (enemy_id, e_transform, e_hitbox) in &enemies {
            let delta = p_transform.translation.truncate() - e_transform.translation.truncate();
            let combined = p_hitbox.0 + e_hitbox.0;
            if delta.x.abs() < combined.x && delta.y.abs() < combined.y {
                commands.entity(enemy_id).despawn();
                events.write(EnemyKilled);
                p_health.0 -= 1;
                if p_health.0 <= 0 {
                    next_state.set(GameState::GameOver);
                }
            }
        }
    }
}
