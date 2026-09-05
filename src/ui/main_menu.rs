//! The title screen, shown before the first game.

use bevy::prelude::*;

use crate::GameState;

/// Marker component for the main menu UI.
#[derive(Component)]
pub struct MainMenu;

/// Sets up the main menu UI for the game.
///
/// # Arguments
/// * `commands`: The commands to spawn UI elements.
pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenu,
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
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
                font_size: FontSize::Px(40.0),
                ..default()
            },
            Text::new("Welcome to Asteroids!\nPress 'Space' to Start"),
        ));
}

/// Handles input for the main menu.
///
/// # Arguments
/// * `keyboard_input`: The `ButtonInput<KeyCode>` resource to check for player input.
/// * `next_state`: The `NextState<GameState>` resource to start the game.
pub fn handle_main_menu_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Transition to the game state when space is pressed
        next_state.set(GameState::Game);
    }
}
