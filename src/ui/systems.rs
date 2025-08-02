//! Systems for the user interface components of the application.

use bevy::prelude::*;

use crate::ui::setup::ScoreText;

/// Resource to hold the player's score.
#[derive(Resource)]
pub struct PlayerScore(pub i32);

/// Event to update the score in the UI.
#[derive(Event)]
pub struct ScoreEvent(pub i32);

/// Updates the score text in the UI based on the player's score.
///
/// # Arguments
/// * `query`: A query that retrieves the `ScoreText` component.
/// * `events`: An event reader to read `ScoreEvent` events.
/// * `player_score`: A mutable reference to the `PlayerScore` resource.
pub fn update_score(
    mut query: Query<&mut Text, With<ScoreText>>,
    mut events: EventReader<ScoreEvent>,
    mut player_score: ResMut<PlayerScore>,
) {
    for event in events.read() {
        player_score.0 += event.0;
    }

    for mut text in query.iter_mut() {
        (*text).0 = format!("Score: {}", player_score.0);
    }
}
