//! Audio playback for the main song in the game.

use bevy::prelude::*;

/// Plays the main song of the game.
///
/// # Arguments
/// * `commands` - The Bevy commands to spawn the audio player.
/// * `asset_server` - The Bevy asset server to load the audio file.
pub fn play_main_song(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/Eternity.mp3")),
        PlaybackSettings::LOOP,
    ));
}
