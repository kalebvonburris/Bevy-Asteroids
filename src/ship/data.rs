use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};

pub const PLAYER_SHIP_POINTS: &[Vec3] = &[
    Vec3::new(-5.0, -5.0, 1.0),
    Vec3::new(0.0, 5.0, 1.0),
    Vec3::new(5.0, -5.0, 1.0),
    Vec3::new(0.0, -2.5, 1.0),
    Vec3::new(-5.0, -5.0, 1.0),
];

#[derive(Component)]
pub struct PlayerShip {
    pub health: i32,
    pub speed: f32,
}

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
            materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::new(
                0.2, 1.0, 0.2, 1.0,
            )))),
        ),
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
    ));
}
