use bevy::prelude::*;

const PLAYER_SIZE: f32 = 50.0;
const PLAYER_SPEED: f32 = 400.0;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.7, 0.9),
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    windows: Query<&Window>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    let dir = dir.normalize_or_zero();

    let Ok(window) = windows.single() else { return };
    let half_w = window.width() / 2.0 - PLAYER_SIZE / 2.0;
    let half_h = window.height() / 2.0 - PLAYER_SIZE / 2.0;

    for mut transform in &mut players {
        transform.translation.x += dir.x * PLAYER_SPEED * time.delta_secs();
        transform.translation.y += dir.y * PLAYER_SPEED * time.delta_secs();
        transform.translation.x = transform.translation.x.clamp(-half_w, half_w);
        transform.translation.y = transform.translation.y.clamp(-half_h, half_h);
    }
}
