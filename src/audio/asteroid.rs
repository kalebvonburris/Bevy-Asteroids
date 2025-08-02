//! Handles the audio for the bullet firing and striking an asteroid.

use bevy::prelude::*;

use crate::asteroid::AsteroidSize;

/// Plays a sound for destroying an asteroid based on its size.
///
/// # Arguments
/// * `commands`: The `Commands` resource to spawn the audio player entity.
/// * `size`: The size of the asteroid being destroyed.
/// * `asset_server`: The `AssetServer` resource to load the asteroid destruction sound asset.
pub fn destroy_asteroid(
    commands: &mut Commands,
    size: AsteroidSize,
    asset_server: &Res<AssetServer>,
) {
    let sound = match size {
        AsteroidSize::Large => "audio/asteroid_large_destruction.mp3",
        AsteroidSize::Medium => "audio/asteroid_medium_destruction.mp3",
        AsteroidSize::Small => "audio/asteroid_small_destruction.mp3",
    };

    // If found 0.75 to be pleasantly loud, but not too loud for these sounds.
    commands.spawn((
        AudioPlayer::new(asset_server.load(sound)),
        PlaybackSettings::REMOVE,
    ));
}
