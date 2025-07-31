//! Handles the audio for the bullet firing and striking an asteroid.

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::channels::ExplosionChannel;

/// Plays a sound for the ship being hit.
///
/// # Arguments
/// * `asset_server`: The `AssetServer` resource to load the sound asset.
/// * `audio`: The `AudioChannel<ExplosionChannel>` resource to play the sound.
pub fn ship_hit(asset_server: &Res<AssetServer>, audio: &Res<AudioChannel<ExplosionChannel>>) {
    audio
        .play(asset_server.load("embedded://audio/player_hit.mp3"))
        .with_volume(0.5);
}

/// Handles the audio for the ship being destroyed.
///
/// # Arguments
/// * `asset_server`: The `AssetServer` resource to load the sound asset.
/// * `audio`: The `AudioChannel<ExplosionChannel>` resource to play the sound.
pub fn ship_destroyed(
    asset_server: &Res<AssetServer>,
    audio: &Res<AudioChannel<ExplosionChannel>>,
) {
    audio
        .play(asset_server.load("embedded://audio/player_destroyed.mp3"))
        .with_volume(0.7);
}
