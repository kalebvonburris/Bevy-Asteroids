//! Channels for audio playback in the game.

use bevy::prelude::*;

/// The audio channel for laser sounds.
#[derive(Resource)]
pub struct LaserChannel;

/// The audio channel for explosion sounds.
#[derive(Resource)]
pub struct ExplosionChannel;
