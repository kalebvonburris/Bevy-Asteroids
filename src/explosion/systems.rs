use bevy::prelude::*;

use super::Explosion;

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
