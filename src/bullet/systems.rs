//! Systems for managing bullets in the game.

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    asteroid::{Asteroid, AsteroidSize, SpawnAsteroid},
    bullet::{BULLET_RADIUS, BulletHit},
    explosion::Explosion,
    lines_intersect, mesh_and_transform_to_points, out_of_bounds,
    ui::ScoreEvent,
};

use super::Bullet;

/// Handles the movement of bullets based on their speed and the time elapsed since the last frame.
///
/// # Arguments
/// * `time`: The `Time` resource to calculate the movement delta.
/// * `query`: A query that retrieves every `Bullet` and its `Transform`.
pub fn move_bullets(time: Res<Time>, mut query: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in query.iter_mut() {
        let angle = transform.rotation.to_euler(EulerRot::ZXY).0;
        let distance = bullet.speed * time.delta_secs();

        transform.translation.x += -angle.sin() * distance;
        transform.translation.y += angle.cos() * distance;
    }
}

/// Despawns bullets once they leave the bounds of the game window.
///
/// # Arguments
/// * `commands`: The `Commands` resource to despawn bullets that are out of bounds.
/// * `query`: A query that retrieves every `Bullet` and its `Transform`.
/// * `window`: A query that retrieves the primary window to get its size.
pub fn check_bullet_bounds(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Bullet>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single().unwrap();
    let half_window = Vec2::new(window.width(), window.height()) / 2.0;

    for (entity, transform) in query.iter() {
        if out_of_bounds(transform.translation, BULLET_RADIUS, half_window) {
            commands.entity(entity).despawn();
        }
    }
}

/// Checks for collisions between bullets and asteroids, firing a [`BulletHit`]
/// and a [`ScoreEvent`] for each one found.
///
/// # Arguments
/// * `commands`: The `Commands` resource to trigger the hit and score events.
/// * `asteroids`: A query that retrieves every `Asteroid`, its `Transform`, and its `Mesh2d`.
/// * `bullets`: A query that retrieves every `Bullet`, its `Transform`, and its `Mesh2d`.
/// * `meshes`: The `Assets<Mesh>` resource to get the mesh of the asteroids and bullets.
pub fn check_bullet_collisions(
    mut commands: Commands,
    asteroids: Query<(Entity, &Asteroid, &Transform, &Mesh2d)>,
    bullets: Query<(Entity, &Transform, &Mesh2d), With<Bullet>>,
    meshes: Res<Assets<Mesh>>,
) {
    'bullets: for (bullet_entity, bullet_transform, bullet_mesh) in bullets.iter() {
        for (asteroid_entity, asteroid, asteroid_transform, asteroid_mesh) in asteroids.iter() {
            let distance = bullet_transform
                .translation
                .distance(asteroid_transform.translation);

            if distance >= asteroid.size.radius() + 3.5 {
                continue;
            }

            let asteroid_mesh = meshes.get(&asteroid_mesh.0).unwrap();
            let asteroid_points = mesh_and_transform_to_points(asteroid_mesh, asteroid_transform);

            let bullet_mesh = meshes.get(&bullet_mesh.0).unwrap();
            let bullet_points = mesh_and_transform_to_points(bullet_mesh, bullet_transform);

            for edge in asteroid_points.windows(2) {
                let Some(p) = lines_intersect(edge[0], edge[1], bullet_points[0], bullet_points[1])
                else {
                    continue;
                };

                commands.trigger(BulletHit {
                    bullet: bullet_entity,
                    asteroid: asteroid_entity,
                    // Sit the explosion behind the asteroid it came from.
                    point: Transform::from_translation(p.extend(-1.0)),
                });

                commands.trigger(ScoreEvent(1));

                // Rust continue marker magic. TECHNICALLY NOT
                // a GOTO so it's cool.
                continue 'bullets;
            }
        }
    }
}

/// Blows up the asteroid a bullet hit, splitting it into two smaller asteroids
/// unless it was already the smallest size.
///
/// # Arguments
/// * `hit`: The [`BulletHit`] event that triggered this observer.
/// * `query`: A query used to look up the struck asteroid.
/// * `commands`: The `Commands` resource to despawn the pair and spawn the debris.
pub fn on_bullet_hit(
    hit: On<BulletHit>,
    query: Query<(&Asteroid, &Transform)>,
    mut commands: Commands,
) {
    // Double hit case
    let Ok((asteroid, transform)) = query.get(hit.asteroid) else {
        return;
    };

    commands.trigger(Explosion {
        point: hit.point,
        size: asteroid.size,
    });

    // Check if we need to make children
    if asteroid.size != AsteroidSize::Small {
        let child_size = match asteroid.size {
            AsteroidSize::Medium => AsteroidSize::Small,
            AsteroidSize::Large => AsteroidSize::Medium,
            _ => unreachable!(),
        };

        // Spawn two smaller asteroids
        for _ in 0..2 {
            let radius = asteroid.size.radius() * rand::random_range(0.0f32..1.0).sqrt();
            let theta = rand::random_range(0.0f32..1.0) * 2.0 * std::f32::consts::PI;
            let offset = Vec2::from_angle(theta) * radius;

            let direction = Vec2::new(
                rand::random_range(-1.0f32..1.0),
                rand::random_range(-1.0f32..1.0),
            )
            .normalize()
                + asteroid.direction;

            commands.trigger(SpawnAsteroid {
                asteroid: Asteroid {
                    size: child_size,
                    direction,
                },
                location: (transform.translation.truncate() + offset).extend(0.0),
            });
        }
    }

    commands.entity(hit.asteroid).despawn();
    commands.entity(hit.bullet).despawn();
}
