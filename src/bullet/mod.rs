//! # Bullet
//!
//! Data for the bullet system in the game.

pub mod data;
pub use data::*;
pub mod systems;
pub use systems::*;

use crate::{GameState, despawn_all, in_play_or_game_over};
use bevy::prelude::*;

/// Fires, moves, and resolves collisions for the player's bullets.
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_bullet);
        app.add_systems(
            Update,
            (move_bullets, check_bullet_bounds, check_bullet_collisions)
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(OnExit(GameState::Game), despawn_all::<Bullet>);
        app.add_observer(on_bullet_hit.run_if(in_play_or_game_over()));
    }
}
