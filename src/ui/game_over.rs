use bevy::prelude::*;

use crate::{ui::PlayerScore, GameState};

pub fn setup_game_over_ui(mut commands: Commands, score: Res<PlayerScore>) {
    commands.spawn((
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        
    )).with_child((
        TextLayout::new_with_justify(JustifyText::Center),
        TextFont {
            font_size: 40.0,
            line_height: bevy::text::LineHeight::RelativeToFont(2.0),
            ..default()
        },
        Text::new(format!("Game Over\nFinal Score: {}", score.0)),
    ));
}

pub fn handle_game_over_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        // Restart the game when 'R' is pressed
        next_state.set(GameState::Game);
    }
}
