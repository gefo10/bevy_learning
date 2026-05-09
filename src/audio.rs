use crate::collision::EnemyKilled;
use crate::game_state::GameState;
use bevy::prelude::*;

/// Holds pre-loaded audio handles so we're not calling asset_server.load()
/// on every frame.  This is the same Resource pattern as SpawnTimer — one-of
/// global data that multiple systems can read.
#[derive(Resource)]
struct SoundHandles {
    kill: Handle<AudioSource>,
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_sounds).add_systems(
            Update,
            play_kill_sound.run_if(in_state(GameState::Playing)),
        );
    }
}

/// Load handles once at startup.
/// asset_server.load() is cheap (just registers a path); the actual bytes
/// are streamed in the background.  Storing the Handle keeps the asset alive.
fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundHandles {
        kill: asset_server.load("audio/enemy_killed.ogg"),
    });
}

/// Spawn a one-shot audio entity for every EnemyKilled message this frame.
///
/// New concepts here:
///   • AudioPlayer   — a component that tells Bevy to play an audio source.
///   • PlaybackSettings::DESPAWN — play once, then automatically despawn
///     the entity.  No manual cleanup needed.
///   • Cloning a Handle is cheap — it's just a reference-counted ID.
fn play_kill_sound(
    mut commands: Commands,
    handles: Res<SoundHandles>,
    mut events: MessageReader<EnemyKilled>,
) {
    for _ in events.read() {
        commands.spawn((
            AudioPlayer::<AudioSource>::new(handles.kill.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}
