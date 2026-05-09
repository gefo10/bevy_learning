use crate::enemy::Enemy;
use crate::movement::Velocity;
use crate::player::{Health, KineticPlayer, MAX_HEALTH};
use crate::score::Score;
use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Component)]
struct GameOverText;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::GameOver), show_game_over_text)
            .add_systems(OnExit(GameState::GameOver), hide_game_over_text)
            .add_systems(Update, restart.run_if(in_state(GameState::GameOver)));
    }
}

fn show_game_over_text(mut commands: Commands) {
    commands.spawn((
        Text::new("GAME OVER\nPress R to restart"),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            left: Val::Percent(30.0),
            width: Val::Percent(40.0),
            ..default()
        },
        GameOverText,
    ));
}

fn hide_game_over_text(mut commands: Commands, q: Query<Entity, With<GameOverText>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn restart(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    enemies: Query<Entity, With<Enemy>>,
    mut players: Query<(&mut Transform, &mut Health, &mut Velocity), With<KineticPlayer>>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    score.0 = 0;

    for e in &enemies {
        commands.entity(e).despawn();
    }

    for (mut t, mut h, mut v) in &mut players {
        t.translation = Vec3::ZERO;
        h.0 = MAX_HEALTH;
        v.0 = Vec2::ZERO;
    }

    next_state.set(GameState::Playing);
}
