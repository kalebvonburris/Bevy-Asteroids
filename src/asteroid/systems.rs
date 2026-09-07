//! Systems for managing asteroids in the game.

use super::{Asteroid, AsteroidSize, RunTimer, SpawnAsteroid};
use crate::out_of_bounds;
use bevy::{prelude::*, window::PrimaryWindow};

/// Moves the asteroids based on their direction. Speed is determined by
/// the x and y components of the direction vector.
///
/// # Arguments
/// * `time`: The `time` resource to calculate the movement delta.
/// * `query`: A query that retrieves every `Asteroid` and its `Transform`.
pub fn move_asteroids(time: Res<Time>, mut query: Query<(&Asteroid, &mut Transform)>) {
    for (asteroid, mut transform) in query.iter_mut() {
        transform.translation += asteroid.direction.extend(0.0) * time.delta_secs();
    }
}

/// Despawns asteroids once they have drifted off the edge of the game window.
///
/// # Arguments
/// * `commands`: The `Commands` resource to despawn asteroids that are out of bounds.
/// * `query`: A query that retrieves every `Asteroid` and its `Transform`.
/// * `window`: A query that retrieves the primary window to get its size.
pub fn check_asteroid_bounds(
    mut commands: Commands,
    query: Query<(Entity, &Asteroid, &Transform)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single().unwrap();
    let half_window = Vec2::new(window.width(), window.height()) / 2.0;

    for (entity, asteroid, transform) in query.iter() {
        // An asteroid that leaves the screen never comes back, so drop it.
        if out_of_bounds(&transform.translation, asteroid.size.radius(), half_window) {
            commands.entity(entity).despawn();
        }
    }
}

/// Spawns new asteroids just off the edges of the window, ramping up in number,
/// size, and speed the longer the game has been running.
///
/// # Arguments
/// * `commands`: The `Commands` resource to trigger [`SpawnAsteroid`] events.
/// * `window`: A query that retrieves the primary window to get its size.
/// * `time`: The `Time` resource to measure the run against.
/// * `run_timer`: The [`RunTimer`] resource holding how long this run has lasted.
pub fn spawn_asteroids(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    run_timer: Res<RunTimer>,
) {
    // The longer the run has lasted, the more asteroids will spawn.
    // This is done by using a logarithmic function to adjust the time between spawns.
    // In this case I found that a logarithmic function with a base of 5.0 works well.
    // The time elapsed is divided by 2.0 to make the game easier at the start,
    // and then the logarithm is applied to it.
    let log_base = 5.0;
    let time_adjusted = ((run_timer.elapsed(&time) / 2.0) + log_base).log(log_base);

    let window = window.single().unwrap();

    // The distance from the center of the screen to each edge.
    let half_window = Vec2::new(window.width(), window.height()) / 2.0;

    // We use a while loop to spawn a random number of asteroids.
    while rand::random_range(0.0..time_adjusted) > 1.0 {
        // Pick a size for the asteroid - the longer the game is running, the bigger the asteroid,
        // and the more frequent larger asteroids will spawn.
        let size = match rand::random_range(0.0..time_adjusted) {
            0.0..1.5 => AsteroidSize::Small,
            1.5..2.5 => AsteroidSize::Medium,
            _ => AsteroidSize::Large,
        };

        let radius = size.radius();

        // Park the asteroid just off one edge of the window, at a random point
        // along that edge, then aim it back across the screen.
        let (location, direction) = match rand::random_range(0..4) {
            // The top of the screen, pointing down.
            0 => (
                Vec2::new(
                    rand::random_range(-half_window.x..half_window.x),
                    half_window.y + radius,
                ),
                Vec2::new(rand::random_range(-1.0..1.0), rand::random_range(-1.0..0.1)),
            ),
            // The right side of the screen, pointing left.
            1 => (
                Vec2::new(
                    half_window.x + radius,
                    rand::random_range(-half_window.y..half_window.y),
                ),
                Vec2::new(
                    rand::random_range(-1.0..-0.1),
                    rand::random_range(-1.0..1.0),
                ),
            ),
            // The bottom of the screen, pointing up.
            2 => (
                Vec2::new(
                    rand::random_range(-half_window.x..half_window.x),
                    -half_window.y - radius,
                ),
                Vec2::new(rand::random_range(-1.0..1.0), rand::random_range(0.1..1.0)),
            ),
            // The left side of the screen, pointing right.
            _ => (
                Vec2::new(
                    -half_window.x - radius,
                    rand::random_range(-half_window.y..half_window.y),
                ),
                Vec2::new(rand::random_range(0.1..1.0), rand::random_range(-1.0..1.0)),
            ),
        };

        // Smaller asteroids travel faster, and everything speeds up over time.
        let speed = match size {
            AsteroidSize::Small => 10.0,
            AsteroidSize::Medium => 5.0,
            AsteroidSize::Large => 2.5,
        };

        commands.trigger(SpawnAsteroid {
            asteroid: Asteroid {
                size,
                direction: direction.normalize() * speed * time_adjusted,
            },
            location: location.extend(0.0),
        });
    }
}
