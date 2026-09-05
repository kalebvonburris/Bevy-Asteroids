//! Data for the explosion system in the game.

use bevy::prelude::*;

use crate::asteroid::AsteroidSize;

/// An `Explosion` event for spawning a new one.
#[derive(Event)]
pub struct Explosion {
    /// The exact center point of the explosion.
    pub point: Transform,
    /// The size of the asteroid.
    pub size: AsteroidSize,
}

/// Explosion component that tracks the start time of the explosion.
#[derive(Component)]
pub struct ExplosionData {
    /// The time when the explosion started.
    pub start_time: f32,
}

/// Configuration for the explosion, including its mesh and materials.
#[derive(Resource)]
pub struct ExplosionConfig {
    /// The mesh used for the explosion.
    pub mesh: Handle<Mesh>,
    /// The material used for explosions.
    pub color: Handle<ColorMaterial>,
}
