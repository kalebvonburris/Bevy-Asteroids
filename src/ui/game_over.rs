//! The screen shown once the player ship has been destroyed.

use bevy::prelude::*;

use crate::{GameState, ui::PlayerScore};

/// Marker component for the game over UI.
#[derive(Component)]
pub struct GameOverMenu;

/// Sets up the game over UI, reporting the score the player finished on.
///
/// # Arguments
/// * `commands`: The commands to spawn UI elements.
/// * `score`: The `PlayerScore` resource holding the final score.
pub fn setup_game_over_ui(mut commands: Commands, score: Res<PlayerScore>) {
    commands
        .spawn((
            GameOverMenu,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_child((
            TextLayout {
                justify: Justify::Center,
                ..default()
            },
            TextFont {
                font_size: FontSize::Vh(5.0),
                ..default()
            },
            Text::new(format!(
                "Game Over\nFinal Score: {}\nPress 'R' to Restart",
                score.0
            )),
        ));
}

/// Handles input for the game over screen.
///
/// # Arguments
/// * `keyboard_input`: The `ButtonInput<KeyCode>` resource to check for player input.
/// * `next_state`: The `NextState<GameState>` resource to start a new game.
pub fn handle_game_over_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        // Restart the game when 'R' is pressed
        next_state.set(GameState::Game);
    }
}
