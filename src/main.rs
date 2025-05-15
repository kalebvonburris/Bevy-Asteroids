#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

use asteroids::*;

fn main() -> AppExit {
    App::new().add_plugins(AsteroidsPlugin).run()
}
