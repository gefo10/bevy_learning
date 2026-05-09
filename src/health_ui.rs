use crate::player::{DirectPlayer, Health, KineticPlayer, MAX_HEALTH};
use bevy::prelude::*;

// Pip colours match the two player sprite colours so they're self-labelling.
const DIRECT_ACTIVE: Color = Color::srgb(0.3, 0.7, 0.9); // same blue as DirectPlayer
const KINETIC_ACTIVE: Color = Color::srgb(0.8, 0.2, 0.5); // same pink as KineticPlayer
const EMPTY: Color = Color::srgb(0.2, 0.2, 0.2);

const PIP_SIZE: f32 = 18.0;
const PIP_GAP: f32 = 4.0;

/// One coloured square in a health bar.
/// `player_index`: 0 = DirectPlayer, 1 = KineticPlayer.
/// `pip_index`: which HP pip this represents (0-based).
#[derive(Component)]
struct HealthPip {
    player_index: u8,
    pip_index: i32,
}

pub struct HealthUiPlugin;

impl Plugin for HealthUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_health_ui)
            .add_systems(Update, update_health_ui);
    }
}

fn spawn_health_ui(mut commands: Commands) {
    // Two rows: DirectPlayer at top:50, KineticPlayer at top:80.
    // Each pip is positioned absolutely — no parent container needed.
    // The pip colours identify which bar belongs to which player.
    let bars: [(u8, f32); 2] = [(0, 50.0), (1, 80.0)];

    for (player_index, top) in bars {
        for i in 0..MAX_HEALTH {
            let left = 10.0 + i as f32 * (PIP_SIZE + PIP_GAP);

            // BackgroundColor is a *required component* of Node in Bevy 0.18,
            // so we must NOT include it in the spawn bundle (duplicate → not a
            // valid Bundle).  update_health_ui sets the colour every frame.
            //
            // BorderRadius is a *field* of Node (not a component), so it lives
            // inside the Node struct literal.
            commands.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(top),
                    left: Val::Px(left),
                    width: Val::Px(PIP_SIZE),
                    height: Val::Px(PIP_SIZE),
                    border_radius: BorderRadius::all(Val::Px(3.0)),
                    ..default()
                },
                HealthPip {
                    player_index,
                    pip_index: i,
                },
            ));
        }
    }
}

/// Runs every frame; cheap because health rarely changes.
///
/// Key concept: we're reading from a *component* on a game entity (Health on
/// the player) rather than a *resource* (like Score).  Same change-driven
/// update pattern, different data source.
fn update_health_ui(
    direct_q: Query<&Health, With<DirectPlayer>>,
    kinetic_q: Query<&Health, With<KineticPlayer>>,
    mut pips: Query<(&HealthPip, &mut BackgroundColor)>,
) {
    let direct_hp = direct_q.single().map(|h| h.0).unwrap_or(0);
    let kinetic_hp = kinetic_q.single().map(|h| h.0).unwrap_or(0);

    for (pip, mut bg) in &mut pips {
        let hp = if pip.player_index == 0 {
            direct_hp
        } else {
            kinetic_hp
        };
        let active = if pip.player_index == 0 {
            DIRECT_ACTIVE
        } else {
            KINETIC_ACTIVE
        };
        bg.0 = if pip.pip_index < hp { active } else { EMPTY };
    }
}
