use bevy::prelude::*;

pub const ALIEN_SHIP_POINTS: &[Vec3] = &[
    Vec3::new(-5.0, -5.0, 1.0),
    Vec3::new(0.0, 5.0, 1.0),
    Vec3::new(5.0, -5.0, 1.0),
    Vec3::new(0.0, -2.5, 1.0),
    Vec3::new(-5.0, -5.0, 1.0),
];

#[derive(Component)]
pub struct Alien {
    health: u8,
}

#[derive(Resource)]
pub struct AlienResource {
    pub mesh: Handle<Mesh>,
    pub color: Handle<ColorMaterial>,
}
