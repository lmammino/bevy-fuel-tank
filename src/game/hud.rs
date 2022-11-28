use crate::game::{Engine, Starship};
use bevy::prelude::*;

#[derive(Component)]
pub struct FuelStatusText;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Monocraft.otf");

    // spawn the fuel status text
    commands.spawn((
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
        ]),
        FuelStatusText,
    ));
}

pub fn fuel_text_system(
    mut query_text: Query<&mut Text, With<FuelStatusText>>,
    query_starship_engine: Query<&Engine, With<Starship>>,
) {
    if let Ok(engine) = query_starship_engine.get_single() {
        let mut text = query_text.single_mut();
        text.sections[1].value = format!("{:.0}", engine.fuel);
    }
}
