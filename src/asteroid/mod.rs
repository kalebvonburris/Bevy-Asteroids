//! # Asteroid
//!
//! This module contains the data structures and systems related to asteroids in the game.

use bevy::prelude::*;

use crate::{GameState, despawn_all, in_play_or_game_over};

pub mod data;
pub use data::*;
pub mod systems;
pub use systems::*;

/// Spawns asteroids, drifts them across the screen, and clears the playfield
/// when the player restarts.
pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, preload_asteroids);
        app.init_resource::<RunTimer>();
        // Each run starts from the same difficulty, however long the app has
        // been open or how long the player sat on a menu.
        app.add_systems(OnEnter(GameState::Game), reset_run_timer);
        // Asteroids keep drifting behind the game over screen, and are only
        // cleared once the player starts a new run.
        app.add_systems(OnExit(GameState::GameOver), despawn_all::<Asteroid>);
        app.add_systems(
            Update,
            (move_asteroids, check_asteroid_bounds).run_if(in_play_or_game_over()),
        );
        app.add_systems(
            FixedUpdate,
            spawn_asteroids
                .before(move_asteroids)
                .before(check_asteroid_bounds)
                .run_if(in_play_or_game_over()),
        );
        app.add_observer(spawn_asteroid.run_if(in_play_or_game_over()));
    }
}
