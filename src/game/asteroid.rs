use crate::Velocity;
use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Polygon};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_asteroids_system);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Asteroid;

pub const MAX_RADIUS: f32 = 32.0;
const MIN_RADIUS: f32 = 12.0;

fn generate_rnd_shape() -> Polygon {
    let mut points = Vec::new();

    let sides = rand::random::<u8>() % 8 + 5;
    let teta = 2.0 * std::f32::consts::PI / sides as f32;
    for i in 0..sides {
        let r = rand::random::<f32>() * (MAX_RADIUS - MIN_RADIUS) + MIN_RADIUS;
        let x = r * (teta * i as f32).cos();
        let y = r * (teta * i as f32).sin();
        points.push(Vec2::new(x, y));
    }

    Polygon {
        points,
        closed: true,
    }
}

fn spawn_asteroids_system(mut commands: Commands) {
    for _ in 0..10 {
        let shape = generate_rnd_shape();
        commands
            .spawn(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::BLACK),
                    outline_mode: StrokeMode::new(Color::WHITE, 2.0),
                },
                Transform {
                    translation: Vec3::new(
                        rand::random::<f32>() * 800.0 - 400.0,
                        rand::random::<f32>() * 600.0 - 300.0,
                        0.0,
                    ),
                    ..Default::default()
                },
            ))
            .insert(Velocity::rand())
            .insert(Asteroid);
    }
}
