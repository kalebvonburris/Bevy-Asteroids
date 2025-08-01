//! # Bevy Asteroids
//!
//! A 2D space shooter game built with Bevy.

// Many bevy systems require >7 arguments, which is not allowed by clippy.
#![allow(clippy::too_many_arguments)]

pub mod asteroid;
pub mod audio;
pub mod bullet;
pub mod explosion;
pub mod ship;
pub mod ui;

use crate::ui::{
    setup::setup_ui,
    systems::{PlayerScore, ScoreEvent, update_score},
};
use asteroid::{check_asteroid_bounds, move_asteroids, spawn_asteroids};
use audio::main_song::play_main_song;
use bevy::{
    app::PanicHandlerPlugin, diagnostic::DiagnosticsPlugin, prelude::*,
    render::mesh::VertexAttributeValues,
};
use bullet::{check_bullet_bounds, check_bullet_collisions, move_bullets, setup_bullet};
use explosion::{setup_explosions, systems::explosion_system};
use ship::*;

/// The main plugin for the game, which sets up the game state and systems.
pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        // Make the background black.
        app.insert_resource(ClearColor(Color::BLACK));

        // Set the fixed time step - this is how often we
        // check to see if we spawn asteroids
        app.insert_resource(Time::<Fixed>::from_seconds(0.5));

        // Insert the player's score resource and add the event.
        app.insert_resource(PlayerScore(0));
        app.add_event::<ScoreEvent>();

        app.add_systems(Startup, spawn_camera);

        // Setup default plugins
        let mut default_plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroids".to_string(),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        });

        // Disable unnecessary plugins
        default_plugins = default_plugins
            .disable::<PanicHandlerPlugin>()
            .disable::<DiagnosticsPlugin>();

        app.add_plugins(default_plugins);

        app.add_systems(
            Startup,
            (
                setup_player,
                setup_bullet,
                setup_explosions,
                play_main_song,
                setup_ui,
            ),
        );

        app.add_systems(
            Update,
            (
                // Asteroids
                move_asteroids,
                check_asteroid_bounds,
                // Player ship
                check_ship_bounds,
                player_input_and_movement,
                check_ship_collisions,
                color_player,
                // Bullets
                move_bullets,
                check_bullet_bounds,
                check_bullet_collisions,
                // Explosions
                explosion_system,
                // Score and UI
                update_score,
            ),
        );

        // Fixed systems
        app.add_systems(
            FixedUpdate,
            (
                spawn_asteroids
                    .before(move_asteroids)
                    .before(check_asteroid_bounds),
                heal_player,
            ),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}

/// Checks if two lines intersect.
///
/// # Arguments
/// * `p1`: The first point of the first line.
/// * `p2`: The second point of the first line.
/// * `p3`: The first point of the second line.
/// * `p4`: The second point of the second line.
///
/// # Returns
/// An `Option<Vec2>` that contains the intersection point if the lines intersect, or `None` if they do not.
pub fn lines_intersect(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) -> Option<Vec2> {
    let s1 = p2 - p1;
    let s2 = p4 - p3;

    let denom = -s2.x * s1.y + s1.x * s2.y;
    if denom == 0.0 {
        return None; // Lines are parallel
    }

    let s = (-s1.y * (p1.x - p3.x) + s1.x * (p1.y - p3.y)) / denom;
    let t = (s2.x * (p1.y - p3.y) - s2.y * (p1.x - p3.x)) / denom;

    if (0.0..=1.0).contains(&s) && (0.0..=1.0).contains(&t) {
        Some(p1 + t * s1)
    } else {
        None
    }
}

/// Converts a mesh and its transform into a list of points in absolute coordinates.
///
/// # Arguments
/// * `mesh`: The `Mesh` to convert.
/// * `transform`: The `Transform` to apply to the mesh points.
///
/// # Returns
/// A `Vec<Vec2>` containing the transformed, absolute points of the mesh.
pub fn mesh_and_transform_to_points(mesh: &Mesh, transform: &Transform) -> Vec<Vec2> {
    let position_data = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

    if let VertexAttributeValues::Float32x3(positions) = position_data {
        // Calculate the new position of the points from the transform
        positions
            .iter()
            .map(|position| {
                let curr_position = Vec3::from((position[0], position[1], position[2]));
                let translated_position = transform.compute_matrix() * curr_position.extend(1.0);

                Vec2::new(translated_position.x, translated_position.y)
            })
            .collect::<Vec<Vec2>>()
    } else {
        panic!("No positions found in the mesh");
    }
}
