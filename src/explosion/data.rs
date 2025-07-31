//! Data for the explosion system in the game.

use bevy::prelude::*;

/// Explosion component that tracks the start time of the explosion.
#[derive(Component)]
pub struct Explosion {
    /// The time when the explosion started.
    pub start_time: f32,
}

impl Explosion {
    pub fn new(start_time: f32) -> Self {
        Self { start_time }
    }
}

/// Configuration for the explosion, including its mesh and materials.
#[derive(Resource)]
pub struct ExplosionConfig {
    /// The mesh used for the explosion.
    pub mesh: Handle<Mesh>,
    /// The material used for asteroid explosions.
    pub asteroid_color: Handle<ColorMaterial>,
    /// The material used for blowing the `PlayerShip` up.
    pub player_color: Handle<ColorMaterial>,
}

/// Sets up the explosion resources, including the mesh and materials.
///
/// # Arguments
/// * `commands`: The `Commands` resource to insert the explosion configuration.
/// * `meshes`: The `Assets<Mesh>` resource to create the explosion mesh.
/// * `materials`: The `Assets<ColorMaterial>` resource to create the explosion materials.
pub fn setup_explosions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create a mesh for the explosion
    let explosion_mesh = Annulus::new(5.0, 7.0);

    // Create a material for the explosion
    let asteroid_color = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::new(
        1.0, 1.0, 1.0, 0.5,
    ))));

    let player_color = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::new(
        1.0, 0.0, 0.0, 0.86,
    ))));

    // Create an explosion config
    let explosion_config = ExplosionConfig {
        mesh: meshes.add(explosion_mesh),
        asteroid_color,
        player_color,
    };

    commands.insert_resource(explosion_config);
}

/// Creates an explosion at the specified transform.
/// 
/// # Arguments
/// * `commands`: The `Commands` resource to spawn the explosion entity.
/// * `transform`: The `Transform` where the explosion should be spawned.
/// * `explosion_config`: The `ExplosionConfig` resource to get the explosion mesh and materials.
/// * `time`: The `Time` resource to get the current time.
/// * `player`: A boolean indicating if the explosion is for the player ship.
pub fn create_explosion(
    commands: &mut Commands,
    transform: Transform,
    explosion_config: &Res<ExplosionConfig>,
    time: &Res<Time>,
    player: bool,
) {
    commands
        .spawn((
            Mesh2d(explosion_config.mesh.clone()),
            MeshMaterial2d(if player {
                explosion_config.player_color.clone()
            } else {
                explosion_config.asteroid_color.clone()
            }),
            transform,
        ))
        .insert(Explosion::new(time.elapsed_secs()));
}
