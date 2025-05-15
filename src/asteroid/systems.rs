use bevy::{prelude::*, window::PrimaryWindow};

use super::{Asteroid, AsteroidSize, LARGE_PARAMETERS, MEDIUM_PARAMETERS, SMALL_PARAMETERS};

pub fn move_asteroids(time: Res<Time>, mut query: Query<(&Asteroid, &mut Transform)>) {
    for (asteroid, mut transform) in query.iter_mut() {
        // Move the asteroid
        transform.translation +=
            Vec3::new(asteroid.direction.x, asteroid.direction.y, 0.0) * time.delta_secs();
    }
}

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
        let asteroid_radius = match asteroid.size {
            AsteroidSize::Small => SMALL_PARAMETERS.1,
            AsteroidSize::Medium => MEDIUM_PARAMETERS.1,
            AsteroidSize::Large => LARGE_PARAMETERS.1,
        };

        // Check if the asteroid is out of bounds
        if transform.translation.x + asteroid_radius < -window_size.x / 2.0
            || transform.translation.x - asteroid_radius > window_size.x / 2.0
            || transform.translation.y + asteroid_radius < -window_size.y / 2.0
            || transform.translation.y - asteroid_radius > window_size.y / 2.0
        {
            // Remove the asteroid
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    // Check if the time is right to spawn an asteroid
    // The longer the game is running, the more asteroids will spawn
    let time_elapsed = (time.elapsed_secs() / 2.0) + 10.0;
    let time_adjusted = time_elapsed.log10();

    let window = window.single().unwrap();

    while rand::random_range(0.0..time_adjusted) > 1.0 {
        // Pick a size for the asteroid - the longer the game is running, the bigger the asteroid
        let size = match rand::random_range(0.0..time_adjusted) {
            0.0..1.5 => AsteroidSize::Small,
            1.5..2.5 => AsteroidSize::Medium,
            _ => AsteroidSize::Large,
        };

        let asteroid_diameter = match size {
            AsteroidSize::Small => SMALL_PARAMETERS.1,
            AsteroidSize::Medium => MEDIUM_PARAMETERS.1,
            AsteroidSize::Large => LARGE_PARAMETERS.1,
        };

        // Pick a location for the asteroid
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
