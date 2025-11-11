//! # User Interface
//!
//! This module contains the user interface components of the application.

pub mod game_ui;
pub use game_ui::*;
pub mod main_menu;
pub use main_menu::*;
pub mod game_over;
pub use game_over::*;

use crate::GameState;
use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerScore(0));
        app.add_event::<ScoreEvent>();
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
        app.add_systems(
            Update,
            (handle_main_menu_input).run_if(in_state(GameState::MainMenu)),
        );
        app.add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
        app.add_systems(OnEnter(GameState::Game), (setup_game_ui, restart_score));
        app.add_systems(Update, (update_score).run_if(in_state(GameState::Game)));
    }
}
