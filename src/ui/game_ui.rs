//! Contains the UI element for handling the player's score in the game.

use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

pub fn setup_game_ui(mut commands: Commands) {
    commands.spawn((
        ScoreText,
        Text::new("Score: 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Percent(5.0),
            ..default()
        },
    ));
}

pub fn despawn_game_ui(mut commands: Commands, query: Query<Entity, With<ScoreText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn restart_score(mut player_score: ResMut<PlayerScore>) {
    player_score.0 = 0;
}

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
