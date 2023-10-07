use crate::game::mouse_position::MousePosition;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const CAMERA_ZOOM_STEP: f32 = 0.05;
const CAMERA_MINIMUM_ZOOM: f32 = 0.15;
const CAMERA_MAXIMUM_ZOOM: f32 = 3.0;

pub fn initialize_mouse_position_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    // commands.spawn(MousePosition { mouse_position: None });
    // we are guaranteed to have a value here so unwrap should be fine
    // @todo(resource) not sure if there is a better way to handle this without unwrap()
    let window = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_query.get_single().unwrap();

    let camera_position = window.cursor_position();

    // the mouse might not be on the game window at startup so this handles that case
    if camera_position.is_none() {
        // not using None as if the user is going to need this is will be setup since the
        // mouse needs to enter the game window for the user to interact with the game
        commands.spawn(MousePosition {
            world: Vec3::new(0.0, 0.0, 0.0),
            viewport: Vec2::new(0.0, 0.0),
        });

        return;
    }

    let cursor_position = camera_position.unwrap();

    let mouse_position = camera
        .viewport_to_world(camera_transform, cursor_position)
        .unwrap()
        .origin;

    println!("initialize mouse with: {:?}", mouse_position);

    commands.spawn(MousePosition {
        world: mouse_position,
        viewport: cursor_position,
    });
}
