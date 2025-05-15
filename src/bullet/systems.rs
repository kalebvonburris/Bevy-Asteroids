use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::AudioChannel;

use crate::{
    asteroid::{Asteroid, AsteroidSize, LARGE_PARAMETERS, MEDIUM_PARAMETERS, SMALL_PARAMETERS}, audio::channels::ExplosionChannel, explosion::{create_explosion, ExplosionConfig}, lines_intersect, mesh_and_transform_to_points
};

use super::Bullet;

pub fn move_bullets(time: Res<Time>, mut query: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in query.iter_mut() {
        let angle = transform.rotation.to_euler(EulerRot::ZXY).0;

        transform.translation.x += -angle.sin() * bullet.speed * time.delta_secs();
        transform.translation.y += angle.cos() * bullet.speed * time.delta_secs();
    }
}

pub fn check_bullet_bounds(
    mut commands: Commands,
    mut query: Query<(Entity, &Bullet, &Transform)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // Get the window size
    let window = window.single().unwrap();

    let window_size = Vec2::new(window.width(), window.height());

    for (entity, _, transform) in query.iter_mut() {
        // Get the asteroid width
        let radius = 1.75;

        // Check if the asteroid is out of bounds
        if transform.translation.x + radius < -window_size.x / 2.0
            || transform.translation.x - radius > window_size.x / 2.0
            || transform.translation.y + radius < -window_size.y / 2.0
            || transform.translation.y - radius > window_size.y / 2.0
        {
            // Remove the asteroid
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_bullet_collisions(
    mut commands: Commands,
    asteroids: Query<(Entity, &Asteroid, &Transform, &Mesh2d)>,
    bullets: Query<(Entity, &Bullet, &Transform, &Mesh2d)>,
    asset_server: Res<AssetServer>,
    audio: Res<AudioChannel<ExplosionChannel>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    explosion_config: Res<ExplosionConfig>,
    time: Res<Time>,
) {
    for (bullet_entity, _, bullet_transform, bullet_mesh) in bullets.iter() {
        for (asteroid_entity, asteroid, asteroid_transform, asteroid_mesh) in asteroids.iter() {
            let asteroid_radius = match asteroid.size {
                AsteroidSize::Small => SMALL_PARAMETERS.1,
                AsteroidSize::Medium => MEDIUM_PARAMETERS.1,
                AsteroidSize::Large => LARGE_PARAMETERS.1,
            };

            let distance = bullet_transform
                .translation
                .distance(asteroid_transform.translation);

            if distance < asteroid_radius + 3.5 {
                // Get the asteroid's points
                let asteroid_mesh = meshes.get(&asteroid_mesh.0).unwrap();

                let asteroid_points =
                    mesh_and_transform_to_points(asteroid_mesh, asteroid_transform);

                // Get the bullet's points
                let bullet_mesh = meshes.get(&bullet_mesh.0).unwrap();

                let bullet_points = mesh_and_transform_to_points(bullet_mesh, bullet_transform);

                let bullet_start = bullet_points[0];
                let bullet_end = bullet_points[1];

                // Check if any of the lines of the bullet intersect with the asteroid
                asteroid_points.windows(2).for_each(|p| {
                    let line_start = Vec2::new(p[0][0], p[0][1]);
                    let line_end = Vec2::new(p[1][0], p[1][1]);

                    // Check if the line intersects with the asteroid
                    if let Some(p) = lines_intersect(line_start, line_end, bullet_start, bullet_end)
                    {
                        // Create the transform for the explosion
                        let point_of_contact = Transform::from_translation(p.extend(-1.0));

                        // Blow up the asteroid
                        commands.entity(asteroid_entity).despawn();

                        // Blow up the bullet
                        commands.entity(bullet_entity).despawn();

                        // Spawn an explosion
                        create_explosion(
                            &mut commands,
                            point_of_contact,
                            &explosion_config,
                            &time,
                            false,
                        );

                        crate::audio::asteroid::destroy_asteroid(asteroid.size, &asset_server, &audio);

                        // Check if we need to make children
                        if asteroid.size != AsteroidSize::Small {
                            let child_size = match asteroid.size {
                                AsteroidSize::Medium => AsteroidSize::Small,
                                AsteroidSize::Large => AsteroidSize::Medium,
                                _ => unreachable!(),
                            };

                            for _ in 0..2 {
                                // Pick a random spot in the asteroid's radius
                                let radius = match asteroid.size {
                                    AsteroidSize::Small => SMALL_PARAMETERS.1,
                                    AsteroidSize::Medium => MEDIUM_PARAMETERS.1,
                                    AsteroidSize::Large => LARGE_PARAMETERS.1,
                                };

                                let r = radius * rand::random_range(0.0f32..1.0).sqrt();

                                let theta =
                                    rand::random_range(0.0f32..1.0) * 2.0 * std::f32::consts::PI;

                                let x = r * theta.cos();
                                let y = r * theta.sin();

                                let location = Vec3::new(
                                    asteroid_transform.translation.x + x,
                                    asteroid_transform.translation.y + y,
                                    0.0,
                                );

                                // Pick a random direction
                                let direction = Vec2::new(
                                    rand::random_range(-1.0f32..1.0),
                                    rand::random_range(-1.0f32..1.0),
                                )
                                .normalize()
                                    + asteroid.direction;

                                // Spawn the new asteroid
                                Asteroid::spawn_new(
                                    child_size,
                                    location,
                                    direction,
                                    &mut commands,
                                    &mut meshes,
                                    &mut materials,
                                );
                            }
                        }
                    }
                });
            }
        }
    }
}
