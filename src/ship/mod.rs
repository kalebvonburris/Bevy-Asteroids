//! # Ship
//!
//! This module contains the data and systems related to the player ship in the game.

use bevy::prelude::*;

use crate::{GameState, despawn_all};

pub mod systems;
pub use systems::*;
pub mod data;
pub use data::*;

/// Spawns the player ship and runs its input, movement, collision, and health systems.
pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_player);
        app.add_systems(OnExit(GameState::Game), despawn_all::<PlayerShip>);
        app.add_systems(
            Update,
            (
                check_ship_bounds,
                player_input_and_movement,
                check_ship_collisions,
            )
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(FixedUpdate, heal_player.run_if(in_state(GameState::Game)));
        app.add_observer(color_player.run_if(in_state(GameState::Game)));
    }
}
