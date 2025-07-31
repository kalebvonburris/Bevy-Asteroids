//! Audio playback for the main song in the game.

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

/// Plays the main song of the game.
///
/// # Arguments
/// * `asset_server`: The `AssetServer` resource to load the sound asset.
/// * `audio`: The `Audio` resource to play the sound.
pub fn play_main_song(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("embedded://audio/Eternity.mp3"))
        .with_volume(0.7)
        .looped();
}
