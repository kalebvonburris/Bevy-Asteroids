use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
}

impl Bullet {
    pub fn spawn_bullet(
        commands: &mut Commands,
        mut transform: Transform,
        speed: f32,
        bullet_config: &Res<BulletConfig>,
    ) {
        // Modify the transform to start the bullet at the ship's nose
        let angle = transform.rotation.to_euler(EulerRot::ZXY).0;

        // Move the bullet 5 units forward in the direction of the ship's rotation
        transform.translation.x += -angle.sin() * 5.0;
        transform.translation.y += angle.cos() * 5.0;

        commands
            .spawn((
                Mesh2d(bullet_config.mesh.clone()),
                MeshMaterial2d(bullet_config.material.clone()),
                transform,
            ))
            .insert(Bullet {
                speed: 55.0 + speed,
            });
    }
}

pub fn setup_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create a mesh for the bullet
    let bullet_mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::all())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 5.0, 2.0)],
        );

    // Create a material for the bullet
    let bullet_material = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::new(
        1.0, 0.2, 0.2, 1.0,
    ))));

    // Create a bullet config
    let bullet_config = BulletConfig::new(meshes.add(bullet_mesh), bullet_material);

    commands.insert_resource(bullet_config);
}

#[derive(Resource)]
pub struct BulletConfig {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

impl BulletConfig {
    pub fn new(mesh: Handle<Mesh>, material: Handle<ColorMaterial>) -> Self {
        Self { mesh, material }
    }
}
