//! Data for the player ship in the game.

use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};

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

/// The player ship component that holds the player's health and speed.
#[derive(Component)]
pub struct PlayerShip {
    /// The health of the player ship.
    pub health: i32,
    /// The speed of the player ship.
    pub speed: f32,
}

/// Sets up the player ship with a mesh and material.
/// 
/// # Arguments
/// * `commands`: The `Commands` resource to spawn the player ship entity.
/// * `meshes`: The `Assets<Mesh>` resource to create the player ship mesh.
/// * `materials`: The `Assets<ColorMaterial>` resource to create the player ship material.
pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create a mesh for the player ship
    let ship_mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::all())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, PLAYER_SHIP_POINTS.to_vec());

    // Spawn the player ship
    commands.spawn((
        Name::new("Player Ship"),
        PlayerShip {
            health: 100,
            speed: 0.0,
        },
        Mesh2d(meshes.add(ship_mesh)),
        MeshMaterial2d(
            // Use a green color for the player ship in full health
            materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::new(
                0.2, 1.0, 0.2, 1.0,
            )))),
        ),
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
    ));
}
