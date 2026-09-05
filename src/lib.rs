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

use bevy::{
    app::PanicHandlerPlugin, diagnostic::DiagnosticsPlugin, ecs::schedule::SystemCondition,
    prelude::*, render::mesh::VertexAttributeValues,
};

use crate::{
    asteroid::AsteroidPlugin, audio::GameAudioPlugin, bullet::BulletPlugin,
    explosion::ExplosionPlugin, ship::ShipPlugin, ui::GameUiPlugin,
};

/// The main plugin for the game, which sets up the game state and systems.
pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Asteroids".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            // Neither of these earn their keep in a game this small.
            .disable::<PanicHandlerPlugin>()
            .disable::<DiagnosticsPlugin>();

        app.add_plugins(default_plugins)
            .init_state::<GameState>()
            // Space is black.
            .insert_resource(ClearColor(Color::BLACK))
            // Drives the fixed timestep, which is how often we try to spawn
            // asteroids and heal the player.
            .insert_resource(Time::<Fixed>::from_seconds(0.5))
            .add_plugins((
                AsteroidPlugin,
                BulletPlugin,
                ExplosionPlugin,
                GameAudioPlugin,
                GameUiPlugin,
                ShipPlugin,
            ))
            // Spawned in `PostStartup` alongside the asset-loading systems, which
            // have to run after `Startup` or the loads break WASM builds.
            .add_systems(PostStartup, spawn_camera);
    }
}

/// The state of the user interface.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    /// The title screen, shown before the first game and between runs.
    #[default]
    MainMenu,
    /// A game in progress.
    Game,
    /// The player ship has been destroyed. Asteroids keep drifting behind the
    /// score readout until the player restarts.
    GameOver,
}

/// Run condition for if the game is in the `GameState::Game` or
/// `GameState::GameOver` states - used by many systems.
pub fn in_play_or_game_over() -> impl SystemCondition<()> {
    in_state(GameState::Game).or_else(in_state(GameState::GameOver))
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}

/// Despawns every entity holding the component `C`.
///
/// Used to tear down a screen or clear the playfield on a state transition, e.g.
/// `despawn_all::<Asteroid>`.
///
/// # Arguments
/// * `commands`: The `Commands` resource to despawn the entities.
/// * `query`: A query that retrieves all entities with the `C` component.
pub fn despawn_all<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Checks whether a point has left the window entirely.
///
/// # Arguments
/// * `translation`: The position to test.
/// * `margin`: The entity's radius, so it only counts as out of bounds once it
///   is fully off-screen.
/// * `half_window`: The distance from the center of the window to each edge.
///
/// # Returns
/// `true` if the point is past any edge of the window.
pub fn out_of_bounds(translation: Vec3, margin: f32, half_window: Vec2) -> bool {
    translation.x + margin < -half_window.x
        || translation.x - margin > half_window.x
        || translation.y + margin < -half_window.y
        || translation.y - margin > half_window.y
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
                let translated_position = transform.to_matrix() * curr_position.extend(1.0);

                Vec2::new(translated_position.x, translated_position.y)
            })
            .collect::<Vec<Vec2>>()
    } else {
        panic!("No positions found in the mesh");
    }
}
