use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn camera_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> () {
    // we are guaranteed to have a value here so unwrap should be fine
    // @todo(resource) not sure if there is a better way to handle this without unwrap()
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
