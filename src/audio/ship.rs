//! Handles the audio for the bullet firing and striking an asteroid.

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::channels::ExplosionChannel;

pub fn ship_hit(
    asset_server: &Res<AssetServer>,
    audio: &Res<AudioChannel<ExplosionChannel>>,
) {
    audio.play(asset_server.load("embedded://audio/player_hit.mp3")).with_volume(0.5);
}

pub fn ship_destroyed(
    asset_server: &Res<AssetServer>,
    audio: &Res<AudioChannel<ExplosionChannel>>,
) {
    audio.play(asset_server.load("embedded://audio/player_destroyed.mp3")).with_volume(0.7);
}
