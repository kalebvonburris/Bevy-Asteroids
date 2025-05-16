//! Handles the audio for the bullet firing and striking an asteroid.

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::asteroid::AsteroidSize;

use super::channels::ExplosionChannel;

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

    audio.play(asset_server.load(sound)).with_volume(0.75);
}
