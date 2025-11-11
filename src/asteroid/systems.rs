//! Systems for managing asteroids in the game.

use super::{Asteroid, AsteroidSize};
use bevy::{prelude::*, window::PrimaryWindow};

/// Moves the asteroids based on their direction. Speed is determined by
/// the x and y components of the direction vector.
///
/// # Arguments
/// * `time`: The `time` resource to calculate the movement delta.
/// * `query`: A query that retrieves every `Asteroid` and its `Transform`.
pub fn move_asteroids(time: Res<Time>, mut query: Query<(&Asteroid, &mut Transform)>) {
    for (asteroid, mut transform) in query.iter_mut() {
        transform.translation +=
            Vec3::new(asteroid.direction.x, asteroid.direction.y, 0.0) * time.delta_secs();
    }
}

/// Checks and fixes asteroids so that they are within the bounds of the game window.
///
/// # Arguments
/// * `commands`: The `Commands` resource to despawn asteroids that are out of bounds.
/// * `query`: A query that retrieves every `Asteroid` and its `Transform`.
/// * `window`: A query that retrieves the primary window to get its size.
pub fn check_asteroid_bounds(
    mut commands: Commands,
    mut query: Query<(Entity, &Asteroid, &Transform)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // Get the window size
    let window = window.single().unwrap();

    let window_size = Vec2::new(window.width(), window.height());

    for (entity, asteroid, transform) in query.iter_mut() {
        // Get the asteroid width
        let asteroid_diameter = asteroid.size.diameter();

        // Check if the asteroid is out of bounds
        // If the asteroid is out of bounds, despawn it - it will never return to the screen
        if transform.translation.x + asteroid_diameter < -window_size.x / 2.0
            || transform.translation.x - asteroid_diameter > window_size.x / 2.0
            || transform.translation.y + asteroid_diameter < -window_size.y / 2.0
            || transform.translation.y - asteroid_diameter > window_size.y / 2.0
        {
            // Remove the asteroid
            commands.entity(entity).despawn();
        }
    }
}

/// Spawns new asteroids based on the game state and window size.
///
/// # Arguments
/// * `commands`: The `Commands` resource to spawn new asteroids.
/// * `meshes`: The `Assets<Mesh>` resource to create the asteroid mesh.
/// * `materials`: The `Assets<ColorMaterial>` resource to create the asteroid material.
/// * `window`: A query that retrieves the primary window to get its size.
/// * `time`: The `Time` resource to determine the frequency of asteroid spawning.
pub fn spawn_asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    // The longer the game is running, the more asteroids will spawn.
    // This is done by using a logarithmic function to adjust the time between spawns.
    // In this case I found that a logarithmic function with a base of 5.0 works well.
    // The time elapsed is divided by 2.0 to make the game easier at the start,
    // and then the logarithm is applied to it.
    let log_base = 5.0;
    let time_elapsed = (time.elapsed_secs() / 2.0) + log_base;
    let time_adjusted = time_elapsed.log(log_base);

    let window = window.single().unwrap();

    // We use a while loop to spawn a random number of asteroids.
    while rand::random_range(0.0..time_adjusted) > 1.0 {
        // Pick a size for the asteroid - the longer the game is running, the bigger the asteroid,
        // and the more frequent larger asteroids will spawn.
        let size = match rand::random_range(0.0..time_adjusted) {
            0.0..1.5 => AsteroidSize::Small,
            1.5..2.5 => AsteroidSize::Medium,
            _ => AsteroidSize::Large,
        };

        let asteroid_diameter = size.diameter();

        // Pick a location for the asteroid.
        // TODO: Don't match on an integer!
        let (location, mut dir) = match rand::random_range(0..4) {
            0 => {
                // The top of the screen
                (
                    Vec3::new(
                        rand::random_range(-window.width() / 2.0..window.width() / 2.0),
                        window.height() / 2.0 + asteroid_diameter,
                        0.0,
                    ),
                    // Pick anywhere pointing down
                    Vec2::new(rand::random_range(-1.0..1.0), rand::random_range(-1.0..0.1))
                        .normalize(),
                )
            }
            1 => {
                // The right side of the screen
                (
                    Vec3::new(
                        window.width() / 2.0 + asteroid_diameter,
                        rand::random_range(-window.height() / 2.0..window.height() / 2.0),
                        0.0,
                    ),
                    // Pick anywhere pointing left
                    Vec2::new(
                        rand::random_range(-1.0..-0.1),
                        rand::random_range(-1.0..1.0),
                    )
                    .normalize(),
                )
            }
            2 => {
                // The bottom of the screen
                (
                    Vec3::new(
                        rand::random_range(-window.width() / 2.0..window.width() / 2.0),
                        -window.height() / 2.0 - asteroid_diameter,
                        0.0,
                    ),
                    // Pick anywhere pointing up
                    Vec2::new(rand::random_range(-1.0..1.0), rand::random_range(0.1..1.0))
                        .normalize(),
                )
            }
            3 => {
                // The left side of the screen
                (
                    Vec3::new(
                        -window.width() / 2.0 - asteroid_diameter,
                        rand::random_range(-window.height() / 2.0..window.height() / 2.0),
                        0.0,
                    ),
                    // Pick anywhere pointing right
                    Vec2::new(rand::random_range(0.1..1.0), rand::random_range(-1.0..1.0))
                        .normalize(),
                )
            }
            _ => unreachable!(),
        };

        // Adjust the speed of the asteroid based on its size
        match size {
            AsteroidSize::Small => {
                // Small asteroids are faster
                dir *= 10.0;
            }
            AsteroidSize::Medium => {
                // Medium asteroids are normal speed
                dir *= 5.0;
            }
            AsteroidSize::Large => {
                // Large asteroids are slower
                dir *= 2.5;
            }
        }

        dir *= time_adjusted;

        // Create the asteroid
        Asteroid::spawn_new(
            size,
            location,
            dir,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}

/// Despawns all asteroids in the game.
///
/// # Arguments
/// * `commands`: The `Commands` resource to despawn asteroids.
/// * `query`: A query that retrieves all entities with the `Asteroid` component.
pub fn despawn_asteroids(mut commands: Commands, query: Query<Entity, With<Asteroid>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
