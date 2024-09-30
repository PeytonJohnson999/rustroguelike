use bevy::prelude::*;

#[derive(Component)]
pub struct Title;



pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    let text_style: TextStyle = TextStyle {
        font: asset_server.load("fonts/Maname-Regular.ttf"),
        font_size: 40.0,
        color: Color::srgb(0.0, 0.0, 0.0),
        ..default()
    };
    let text_justification = JustifyText::Center;


    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Hello!", text_style.clone())
                .with_justify(text_justification),
            ..default()
        },
        Title,
    ));

}
