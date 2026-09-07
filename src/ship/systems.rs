//! Systems for the player ship in the game.

use bevy::window::PrimaryWindow;
use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};

use crate::asteroid::{Asteroid, AsteroidSize};
use crate::audio::bullet::BulletAudio;
use crate::audio::play_sound;
use crate::audio::ship::PlayerSounds;
use crate::bullet::{Bullet, BulletConfig};
use crate::explosion::Explosion;
use crate::ship::{MAX_SHIP_HEALTH, PLAYER_SHIP_POINTS, SHIP_RADIUS};
use crate::{GameState, lines_intersect, mesh_and_transform_to_points};

use super::{PlayerShip, ShipHealthChange};

/// Handles player input and movement, including shooting bullets.
///
/// # Arguments
/// * `commands`: The `Commands` resource to spawn bullets.
/// * `keyboard_input`: The `ButtonInput<KeyCode>` resource to check for player input.
/// * `query`: A query that retrieves the player ship's `PlayerShip` and its `Transform`.
/// * `time`: The `Time` resource to calculate the movement delta.
/// * `bullet_config`: The `BulletConfig` resource to configure the bullets.
/// * `bullet_audio`: The `BulletAudio` resource holding the firing sound.
pub fn player_input_and_movement(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlayerShip, &mut Transform)>,
    time: Res<Time>,
    bullet_config: Res<BulletConfig>,
    bullet_audio: Res<BulletAudio>,
) {
    for (mut player_ship, mut transform) in query.iter_mut() {
        let target_speed = if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            50.0
        } else {
            0.0
        };
        player_ship.speed = player_ship.speed.lerp(target_speed, time.delta_secs());

        if player_ship.speed > 0.0 {
            // Get the direction of the ship
            let angle = transform.rotation.to_euler(EulerRot::ZXY).0;
            let distance = player_ship.speed * time.delta_secs();

            transform.translation.x += -angle.sin() * distance;
            transform.translation.y += angle.cos() * distance;
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
            Bullet::spawn_bullet(&mut commands, *transform, player_ship.speed, &bullet_config);
            play_sound(&mut commands, bullet_audio.0.clone());
        }
    }
}

/// Keeps the player ship inside the window instead of letting it fly off-screen.
///
/// # Arguments
/// * `query`: A query that retrieves the player ship's `Transform`.
/// * `window`: A query that retrieves the primary window to get its size.
pub fn check_ship_bounds(
    mut query: Query<&mut Transform, With<PlayerShip>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single().unwrap();
    let half_window = Vec2::new(window.width(), window.height()) / 2.0;

    let limit = half_window - Vec2::splat(SHIP_RADIUS);

    for mut transform in query.iter_mut() {
        transform.translation.x = transform.translation.x.clamp(-limit.x, limit.x);
        transform.translation.y = transform.translation.y.clamp(-limit.y, limit.y);
    }
}

