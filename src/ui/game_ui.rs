//! Contains the UI element for handling the player's score in the game.

use bevy::prelude::*;

/// Marker component for the in-game score readout.
#[derive(Component)]
pub struct ScoreText;

/// Resource to hold the player's score.
#[derive(Resource)]
pub struct PlayerScore(pub i32);

/// Event to update the score in the UI.
#[derive(Event)]
pub struct ScoreEvent(pub i32);

/// Spawns the score readout across the top of the screen.
///
/// # Arguments
/// * `commands`: The commands to spawn UI elements.
pub fn setup_game_ui(mut commands: Commands) {
    commands.spawn((
        ScoreText,
        Text::new("Score: 0"),
        TextFont {
            font_size: FontSize::Vh(4.0),
            ..default()
        },
        TextLayout {
            justify: Justify::Center,
            ..default()
        },
        Node {
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Percent(5.0),
            ..default()
        },
    ));
}

/// Resets the score back to zero at the start of a new run.
///
/// # Arguments
/// * `player_score`: The `PlayerScore` resource to reset.
pub fn restart_score(mut player_score: ResMut<PlayerScore>) {
    player_score.0 = 0;
}

/// Adds to the player's score and updates the score text to match.
///
/// # Arguments
/// * `update`: The `ScoreEvent` event that triggered this observer.
/// * `query`: A query that retrieves the `ScoreText` component.
/// * `player_score`: A mutable reference to the `PlayerScore` resource.
pub fn on_score(
    update: On<ScoreEvent>,
    mut query: Query<&mut Text, With<ScoreText>>,
    mut player_score: ResMut<PlayerScore>,
) {
    player_score.0 += update.0;

    for mut text in query.iter_mut() {
        text.0 = format!("Score: {}", player_score.0);
    }
}
