//! # Explosion
//!
//! This module handles the explosion effects in the game.

use bevy::prelude::*;

use crate::in_play_or_game_over;

pub mod data;
pub use data::*;
pub mod systems;
pub use systems::*;

/// Spawns and animates the expanding rings left behind by destroyed asteroids.
pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_explosions);
        app.add_systems(Update, explosion_system.run_if(in_play_or_game_over()));
        app.add_observer(on_explosion.run_if(in_play_or_game_over()));
    }
}
