use crate::enemy::Enemy;
use crate::player::{DirectPlayer, KineticPlayer};
use bevy::prelude::*;

#[derive(Component)]
pub struct Hitbox(pub Vec2); // half-extents (width/2, height/2)

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_player_enemy_collisions);
    }
}

fn detect_player_enemy_collisions(
    mut commands: Commands,
    players: Query<(&Transform, &Hitbox), Or<(With<DirectPlayer>, With<KineticPlayer>)>>,
    enemies: Query<(Entity, &Transform, &Hitbox), With<Enemy>>,
) {
    for (p_transform, p_hitbox) in &players {
        for (enemy_id, e_transform, e_hitbox) in &enemies {
            let delta = p_transform.translation.truncate() - e_transform.translation.truncate();
            let combined = p_hitbox.0 + e_hitbox.0;
            if delta.x.abs() < combined.x && delta.y.abs() < combined.y {
                commands.entity(enemy_id).despawn();
            }
        }
    }
}
