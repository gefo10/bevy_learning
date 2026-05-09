use crate::player::{Health, KineticPlayer, MAX_HEALTH};
use bevy::prelude::*;

const ACTIVE: Color = Color::srgb(0.8, 0.2, 0.5);
const EMPTY: Color = Color::srgb(0.2, 0.2, 0.2);
const PIP_SIZE: f32 = 18.0;
const PIP_GAP: f32 = 4.0;

#[derive(Component)]
struct HealthPip(i32);

pub struct HealthUiPlugin;

impl Plugin for HealthUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_health_ui)
            .add_systems(Update, update_health_ui);
    }
}

fn spawn_health_ui(mut commands: Commands) {
    for i in 0..MAX_HEALTH {
        let left = 10.0 + i as f32 * (PIP_SIZE + PIP_GAP);
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(left),
                width: Val::Px(PIP_SIZE),
                height: Val::Px(PIP_SIZE),
                border_radius: BorderRadius::all(Val::Px(3.0)),
                ..default()
            },
            HealthPip(i),
        ));
    }
}

fn update_health_ui(
    player: Query<&Health, With<KineticPlayer>>,
    mut pips: Query<(&HealthPip, &mut BackgroundColor)>,
) {
    let hp = player.single().map(|h| h.0).unwrap_or(0);
    for (pip, mut bg) in &mut pips {
        bg.0 = if pip.0 < hp { ACTIVE } else { EMPTY };
    }
}
