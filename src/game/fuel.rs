use crate::game::Velocity;
use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct FuelCell {
    pub capacity: f32,
}

pub fn spawn_fuel_cells(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let w = window.width();
    let h = window.height();

    let texture_handle = asset_server.load("fuel.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for _ in 0..50 {
        commands
            // spawn the spaceship
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform {
                        scale: Vec3::new(1.0, 1.0, 0.0),
                        // rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0),
                        translation: Vec3::new(
                            rand::random::<f32>() * w - w / 2.0,
                            rand::random::<f32>() * h - h / 2.0,
                            0.0,
                        ),
                        ..default()
                    },
                    ..default()
                },
                FuelCell {
                    capacity: rand::random::<f32>() * 10.0 + 10.0, // 10..20
                },
                Velocity {
                    x: rand::random::<f32>() * 20.0 - 10.0, // -10..10
                    y: rand::random::<f32>() * 20.0 - 10.0, // -10..10
                },
            ));
    }
}
