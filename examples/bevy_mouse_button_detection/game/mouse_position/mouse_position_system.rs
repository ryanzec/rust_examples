use crate::game::mouse_position::MousePosition;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const CAMERA_ZOOM_STEP: f32 = 0.05;
const CAMERA_MINIMUM_ZOOM: f32 = 0.15;
const CAMERA_MAXIMUM_ZOOM: f32 = 3.0;

pub fn mouse_position_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut mouse_position_query: Query<&mut MousePosition>,
) {
    let window = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_query.get_single().unwrap();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let mouse_position = camera
        .viewport_to_world(camera_transform, cursor_position)
        .unwrap()
        .origin;

    mouse_position_query.get_single_mut().unwrap().world = mouse_position;
    mouse_position_query.get_single_mut().unwrap().viewport = cursor_position;
}
