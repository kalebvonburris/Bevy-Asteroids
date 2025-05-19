use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};
use rand::Rng;

pub const SMALL_PARAMETERS: (f32, f32, usize) = (8.0, 15.0, 10);
pub const MEDIUM_PARAMETERS: (f32, f32, usize) = (22.0, 35.0, 20);
pub const LARGE_PARAMETERS: (f32, f32, usize) = (40.0, 60.0, 30);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AsteroidSize {
    Small = 15,
    Medium = 30,
    Large = 50,
}

#[derive(Component, Debug)]
pub struct Asteroid {
    pub size: AsteroidSize,
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
        let mut asteroid_points = (0..asteroid_parameters.2)
            .map(|i| {
                let random = rng.random_range(0.0..1.0);

                let c = (random + (i as f32))
                    * (std::f32::consts::PI * 2.0 / asteroid_parameters.2 as f32);

                (
                    c,
                    rng.random_range(asteroid_parameters.0..asteroid_parameters.1),
                )
            })
            .collect::<Vec<_>>();

        // Push the first point to the end of the vector
        asteroid_points.push(asteroid_points[0]);

        // Spawn one asteroid
        let mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::all())
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                asteroid_points
                    .iter()
                    .map(|(c, y)| Vec3::new(c.cos() * y, c.sin() * y, 5.0))
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
