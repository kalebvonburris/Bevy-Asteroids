use bevy::prelude::*;

#[derive(Component)]
pub struct Explosion {
    pub start_time: f32,
}

impl Explosion {
    pub fn new(start_time: f32) -> Self {
        Self { start_time }
    }
}

#[derive(Resource)]
pub struct ExplosionConfig {
    pub mesh: Handle<Mesh>,
    pub asteroid_color: Handle<ColorMaterial>,
    pub player_color: Handle<ColorMaterial>,
}

pub fn setup_explosions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create a mesh for the explosion
    let explosion_mesh = Annulus::new(5.0, 7.0);

    // Create a material for the explosion
    let asteroid_color = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::new(
        1.0, 1.0, 1.0, 0.5,
    ))));

    let player_color = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::new(
        1.0, 0.0, 0.0, 0.86,
    ))));

    // Create an explosion config
    let explosion_config = ExplosionConfig {
        mesh: meshes.add(explosion_mesh),
        asteroid_color,
        player_color,
    };

    commands.insert_resource(explosion_config);
}

pub fn create_explosion(
    commands: &mut Commands,
    transform: Transform,
    explosion_config: &Res<ExplosionConfig>,
    time: &Res<Time>,
    player: bool,
) {
    commands
        .spawn((
            Mesh2d(explosion_config.mesh.clone()),
            MeshMaterial2d(if player {
                explosion_config.player_color.clone()
            } else {
                explosion_config.asteroid_color.clone()
            }),
            transform,
        ))
        .insert(Explosion::new(time.elapsed_secs()));
}
