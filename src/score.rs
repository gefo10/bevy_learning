use crate::collision::EnemyKilled;
use crate::game_state::GameState;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Component)]
struct ScoreText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, spawn_score_text)
            .add_systems(
                Update,
                (
                    increment_score.run_if(in_state(GameState::Playing)),
                    update_score_text,
                ),
            );
    }
}

fn spawn_score_text(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ScoreText,
    ));
}

fn increment_score(mut score: ResMut<Score>, mut events: MessageReader<EnemyKilled>) {
    for _ in events.read() {
        score.0 += 1;
    }
}

fn update_score_text(score: Res<Score>, mut text: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }
    let Ok(mut text) = text.single_mut() else {
        return;
    };
    text.0 = format!("Score: {}", score.0);
}
