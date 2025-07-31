//! Explosion systems.

use bevy::prelude::*;

use super::Explosion;

/// System that handles the explosion effects in the game.
/// 
/// # Arguments
/// * `commands`: The `Commands` resource to despawn the explosion entity.
/// * `query`: The `Query` resource to iterate over entities with the `Explosion`
///   component.
/// * `time`: The `Time` resource to get the current time.
pub fn explosion_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Explosion)>,
    time: Res<Time>,
) {
    for (entity, mut transform, explosion) in query.iter_mut() {
        transform.scale *= 1.0 + time.delta_secs();
        if explosion.start_time + 1.0 <= time.elapsed_secs() {
            commands.entity(entity).despawn();
        }
    }
}
