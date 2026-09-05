//! Data for the player ship in the game.

use bevy::prelude::*;

/// The points that define the player ship's shape.
/// These point are turned into a `LineStrip` mesh for rendering.
/// The ship itself is an arrow head shape.
pub const PLAYER_SHIP_POINTS: &[Vec3] = &[
    Vec3::new(-5.0, -5.0, 1.0),
    Vec3::new(0.0, 5.0, 1.0),
    Vec3::new(5.0, -5.0, 1.0),
    Vec3::new(0.0, -2.5, 1.0),
    Vec3::new(-5.0, -5.0, 1.0),
];

/// How far the ship reaches from its center, taken from [`PLAYER_SHIP_POINTS`].
/// Used to keep the ship on screen and to place the muzzle of its bullets.
pub const SHIP_RADIUS: f32 = 5.0;

/// The health the ship spawns with, and the cap it heals back up to.
pub const MAX_SHIP_HEALTH: i32 = 100;

/// The player ship component that holds the player's health and speed.
#[derive(Component)]
pub struct PlayerShip {
    /// The health of the player ship.
    pub health: i32,
    /// The speed of the player ship.
    pub speed: f32,
}

/// An event fired when the ship's health changes, so the UI and the ship's
/// color can react to it.
#[derive(EntityEvent)]
pub struct ShipHealthChange {
    /// The ship whose health changed.
    pub entity: Entity,
    /// The ship's health after the change.
    pub amount: i32,
}
