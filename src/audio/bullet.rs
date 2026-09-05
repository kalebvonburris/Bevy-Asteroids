//! Handles the audio for the ship firing a bullet.

use bevy::prelude::*;

/// The sound played when the ship fires a bullet.
#[derive(Resource)]
pub struct BulletAudio(pub Handle<AudioSource>);

/// Loads the bullet firing sound into [`BulletAudio`].
///
/// # Arguments
/// * `asset_server`: The `AssetServer` resource to load the audio file.
/// * `commands`: The `Commands` resource to insert the loaded handle.
pub fn preload_bullet_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(BulletAudio(asset_server.load("audio/laser.mp3")));
}
