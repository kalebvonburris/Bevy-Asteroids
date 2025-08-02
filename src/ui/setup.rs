//! Setup for the user interface components of the application.

use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        ScoreText,
        Text::new("Score: 0"),
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
