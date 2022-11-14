use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

mod game;
use game::*;

#[derive(Inspectable, Component)]
struct InspectableType;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(spawn_ship)
        .add_startup_system(spawn_hud)
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
