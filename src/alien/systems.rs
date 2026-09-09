use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};

use crate::alien::data::{ALIEN_SHIP_POINTS, AlienResource};

/// Sets up the alien ship resource with a mesh and material.
///
/// # Arguments
/// * `commands`: `Commands` to spawn the [`AlienResource`].
/// * `meshes`: The `Assets<Mesh>` resource to create the ship mesh.
/// * `materials`: The `Assets<ColorMaterial>` resource to create the ship material (color).
pub fn setup_alien(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::all())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, ALIEN_SHIP_POINTS.to_vec());

    commands.insert_resource(AlienResource {
        mesh: meshes.add(mesh),
        color: materials.add(ColorMaterial::from(Color::linear_rgb(0.4, 1.0, 0.4))),
    });
}

pub fn spawn_aliens(mut commands: Commands) {}
