//! # User Interface
//!
//! This module contains the user interface components of the application.

pub mod game_ui;
pub use game_ui::*;
pub mod main_menu;
pub use main_menu::*;
pub mod game_over;
pub use game_over::*;

use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, main_menu::setup_main_menu);
    }
}
