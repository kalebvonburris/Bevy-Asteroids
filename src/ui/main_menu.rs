use bevy::{prelude::*, text::LineHeight};

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
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont {
                font_size: 40.0,
                line_height: LineHeight::RelativeToFont(2.0),
                ..default()
            },
            Text::new("Welcome to Asteroids!\nPress 'Space' to Start"),
        ));
}

/// Handles input for the main menu.
pub fn handle_main_menu_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Transition to the game state when space is pressed
        next_state.set(GameState::Game);
    }
}

/// Deletes the main menu UI from the game.
///
/// # Arguments
/// * `commands`: The commands to despawn the main menu UI.
/// * `query`: A query that retrieves all entities with the `MainMenu` component.
pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
