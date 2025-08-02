//! Handles the audio for the bullet firing and striking an asteroid.

use bevy::prelude::*;

/// Plays a sound for the ship being hit.
///
/// # Arguments
/// * `commands` - The Bevy commands to spawn the audio player.
/// * `asset_server` - The Bevy asset server to load the audio file.
pub fn ship_hit(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/player_hit.mp3")),
        PlaybackSettings::REMOVE,
    ));
}

/// Handles the audio for the ship being destroyed.
///
/// # Arguments
/// * `commands` - The Bevy commands to spawn the audio player.
/// * `asset_server` - The Bevy asset server to load the audio file.
pub fn ship_destroyed(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/player_destroyed.mp3")),
        PlaybackSettings::REMOVE,
    ));
}
