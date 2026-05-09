use crate::enemy::Enemy;
use crate::game_state::GameState;
use crate::player::{Health, KineticPlayer};
use bevy::prelude::*;

#[derive(Component)]
pub struct Hitbox(pub Vec2); // half-extents (width/2, height/2)

#[derive(Message)]
pub struct EnemyKilled;

#[derive(Message)]
pub struct PlayerHit;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EnemyKilled>()
            .add_message::<PlayerHit>()
            .add_systems(
            Update,
            detect_player_enemy_collisions.run_if(in_state(GameState::Playing)),
        );
    }
}

fn detect_player_enemy_collisions(
    mut commands: Commands,
    mut kill_events: MessageWriter<EnemyKilled>,
    mut hit_events: MessageWriter<PlayerHit>,
    mut next_state: ResMut<NextState<GameState>>,
    mut players: Query<(&Transform, &Hitbox, &mut Health), With<KineticPlayer>>,
    enemies: Query<(Entity, &Transform, &Hitbox), With<Enemy>>,
) {
    for (p_transform, p_hitbox, mut p_health) in &mut players {
        for (enemy_id, e_transform, e_hitbox) in &enemies {
            let delta = p_transform.translation.xz() - e_transform.translation.xz();
            let combined = p_hitbox.0 + e_hitbox.0;
            if delta.x.abs() < combined.x && delta.y.abs() < combined.y {
                commands.entity(enemy_id).despawn();
                kill_events.write(EnemyKilled);
                hit_events.write(PlayerHit);
                p_health.0 -= 1;
                if p_health.0 <= 0 {
                    next_state.set(GameState::GameOver);
                }
            }
        }
    }
}