/// Checks for collisions between the player ship and asteroids, damaging the
/// ship and ending the game if that damage kills it.
///
/// # Arguments
/// * `commands`: The `Commands` resource to despawn the player ship and asteroids.
/// * `asteroids`: A query that retrieves every `Asteroid`, its `Transform`, and its `Mesh2d`.
/// * `ships`: A query that retrieves the player ship's `PlayerShip`, its `Transform`, and its `Mesh2d`.
/// * `meshes`: The `Assets<Mesh>` resource to get the mesh of the asteroids and ships.
/// * `player_sounds`: The `PlayerSounds` resource holding the hit and death sounds.
/// * `next_state`: The `NextState<GameState>` resource to change the game state to `GameOver` if the player ship is destroyed.
pub fn check_ship_collisions(
    mut commands: Commands,
    asteroids: Query<(Entity, &Asteroid, &Transform, &Mesh2d)>,
    mut ships: Query<(Entity, &mut PlayerShip, &Transform, &Mesh2d)>,
    meshes: Res<Assets<Mesh>>,
    player_sounds: Res<PlayerSounds>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (player_entity, mut player_ship, ship_transform, ship_mesh) in ships.iter_mut() {
        for (asteroid_entity, asteroid, asteroid_transform, asteroid_mesh) in asteroids.iter() {
            // Broad phase: skip the per-line check unless the two are close enough
            // to possibly be touching.
            if ship_transform
                .translation
                .distance(asteroid_transform.translation)
                >= SHIP_RADIUS + asteroid.size.radius()
            {
                continue;
            }

            let asteroid_mesh = meshes.get(&asteroid_mesh.0).unwrap();
            let asteroid_points = mesh_and_transform_to_points(asteroid_mesh, asteroid_transform);

            let ship_mesh = meshes.get(&ship_mesh.0).unwrap();
            let ship_points = mesh_and_transform_to_points(ship_mesh, ship_transform);

            for s in ship_points.windows(2) {
                for a in asteroid_points.windows(2) {
                    let Some(p) = lines_intersect(&s[0], &s[1], &a[0], &a[1]) else {
                        continue;
                    };

                    let point = Transform::from_translation(p.extend(-1.0));

                    // Bigger asteroids hit harder; see `AsteroidSize`.
                    player_ship.health -= asteroid.size as i32;

                    if player_ship.health <= 0 {
                        commands.entity(player_entity).despawn();
                        commands.trigger(Explosion {
                            point,
                            size: AsteroidSize::Large,
                        });
                        play_sound(&mut commands, player_sounds.destroyed.clone());
                        next_state.set(GameState::GameOver);
                    } else {
                        play_sound(&mut commands, player_sounds.hit.clone());
                    }

                    // Blow up the asteroid
                    commands.entity(asteroid_entity).despawn();
                    commands.trigger(Explosion {
                        point,
                        size: asteroid.size,
                    });

                    return;
                }
            }
        }
    }
}

/// Slowly regenerates the player's health on the fixed timestep.
///
/// # Arguments
/// * `query`: A query that retrieves the player ship's `PlayerShip`.
/// * `commands`: The `Commands` resource to trigger `ShipHealthChange` events.
pub fn heal_player(mut query: Query<(Entity, &mut PlayerShip)>, mut commands: Commands) {
    for (entity, mut player_ship) in query.iter_mut() {
        if player_ship.health != MAX_SHIP_HEALTH {
            player_ship.health = (player_ship.health + 1).min(MAX_SHIP_HEALTH);
            commands.trigger(ShipHealthChange {
                entity,
                amount: player_ship.health,
            });
        }
    }
}

/// Fades the ship from green to red as its health drops.
///
/// # Arguments
/// * `hc`: The `ShipHealthChange` event that triggered this observer.
/// * `query`: A query used to look up the ship's material.
/// * `materials`: The `Assets<ColorMaterial>` resource holding that material.
pub fn color_player(
    hc: On<ShipHealthChange>,
    query: Query<&MeshMaterial2d<ColorMaterial>, With<PlayerShip>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(color_material) = query.get(hc.entity) else {
        return;
    };

    let health_percentage = hc.amount as f32 / MAX_SHIP_HEALTH as f32;

    materials.get_mut(&color_material.0).unwrap().color =
        Color::linear_rgb(1.0 - health_percentage, health_percentage, 0.0);
}

/// Sets up the player ship with a mesh and material.
///
/// # Arguments
/// * `commands`: The `Commands` resource to spawn the player ship entity.
/// * `meshes`: The `Assets<Mesh>` resource to create the player ship mesh.
/// * `materials`: The `Assets<ColorMaterial>` resource to create the player ship material.
pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ship_mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::all())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, PLAYER_SHIP_POINTS.to_vec());

    commands.spawn((
        Name::new("Player Ship"),
        PlayerShip {
            health: MAX_SHIP_HEALTH,
            speed: 0.0,
        },
        Mesh2d(meshes.add(ship_mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::linear_rgb(0.2, 1.0, 0.2)))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
    ));
}
