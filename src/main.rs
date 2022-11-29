use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use bevy_prototype_lyon::prelude::*;

mod game;
use game::*;

#[derive(Inspectable, Component)]
struct InspectableType;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<InspectableType>()
        .register_type::<Starship>()
        .register_type::<Engine>()
        .register_type::<Velocity>()
        .register_type::<AnimationTimer>()
        .add_plugin(CommonComponentsPlugin)
        .add_plugin(ShipPlugin)
        .add_plugin(AsteroidPlugin)
        .add_plugin(FuelPlugin)
        .add_plugin(HudPlugin)
        .add_startup_system(setup)
        .run()
}
