//! # Audio
//!
//! This module contains audio playback functionality for the game.

use bevy::prelude::*;

use crate::in_play_or_game_over;

pub mod asteroid;
pub mod bullet;
pub mod main_song;
pub mod ship;

/// Loads the game's sound effects and starts the background music.
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        // These load assets, so they run after `Startup` - loading during
        // `Startup` breaks WASM builds.
        app.add_systems(
            PostStartup,
            (
                asteroid::preload_asteroid_sounds,
                bullet::preload_bullet_audio,
                ship::preload_player_sounds,
                main_song::play_main_song,
            ),
        );
        app.add_observer(asteroid::play_explosion_sound.run_if(in_play_or_game_over()));
    }
}

/// Plays a one-shot sound from an entity that despawns itself once the clip ends.
///
/// # Arguments
/// * `commands`: The `Commands` resource to spawn the audio player entity.
/// * `sound`: A handle to the clip to play.
pub fn play_sound(commands: &mut Commands, sound: Handle<AudioSource>) {
    commands.spawn((AudioPlayer::new(sound), PlaybackSettings::DESPAWN));
}
