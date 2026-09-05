//! Handles the audio for the player ship being hit and destroyed.

use bevy::audio::AudioSource;
use bevy::prelude::*;

/// The sounds played when the player ship takes damage or dies.
#[derive(Resource)]
pub struct PlayerSounds {
    pub hit: Handle<AudioSource>,
    pub destroyed: Handle<AudioSource>,
}

/// Loads the player ship sounds into [`PlayerSounds`].
///
/// # Arguments
/// * `asset_server`: The `AssetServer` resource to load the audio files.
/// * `commands`: The `Commands` resource to insert the loaded handles.
pub fn preload_player_sounds(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(PlayerSounds {
        hit: asset_server.load("audio/player_hit.mp3"),
        // The ship going up reuses the large asteroid's destruction sound.
        destroyed: asset_server.load("audio/asteroid_large_destruction.mp3"),
    });
}
