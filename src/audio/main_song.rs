use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub fn play_main_song(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    audio.play(asset_server.load("embedded://audio/Eternity.mp3")).with_volume(0.7).looped();
}
