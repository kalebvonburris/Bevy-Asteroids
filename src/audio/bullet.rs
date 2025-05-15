//! Handles the audio for the bullet firing and striking an asteroid.

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::channels::LaserChannel;

pub fn fire_bullet(
    asset_server: &Res<AssetServer>,
    audio: &Res<AudioChannel<LaserChannel>>,
) {
    audio.play(asset_server.load("embedded://audio/laser.wav")).with_volume(0.5);
}
