//! Asteroid data and parameters for the game.

use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};
use rand::{RngExt, random_range};

/// Parameters that define the `min_radius, max_radius, number_of_points` for small asteroids.
pub const SMALL_PARAMETERS: AsteroidParameters = AsteroidParameters {
    min_radius: 8.0,
    max_radius: 15.0,
    number_of_points: 10,
};

/// Parameters that define the `min_radius, max_radius, number_of_points` for medium asteroids.
pub const MEDIUM_PARAMETERS: AsteroidParameters = AsteroidParameters {
    min_radius: 20.0,
    max_radius: 30.0,
    number_of_points: 20,
};

/// Parameters that define the `min_radius, max_radius, number_of_points` for large asteroids.
pub const LARGE_PARAMETERS: AsteroidParameters = AsteroidParameters {
    min_radius: 40.0,
    max_radius: 60.0,
    number_of_points: 30,
};

/// The difficulty clock for the current run.
///
/// Asteroid spawning ramps up with how long the player has been alive, so this
/// has to be measured from the start of the run rather than from app startup -
/// otherwise a restart would inherit the difficulty of the previous run, and
/// time spent sitting on a menu would count against the player.
#[derive(Resource, Default)]
pub struct RunTimer {
    /// The value of `Time::elapsed_secs()` when this run began.
    start: f32,
}

impl RunTimer {
    /// Returns the seconds elapsed since the current run began.
    ///
    /// # Arguments
    /// * `time`: The `Time` resource to measure against.
    pub fn elapsed(&self, time: &Time) -> f32 {
        time.elapsed_secs() - self.start
    }
}

/// Restarts the difficulty clock at zero when a new run begins.
///
/// # Arguments
/// * `time`: The `Time` resource to stamp the start of the run with.
/// * `run_timer`: The [`RunTimer`] resource to reset.
pub fn reset_run_timer(time: Res<Time>, mut run_timer: ResMut<RunTimer>) {
    run_timer.start = time.elapsed_secs();
}

/// Parameters for asteroid generation.
#[derive(Debug, Clone, Copy)]
pub struct AsteroidParameters {
    /// The minimum distance an edge point sits from the asteroid's center.
    pub min_radius: f32,
    /// The maximum distance an edge point sits from the asteroid's center.
    pub max_radius: f32,
    /// The number of points that make up the asteroid's along its edges.
    pub number_of_points: usize,
}

/// The size of the asteroid, which determines its radius and number of points.
///
/// The discriminant doubles as the damage the asteroid deals to the player ship
/// on impact; see [`crate::ship::check_ship_collisions`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AsteroidSize {
    Small = 15,
    Medium = 30,
    Large = 50,
}

impl AsteroidSize {
    pub fn radius(self) -> f32 {
        match self {
            AsteroidSize::Small => SMALL_PARAMETERS.max_radius,
            AsteroidSize::Medium => MEDIUM_PARAMETERS.max_radius,
            AsteroidSize::Large => LARGE_PARAMETERS.max_radius,
        }
    }
}

/// Prerendered data about asteroids - saves on huge memory
/// usage from spawning a new mesh/color for every instantiated asteroid.
#[derive(Resource)]
pub struct AsteroidResource {
    pub small_meshes: Vec<Handle<Mesh>>,
    pub medium_meshes: Vec<Handle<Mesh>>,
    pub large_meshes: Vec<Handle<Mesh>>,
    pub color: Handle<ColorMaterial>,
}

pub fn preload_asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let small_meshes = (0..10)
        .map(|_| meshes.add(generate_random_asteroid_mesh(AsteroidSize::Small)))
        .collect();
    let medium_meshes = (0..10)
        .map(|_| meshes.add(generate_random_asteroid_mesh(AsteroidSize::Medium)))
        .collect();
    let large_meshes = (0..10)
        .map(|_| meshes.add(generate_random_asteroid_mesh(AsteroidSize::Large)))
        .collect();

    let color = materials.add(ColorMaterial::from_color(Color::WHITE));
    commands.insert_resource(AsteroidResource {
        small_meshes,
        medium_meshes,
        large_meshes,
        color,
    });
}

fn generate_random_asteroid_mesh(size: AsteroidSize) -> Mesh {
    let asteroid_parameters = match size {
        AsteroidSize::Small => SMALL_PARAMETERS,
        AsteroidSize::Medium => MEDIUM_PARAMETERS,
        AsteroidSize::Large => LARGE_PARAMETERS,
    };

    // Generate a random asteroid
    let mut rng = rand::rng();
    let mut asteroid_points = (0..asteroid_parameters.number_of_points)
        .map(|i| {
            let random = rng.random_range(0.0..1.0);

            // Calculate the angle for the point.
            // The angle is unevenly distributed.
            // To do so, we use a random offset from an evenly distributed angle
            // and add it to the evenly distributed angle to generate a random point.
            let c = (random + (i as f32))
                * (std::f32::consts::PI * 2.0 / asteroid_parameters.number_of_points as f32);

            (
                c,
                rng.random_range(asteroid_parameters.min_radius..=asteroid_parameters.max_radius),
            )
        })
        .collect::<Vec<_>>();

    // Push the first point to the end of the vector
    // to close the asteroid shape. We have to do this because
    // the asteroid is drawn as a line strip, and the last point
    // needs to connect to the first point to draw a closed shape.
    asteroid_points.push(asteroid_points[0]);

    Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::all()).with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        asteroid_points
            .iter()
            .map(|(c, radius)| Vec3::new(c.cos() * radius, c.sin() * radius, 5.0))
            .collect::<Vec<_>>(),
    )
}

/// Represents an `Asteroid` in the game.
#[derive(Component, Clone, Debug)]
pub struct Asteroid {
    /// The `size` of the asteroid. See [`AsteroidSize`].
    pub size: AsteroidSize,
    /// The `direction` of the asteroid's movement.
    /// This is a vector that indicates the direction and speed of the asteroid.
    pub direction: Vec2,
}

/// An event that spawns a new asteroid, handled by [`spawn_asteroid`].
#[derive(Event)]
pub struct SpawnAsteroid {
    /// The asteroid to build a mesh for and spawn.
    pub asteroid: Asteroid,
    /// Where in the world to place it.
    pub location: Vec3,
}

/// Builds a randomly shaped mesh for a [`SpawnAsteroid`] event and spawns it.
///
/// # Arguments
/// * `data`: The `SpawnAsteroid` event that triggered this observer.
/// * `commands`: The `Commands` resource to spawn the asteroid entity.
/// * `meshes`: The `Assets<Mesh>` resource to create the asteroid mesh.
/// * `materials`: The `Assets<ColorMaterial>` resource to create the asteroid material.
pub fn spawn_asteroid(
    data: On<SpawnAsteroid>,
    asteroid_resource: Res<AsteroidResource>,
    mut commands: Commands,
) {
    let mesh = match data.asteroid.size {
        AsteroidSize::Small => {
            let index = random_range(..asteroid_resource.small_meshes.len());
            asteroid_resource.small_meshes[index].clone()
        }
        AsteroidSize::Medium => {
            let index = random_range(..asteroid_resource.medium_meshes.len());
            asteroid_resource.medium_meshes[index].clone()
        }
        AsteroidSize::Large => {
            let index = random_range(..asteroid_resource.large_meshes.len());
            asteroid_resource.large_meshes[index].clone()
        }
    };

    commands.spawn((
        data.asteroid.clone(),
        Mesh2d(mesh),
        MeshMaterial2d(asteroid_resource.color.clone()),
        Transform::from_translation(data.location),
    ));
}
