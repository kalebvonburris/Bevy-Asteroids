//! Systems for the player ship in the game.

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::AudioChannel;

use crate::asteroid::Asteroid;

use crate::audio::bullet::fire_bullet;
use crate::audio::channels::{ExplosionChannel, LaserChannel};
use crate::audio::ship::*;
use crate::bullet::{Bullet, BulletConfig};
use crate::explosion::{ExplosionConfig, create_explosion};
use crate::{lines_intersect, mesh_and_transform_to_points};

use super::PlayerShip;

/// Handles player input and movement, including shooting bullets.
/// 
/// # Arguments
/// * `commands`: The `Commands` resource to spawn bullets.
/// * `keyboard_input`: The `ButtonInput<KeyCode>` resource to check for player input
/// * `query`: A query that retrieves the player ship's `PlayerShip` and its `Transform`.
/// * `time`: The `Time` resource to calculate the movement delta.
/// * `bullet_config`: The `BulletConfig` resource to configure the bullets.
/// * `asset_server`: The `AssetServer` resource to load the bullet sound asset.
/// * `audio`: The `AudioChannel<LaserChannel>` resource to play the bullet firing
pub fn player_input_and_movement(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlayerShip, &mut Transform)>,
    time: Res<Time>,
    bullet_config: Res<BulletConfig>,
    asset_server: Res<AssetServer>,
    audio: Res<AudioChannel<LaserChannel>>,
) {
    for (mut player_ship, mut transform) in query.iter_mut() {
        if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            // Lerp the speed between 0.0 and 50.0
            player_ship.speed = player_ship.speed.lerp(50.0, time.delta_secs());
        } else {
            // Lerp the speed between the current speed and 0.0
            player_ship.speed = player_ship.speed.lerp(0.0, time.delta_secs());
        }

        if player_ship.speed > 0.0 {
            // Get the direction of the ship
            let angle = transform.rotation.to_euler(EulerRot::ZXY).0;

            transform.translation.x += -angle.sin() * player_ship.speed * time.delta_secs();
            transform.translation.y += angle.cos() * player_ship.speed * time.delta_secs();
        }

        // Rotate left
        if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            transform.rotate_local_z(6.0 * time.delta_secs());
        }

        // Rotate right
        if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            transform.rotate_local_z(-6.0 * time.delta_secs());
        }

        // Shoot
        if keyboard_input.any_just_pressed([KeyCode::Space]) {
            // Spawn a bullet
            Bullet::spawn_bullet(
                &mut commands.reborrow(),
                *transform,
                player_ship.speed,
                &bullet_config,
            );
            fire_bullet(&asset_server, &audio);
        }
    }
}

pub fn check_ship_bounds(
    mut query: Query<(&PlayerShip, &mut Transform)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // Get the window size
    let window = window.single().unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    for (_, mut transform) in query.iter_mut() {
        // Check if the ship is out of bounds
        if transform.translation.x - 5.0 < -window_size.x / 2.0 {
            transform.translation.x = -window_size.x / 2.0 + 5.0;
        } else if transform.translation.x + 5.0 > window_size.x / 2.0 {
            transform.translation.x = window_size.x / 2.0 - 5.0;
        } else if transform.translation.y - 5.0 < -window_size.y / 2.0 {
            transform.translation.y = -window_size.y / 2.0 + 5.0;
        } else if transform.translation.y + 5.0 > window_size.y / 2.0 {
            transform.translation.y = window_size.y / 2.0 - 5.0;
        }
    }
}

pub fn check_ship_collisions(
    mut commands: Commands,
    asteroids: Query<(Entity, &Asteroid, &Transform, &Mesh2d)>,
    mut ships: Query<(Entity, &mut PlayerShip, &Transform, &Mesh2d)>,
    explosion_config: Res<ExplosionConfig>,
    time: Res<Time>,
    meshes: Res<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    audio: Res<AudioChannel<ExplosionChannel>>,
) {
    for (player_entity, mut player_ship, ship_transform, ship_mesh) in ships.iter_mut() {
        for (asteroid_entity, asteroid, asteroid_transform, asteroid_mesh) in asteroids.iter() {
            let asteroid_diameter = asteroid.size.diameter();

            // Check if the ship is colliding with the asteroid
            if ship_transform
                .translation
                .distance(asteroid_transform.translation)
                < 5.0 + asteroid_diameter
            {
                // Get the asteroid's points
                let asteroid_mesh = meshes.get(&asteroid_mesh.0).unwrap();
                let asteroid_points =
                    mesh_and_transform_to_points(asteroid_mesh, asteroid_transform);

                // Get the ship's points
                let ship_mesh = meshes.get(&ship_mesh.0).unwrap();
                let ship_points = mesh_and_transform_to_points(ship_mesh, ship_transform);

                for s in ship_points.windows(2) {
                    for a in asteroid_points.windows(2) {
                        if let Some(p) = lines_intersect(s[0], s[1], a[0], a[1]) {
                            // Get the point of contact
                            let point_of_contact = Transform::from_translation(p.extend(-1.0));

                            // Damage the ship
                            player_ship.health -= asteroid.size as i32;

                            if player_ship.health <= 0 {
                                // Create the transform for the explosion
                                let point_of_contact = Transform::from_translation(p.extend(-1.0));

                                // Blow up the ship
                                commands.entity(player_entity).despawn();

                                // Create an explosion
                                create_explosion(
                                    &mut commands,
                                    point_of_contact,
                                    &explosion_config,
                                    &time,
                                    false,
                                );
                                ship_destroyed(&asset_server, &audio);
                            } else {
                                ship_hit(&asset_server, &audio);
                            }

                            // Blow up the asteroid
                            commands.entity(asteroid_entity).despawn();

                            // Create an explosion
                            create_explosion(
                                &mut commands,
                                point_of_contact,
                                &explosion_config,
                                &time,
                                true,
                            );
                            return;
                        }
                    }
                }
            }
        }
    }
}

pub fn heal_player(mut query: Query<&mut PlayerShip>) {
    for mut player_ship in query.iter_mut() {
        player_ship.health = (player_ship.health + 1).min(100);
    }
}

pub fn color_player(
    mut query: Query<(&PlayerShip, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (player_ship, color_material) in query.iter_mut() {
        let health_percentage = player_ship.health as f32 / 100.0;

        let color = materials.get_mut(&color_material.0).unwrap();

        color.color = Color::LinearRgba(LinearRgba::new(
            1.0 - health_percentage,
            health_percentage,
            0.0,
            1.0,
        ));
    }
}
