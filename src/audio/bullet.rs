//! Handles the audio for the bullet firing and striking an asteroid.

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::channels::LaserChannel;

/// Plays a sound for firing a bullet.
///
/// # Arguments
/// * `asset_server`: The `AssetServer` resource to load the sound asset.
/// * `audio`: The `AudioChannel<LaserChannel>` resource to play the sound.
pub fn fire_bullet(asset_server: &Res<AssetServer>, audio: &Res<AudioChannel<LaserChannel>>) {
    audio
        .play(asset_server.load("embedded://audio/laser.mp3"))
        .with_volume(0.35);
}
