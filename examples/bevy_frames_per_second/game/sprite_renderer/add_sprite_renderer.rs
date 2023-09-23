use bevy::math::vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn add_sprite_renderer(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_sever: Res<AssetServer>,
) -> () {
    // we are guaranteed to have a value here so unwrap should be fine
    // @todo(resource) not sure if there is a better way to handle this without unwrap()
    let window: &Window = window_query.get_single().unwrap();

    for i in 0..100 {
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz((i * 13) as f32, window.height() / 2.0, 0.01),
            texture: asset_sever.load("sprites/planet1.png"),
            sprite: Sprite {
                custom_size: Some(vec2(100.0, 100.0)),
                color: Color::Rgba {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                    alpha: 1.0,
                },
                ..default()
            },
            ..default()
        });
    }
}
