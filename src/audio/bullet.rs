//! Handles the audio for the bullet firing and striking an asteroid.

use bevy::prelude::*;

/// Plays a sound for firing a bullet.
///
/// # Arguments
/// * `commands`: The `Commands` resource to spawn the audio player entity.
/// * `asset_server`: The `AssetServer` resource to load the bullet sound asset.
pub fn fire_bullet(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/laser.mp3")),
        PlaybackSettings::REMOVE,
    ));
}
