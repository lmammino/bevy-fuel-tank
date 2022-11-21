use crate::game::{FuelCell, Velocity};
use bevy::prelude::*;

const SHIP_SIZE: f32 = 32.0;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Starship {
    pub rotation_speed: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Engine {
    pub fuel: f32,
    pub capacity: f32,
    pub thrust: f32,
    pub is_on: bool,
}

#[derive(Component, Deref, DerefMut, Reflect, Default)]
#[reflect(Component)]
pub struct AnimationTimer(Timer);

pub fn spawn_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("ship.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(SHIP_SIZE, SHIP_SIZE),
        2,
        2,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        // spawn the spaceship
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    scale: Vec3::new(1.0, 1.0, 0.0),
                    // rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0),
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Starship {
                rotation_speed: 1.0,
            },
            Velocity { x: 0.0, y: 0.0 },
            Engine {
                fuel: 1000.0,
                capacity: 1000.0,
                thrust: 100.0,
                is_on: false,
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
}

pub fn rotate_ship_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Starship)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (mut transform, starship) = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        transform.rotation *= Quat::from_rotation_z(time.delta_seconds() * starship.rotation_speed);
    } else if keyboard_input.pressed(KeyCode::Right) {
        transform.rotation *=
            Quat::from_rotation_z(time.delta_seconds() * -starship.rotation_speed);
    }
}

pub fn engine_system(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Transform, &mut Engine)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut velocity, transform, mut engine) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) && engine.fuel > 0.0 {
            engine.is_on = true;
            let (_, _, z) = transform.rotation.to_euler(EulerRot::YXZ);
            velocity.x -= engine.thrust * time.delta_seconds() * z.sin();
            velocity.y += engine.thrust * time.delta_seconds() * z.cos();
            engine.fuel -= engine.thrust * time.delta_seconds();
            engine.fuel = engine.fuel.clamp(0.0, 1000.0);
        } else {
            engine.is_on = false;
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Engine,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, engine) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() && engine.is_on {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            if sprite.index == 0 {
                sprite.index = 1;
            }
        }

        if !engine.is_on {
            sprite.index = 0;
        }
    }
}

pub fn collide_with_fuel_system(
    mut commands: Commands,
    mut fuel_query: Query<(Entity, &Transform, &FuelCell)>,
    mut ship_query: Query<(&Transform, &mut Engine), With<Starship>>,
) {
    let (ship_transform, mut engine) = ship_query.single_mut();

    for (fuel_entity, fuel_transform, fuel_cell) in fuel_query.iter_mut() {
        let distance = ship_transform
            .translation
            .distance(fuel_transform.translation);
        if distance < SHIP_SIZE / 2.0 {
            engine.fuel += fuel_cell.capacity;
            engine.fuel = engine.fuel.clamp(0.0, engine.capacity);
            commands.entity(fuel_entity).despawn();
        }
    }
}
