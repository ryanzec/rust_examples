use crate::game::mouse_position::MousePosition;
use crate::game::pan_camera::PanCameraDetails;
use bevy::prelude::*;

const CAMERA_ZOOM_STEP: f32 = 0.05;
const CAMERA_MINIMUM_ZOOM: f32 = 0.15;
const CAMERA_MAXIMUM_ZOOM: f32 = 3.0;

pub fn pan_camera_system(
    mouse_button: Res<Input<MouseButton>>,
    mouse_position_query: Query<&MousePosition>,
    projection_query: Query<&OrthographicProjection, With<Camera>>,
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
    mut pan_camera_details_query: Query<&mut PanCameraDetails>,
) {
    let projection = projection_query.get_single().unwrap();
    let mut camera_transform = camera_transform_query.get_single_mut().unwrap();
    let mouse_position = mouse_position_query.get_single().unwrap();
    let mut pan_camera_details = pan_camera_details_query.get_single_mut().unwrap();

    if pan_camera_details.is_tracking {
        println!(
            "is tracking: {}:{}:{} ({})",
            pan_camera_details.origin_camera_position.unwrap(),
            pan_camera_details.origin_mouse_position.unwrap(),
            pan_camera_details.current_mouse_position.unwrap(),
            pan_camera_details.current_mouse_position.unwrap()
                - pan_camera_details.origin_mouse_position.unwrap()
        );
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        println!("left mouse button clicked");
    }

    if mouse_button.just_pressed(MouseButton::Middle) {
        pan_camera_details.is_tracking = true;
        pan_camera_details.origin_camera_position = Some(camera_transform.translation.clone());
        pan_camera_details.origin_mouse_position = Some(mouse_position.viewport);
    }

    if mouse_button.pressed(MouseButton::Middle) {
        pan_camera_details.current_mouse_position = Some(mouse_position.viewport);

        let mut difference = pan_camera_details.current_mouse_position.unwrap()
            - pan_camera_details.origin_mouse_position.unwrap();

        // need to account for scale so the movements is based on the zoomed level
        difference.y *= 1.0 * projection.scale;

        // need to flip since we are using the viewport position which has 0,0 at top left
        // not bottom left
        difference.x *= -1.0 * projection.scale;

        camera_transform.translation =
            pan_camera_details.origin_camera_position.unwrap() + Vec3::from((difference, 0.0));
    }

    if mouse_button.just_released(MouseButton::Middle) {
        pan_camera_details.is_tracking = false;
    }

    if mouse_button.just_pressed(MouseButton::Right) {
        println!("right mouse button clicked");
    }
}
