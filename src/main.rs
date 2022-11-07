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
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let texture_handle = asset_server.load("ship.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
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
            fuel: 100.0,
            thrust: 100.0,
        });
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
            let (_, _, z) = transform.rotation.to_euler(EulerRot::YXZ);
            velocity.x -= engine.thrust * time.delta_seconds() * z.sin();
            velocity.y += engine.thrust * time.delta_seconds() * z.cos();
            engine.fuel -= engine.thrust * time.delta_seconds();
            engine.fuel = engine.fuel.clamp(0.0, 1000.0);
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
        .add_system(rotate_ship_system)
        .add_system(velocity_system)
        .add_system(engine_system)
        .run()
}
