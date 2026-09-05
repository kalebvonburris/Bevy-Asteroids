//! Handles the audio for an asteroid being destroyed.

use bevy::audio::AudioSource;
use bevy::prelude::*;

use crate::{asteroid::AsteroidSize, audio::play_sound, explosion::Explosion};

/// The destruction sound for each [`AsteroidSize`].
#[derive(Resource)]
pub struct AsteroidSounds {
    pub small: Handle<AudioSource>,
    pub medium: Handle<AudioSource>,
    pub large: Handle<AudioSource>,
}

/// Loads the asteroid destruction sounds into [`AsteroidSounds`].
///
/// # Arguments
/// * `asset_server`: The `AssetServer` resource to load the audio files.
/// * `commands`: The `Commands` resource to insert the loaded handles.
pub fn preload_asteroid_sounds(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(AsteroidSounds {
        small: asset_server.load("audio/asteroid_small_destruction.mp3"),
        medium: asset_server.load("audio/asteroid_medium_destruction.mp3"),
        large: asset_server.load("audio/asteroid_large_destruction.mp3"),
    });
}

/// Plays the destruction sound matching the size of whatever just exploded.
///
/// # Arguments
/// * `explosion`: The `Explosion` event that triggered this observer.
/// * `sounds`: The `AsteroidSounds` resource holding the clips.
/// * `commands`: The `Commands` resource to spawn the audio player entity.
pub fn play_explosion_sound(
    explosion: On<Explosion>,
    sounds: Res<AsteroidSounds>,
    mut commands: Commands,
) {
    let sound = match explosion.size {
        AsteroidSize::Small => sounds.small.clone(),
        AsteroidSize::Medium => sounds.medium.clone(),
        AsteroidSize::Large => sounds.large.clone(),
    };

    play_sound(&mut commands, sound);
}
