use crate::game::Velocity;
use bevy::{prelude::*, sprite::collide_aabb};

const FUEL_CELL_SIZE: f32 = 8.0;

pub struct FuelPlugin;

impl Plugin for FuelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_fuel_cells_system)
            .add_system(fuel_cells_collision_system);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct FuelCell {
    pub capacity: f32,
    pub has_spawn_children: bool,
}

fn spawn_fuel_cells_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let w = window.width();
    let h = window.height();

    let texture_handle = asset_server.load("fuel.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(FUEL_CELL_SIZE, FUEL_CELL_SIZE),
        1,
        1,
        None,
        None,
    );
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
                    has_spawn_children: false,
                },
                Velocity::rand(),
            ));
    }
}

fn fuel_cells_collision_system(
    mut commands: Commands,
    mut fuel_cells_query: Query<(
        &Handle<TextureAtlas>,
        &Transform,
        &mut Velocity,
        &mut FuelCell,
    )>,
    cell_positions: Query<&Transform, With<FuelCell>>,
) {
    let mut combinations = fuel_cells_query.iter_combinations_mut();
    while let Some(
        [(a_sprite, a_transform, mut a_velocity, mut a_cell), (_, b_transform, mut b_velocity, mut b_cell)],
    ) = combinations.fetch_next()
    {
        if collide_aabb::collide(
            a_transform.translation,
            Vec2::new(FUEL_CELL_SIZE, FUEL_CELL_SIZE),
            b_transform.translation,
            Vec2::new(FUEL_CELL_SIZE, FUEL_CELL_SIZE),
        )
        .is_some()
        {
            let new_velocity_x = (a_velocity.x + b_velocity.x) / 2.0;
            let new_velocity_y = (a_velocity.y + b_velocity.y) / 2.0;
            a_velocity.x = new_velocity_x;
            b_velocity.x = new_velocity_x;
            a_velocity.y = new_velocity_y;
            b_velocity.y = new_velocity_y;

            if !a_cell.has_spawn_children || !b_cell.has_spawn_children {
                let new_cell_x = a_transform.translation.x - 10.0 * a_velocity.x;
                let new_cell_y = a_transform.translation.y - 10.0 * a_velocity.y;

                let has_space_to_spawn = cell_positions.iter().all(|cell_transform| {
                    collide_aabb::collide(
                        Vec3::new(new_cell_x, new_cell_y, 0.0),
                        Vec2::new(FUEL_CELL_SIZE, FUEL_CELL_SIZE),
                        cell_transform.translation,
                        Vec2::new(FUEL_CELL_SIZE, FUEL_CELL_SIZE),
                    )
                    .is_none()
                });

                a_cell.has_spawn_children = true;
                b_cell.has_spawn_children = true;

                if has_space_to_spawn {
                    commands.spawn((
                        SpriteSheetBundle {
                            texture_atlas: a_sprite.clone(),
                            transform: Transform {
                                scale: Vec3::new(1.0, 1.0, 0.0),
                                // rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0),
                                translation: Vec3::new(new_cell_x, new_cell_y, 0.0),
                                ..default()
                            },
                            ..default()
                        },
                        FuelCell {
                            capacity: rand::random::<f32>() * 10.0 + 10.0, // 10..20
                            has_spawn_children: false,
                        },
                        Velocity {
                            x: -new_velocity_x,
                            y: -new_velocity_y,
                        },
                    ));
                }
            }
        }
    }
}
