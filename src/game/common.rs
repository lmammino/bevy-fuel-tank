use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub fn velocity_system(
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
