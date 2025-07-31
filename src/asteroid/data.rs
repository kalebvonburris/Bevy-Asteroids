//! Asteroid data and parameters for the game.

use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};
use rand::Rng;

/// Parameters that define the `min_diameter, max_diameter, number_of_points` for small asteroids.
pub const SMALL_PARAMETERS: AsteroidParameters = AsteroidParameters {
    min_diameter: 8.0,
    max_diameter: 15.0,
    number_of_points: 10,
};

/// Parameters that define the `min_diameter, max_diameter, number_of_points` for medium asteroids.
pub const MEDIUM_PARAMETERS: AsteroidParameters = AsteroidParameters {
    min_diameter: 20.0,
    max_diameter: 30.0,
    number_of_points: 20,
};

/// Parameters that define the `min_diameter, max_diameter, number_of_points` for large asteroids.
pub const LARGE_PARAMETERS: AsteroidParameters = AsteroidParameters {
    min_diameter: 40.0,
    max_diameter: 60.0,
    number_of_points: 30,
};

/// Parameters for asteroid generation.
#[derive(Debug, Clone, Copy)]
pub struct AsteroidParameters {
    /// The minimum radius of the asteroid.
    pub min_diameter: f32,
    /// The maximum radius of the asteroid.
    pub max_diameter: f32,
    /// The number of points that make up the asteroid's along its edges.
    pub number_of_points: usize,
}

/// The size of the asteroid, which determines its radius and number of points.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AsteroidSize {
    Small = 15,
    Medium = 30,
    Large = 50,
}

impl AsteroidSize {
    /// Returns the diameter of the asteroid based on its size.
    pub fn diameter(self) -> f32 {
        match self {
            AsteroidSize::Small => SMALL_PARAMETERS.max_diameter,
            AsteroidSize::Medium => MEDIUM_PARAMETERS.max_diameter,
            AsteroidSize::Large => LARGE_PARAMETERS.max_diameter,
        }
    }

    /// Returns the radius of the asteroid based on its size.
    pub fn radius(self) -> f32 {
        self.diameter() / 2.0
    }
}

/// Represents an `Asteroid` in the game.
#[derive(Component, Debug)]
pub struct Asteroid {
    /// The `size` of the asteroid. See [`AsteroidSize`].
    pub size: AsteroidSize,
    /// The `direction` of the asteroid's movement.
    /// This is a vector that indicates the direction and speed of the asteroid.
    pub direction: Vec2,
}

impl Asteroid {
    pub fn spawn_new(
        size: AsteroidSize,
        location: Vec3,
        direction: Vec2,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
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
                    rng.random_range(
                        asteroid_parameters.min_diameter..=asteroid_parameters.max_diameter,
                    ),
                )
            })
            .collect::<Vec<_>>();

        // Push the first point to the end of the vector
        // to close the asteroid shape. We have to do this because
        // the asteroid is drawn as a line strip, and the last point
        // needs to connect to the first point to draw a closed shape.
        asteroid_points.push(asteroid_points[0]);

        let mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::all())
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                asteroid_points
                    .iter()
                    // Here we use the angle and radius to calculate the position of the point
                    // using polar coordinates. The cosine of the angle `c` times its radius gives the x coordinate,
                    // and the sine of the angle `c` times the point's radius gives the y coordinate.
                    .map(|(c, angle)| Vec3::new(c.cos() * angle, c.sin() * angle, 5.0))
                    .collect::<Vec<_>>(),
            );

        let asteroid = Asteroid { size, direction };

        // Spawn a list of lines with start and end points for each lines
        commands.spawn((
            asteroid,
            Mesh2d(meshes.add(mesh)),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
            Transform::from_xyz(location.x, location.y, location.z),
        ));
    }
}
