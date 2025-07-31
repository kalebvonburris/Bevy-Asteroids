//! Handles the audio for the bullet firing and striking an asteroid.

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::asteroid::AsteroidSize;

use super::channels::ExplosionChannel;

/// Plays a sound for destroying an asteroid based on its size.
///
/// # Arguments
/// * `size`: The size of the asteroid being destroyed.
/// * `asset_server`: The `AssetServer` resource to load the sound asset.
/// * `audio`: The `AudioChannel<ExplosionChannel>` resource to play the sound.
pub fn destroy_asteroid(
    size: AsteroidSize,
    asset_server: &Res<AssetServer>,
    audio: &Res<AudioChannel<ExplosionChannel>>,
) {
    let sound = match size {
        AsteroidSize::Large => "embedded://audio/asteroid_large_destruction.mp3",
        AsteroidSize::Medium => "embedded://audio/asteroid_medium_destruction.mp3",
        AsteroidSize::Small => "embedded://audio/asteroid_small_destruction.mp3",
    };

    // If found 0.75 to be pleasantly loud, but not too loud for these sounds.
    audio.play(asset_server.load(sound)).with_volume(if audio.is_playing_sound() {
        0.35
    } else {
        0.5
    });
}
