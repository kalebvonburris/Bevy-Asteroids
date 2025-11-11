//! # Bullet
//!
//! Data for the bullet system in the game.

pub mod data;
pub use data::*;
pub mod systems;
pub use systems::*;

use crate::GameState;
use bevy::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_bullet);
        app.add_systems(
            Update,
            (move_bullets, check_bullet_bounds, check_bullet_collisions)
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(OnExit(GameState::Game), despawn_bullets);
    }
}
