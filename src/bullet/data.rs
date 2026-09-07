//! Data for bullets in the game.

use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};

use crate::ship::SHIP_RADIUS;

/// The speed a bullet leaves the ship at, before the ship's own speed is added.
pub const BULLET_BASE_SPEED: f32 = 55.0;

/// The bullet's collision radius, used to decide when it has left the screen.
pub const BULLET_RADIUS: f32 = 1.75;

/// A bullet fired from the `PlayerShip`.
#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
}

impl Bullet {
    /// Spawns a bullet at the ship's nose, inheriting the ship's speed.
    ///
    /// # Arguments
    /// * `commands`: The `Commands` resource to spawn the bullet entity.
    /// * `transform`: The `Transform` of the ship, which determines the bullet's position and rotation.
    /// * `ship_speed`: The speed of the ship, added on top of [`BULLET_BASE_SPEED`].
    /// * `bullet_config`: The [`BulletConfig`] that holds the bullet's mesh and material.
    pub fn spawn_bullet(
        commands: &mut Commands,
        mut transform: Transform,
        ship_speed: f32,
        bullet_config: &Res<BulletConfig>,
    ) {
        // Move the bullet forward out of the ship's nose so it does not start
        // inside the ship's own mesh.
        let angle = transform.rotation.to_euler(EulerRot::ZXY).0;
        transform.translation.x += -angle.sin() * SHIP_RADIUS;
        transform.translation.y += angle.cos() * SHIP_RADIUS;

        commands.spawn((
            Mesh2d(bullet_config.mesh.clone()),
            MeshMaterial2d(bullet_config.material.clone()),
            transform,
            Bullet {
                speed: BULLET_BASE_SPEED + ship_speed,
            },
        ));
    }
}

/// An event fired when a bullet's mesh intersects an asteroid's mesh.
#[derive(Event)]
pub struct BulletHit {
    /// The bullet that landed the hit.
    pub bullet: Entity,
    /// The asteroid that was hit.
    pub asteroid: Entity,
    /// The point of contact, used to place the explosion.
    pub point: Transform,
}

/// Builds the shared bullet mesh and material and stores them in [`BulletConfig`].
///
/// # Arguments
/// * `commands`: The `Commands` resource to insert the bullet configuration.
/// * `meshes`: The `Assets<Mesh>` resource to create the bullet mesh.
/// * `materials`: The `Assets<ColorMaterial>` resource to create the bullet material.
pub fn setup_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let bullet_mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::all())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 5.0, 2.0)],
        );

    commands.insert_resource(BulletConfig {
        mesh: meshes.add(bullet_mesh),
        material: materials.add(ColorMaterial::from(Color::linear_rgb(1.0, 0.2, 0.2))),
    });
}

/// Configuration for the bullet; includes its mesh and material.
///
/// Every bullet shares one mesh and one material rather than building its own.
#[derive(Resource)]
pub struct BulletConfig {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}
