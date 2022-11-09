use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

#[derive(Inspectable, Component)]
struct InspectableType;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Starship {
    rotation_speed: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Engine {
    fuel: f32,
    thrust: f32,
    is_on: bool,
}

#[derive(Component, Deref, DerefMut, Reflect, Default)]
#[reflect(Component)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct FuelStatusText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let texture_handle = asset_server.load("ship.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let font = asset_server.load("fonts/Monocraft.otf");

    commands
        // spawn the spaceship
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(1.0, 1.0, 0.0),
                // rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0),
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Starship {
            rotation_speed: 1.0,
        })
        .insert(Velocity { x: 0.0, y: 0.0 })
        .insert(Engine {
            fuel: 1000.0,
            thrust: 100.0,
            is_on: false,
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

    // spawn the fuel status text
    commands
        .spawn()
        .insert_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Fuel: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "1000",
                    TextStyle {
                        font,
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.5, 1.0),
                    },
                ),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexStart,
                ..default()
            }),
        )
        .insert(FuelStatusText);
}

fn rotate_ship_system(
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

fn velocity_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let w = window.width();
    let h = window.height();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();

        if transform.translation.x < -w / 2.0 {
            transform.translation.x += w;
        } else if transform.translation.x > w / 2.0 {
            transform.translation.x -= w;
        }

        if transform.translation.y < -h / 2.0 {
            transform.translation.y += h;
        } else if transform.translation.y > h / 2.0 {
            transform.translation.y -= h;
        }
    }
}

fn engine_system(
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

fn fuel_text_system(
    mut query_text: Query<&mut Text, With<FuelStatusText>>,
    query_starship_engine: Query<&Engine, With<Starship>>,
) {
    let mut text = query_text.single_mut();
    let engine = query_starship_engine.single();
    text.sections[1].value = format!("{:.0}", engine.fuel);
}

fn animate_sprite(
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

fn main() {
    App::new()
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<InspectableType>()
        .register_type::<Starship>()
        .register_type::<Engine>()
        .register_type::<Velocity>()
        .register_type::<AnimationTimer>()
        .add_system(rotate_ship_system)
        .add_system(velocity_system)
        .add_system(engine_system)
        .add_system(animate_sprite)
        .add_system(fuel_text_system)
        .run()
}
