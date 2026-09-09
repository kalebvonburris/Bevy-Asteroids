use bevy::prelude::*;

use crate::alien::systems::setup_alien;

mod data;
mod systems;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_alien);
    }
}
