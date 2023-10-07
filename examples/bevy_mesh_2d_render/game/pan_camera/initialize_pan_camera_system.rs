use crate::game::pan_camera::PanCameraDetails;
use bevy::prelude::*;

const CAMERA_ZOOM_STEP: f32 = 0.05;
const CAMERA_MINIMUM_ZOOM: f32 = 0.15;
const CAMERA_MAXIMUM_ZOOM: f32 = 3.0;

pub fn initialize_pan_camera_system(mut commands: Commands) {
    commands.spawn(PanCameraDetails {
        origin_camera_position: None,
        origin_mouse_position: None,
        current_mouse_position: None,
        is_tracking: false,
    });
}
