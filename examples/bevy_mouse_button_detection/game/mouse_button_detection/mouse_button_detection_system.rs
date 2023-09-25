use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const CAMERA_ZOOM_STEP: f32 = 0.05;
const CAMERA_MINIMUM_ZOOM: f32 = 0.15;
const CAMERA_MAXIMUM_ZOOM: f32 = 3.0;

pub fn mouse_button_detection_system(
    mouse_button: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        println!("left mouse button clicked");
    }

    if mouse_button.just_pressed(MouseButton::Middle) {
        // we are guaranteed to have a value here so unwrap should be fine
        // @todo(resource) not sure if there is a better way to handle this without unwrap()
        let window: &Window = window_query.get_single().unwrap();
        let (camera, camera_transform) = camera_query.get_single().unwrap();
        let cursor_position = window.cursor_position().unwrap();

        println!(
            "{:?}",
            camera
                .viewport_to_world(camera_transform, cursor_position)
                .unwrap()
                .origin
        );
    }

    if mouse_button.just_pressed(MouseButton::Right) {
        println!("right mouse button clicked");
    }

    if mouse_button.pressed(MouseButton::Middle) == false {
        return;
    }
}
