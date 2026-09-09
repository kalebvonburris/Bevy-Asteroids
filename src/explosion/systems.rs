//! Explosion systems.

use crate::explosion::*;

/// How long an explosion expands for before it is despawned, in seconds.
const EXPLOSION_LIFETIME: f32 = 1.5;

/// Grows each explosion ring and despawns it once it has outlived its lifetime.
///
/// # Arguments
/// * `commands`: The `Commands` resource to despawn the explosion entity.
/// * `query`: A query that retrieves every explosion's `Transform` and `ExplosionData`.
/// * `time`: The `Time` resource to get the current time.
pub fn explosion_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &ExplosionData)>,
    time: Res<Time>,
) {
    for (entity, mut transform, explosion) in query.iter_mut() {
        transform.scale *= 1.0 + time.delta_secs();
        if explosion.start_time + EXPLOSION_LIFETIME <= time.elapsed_secs() {
            commands.entity(entity).despawn();
        }
    }
}

/// Creates an explosion at the given point from an
/// `Explosion` event.
///
/// # Arguments
/// * `explosion`: The `Explosion` event that triggered this observer.
/// * `config`: The `ExplosionConfig` resource holding the shared mesh and material.
/// * `time`: The `Time` resource used to stamp the explosion's start time.
/// * `commands`: The `Commands` resource to spawn the explosion entity.
pub fn on_explosion(
    explosion: On<Explosion>,
    config: Res<ExplosionConfig>,
    time: Res<Time>,
    mut commands: Commands,
) {
    commands.spawn((
        Mesh2d(config.mesh.clone()),
        MeshMaterial2d(config.color.clone()),
        explosion.point,
        ExplosionData {
            start_time: time.elapsed_secs(),
        },
    ));
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
    commands.insert_resource(ExplosionConfig {
        // A ring, scaled up over the explosion's lifetime to look like a blast wave.
        mesh: meshes.add(Annulus::new(5.0, 7.0)),
        color: materials.add(ColorMaterial::from(Color::linear_rgba(1.0, 1.0, 1.0, 0.5))),
    });
}
