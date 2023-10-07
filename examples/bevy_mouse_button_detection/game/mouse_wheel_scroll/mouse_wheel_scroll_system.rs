use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

const CAMERA_ZOOM_STEP: f32 = 0.03;
const CAMERA_MINIMUM_ZOOM: f32 = 0.05;
const CAMERA_MAXIMUM_ZOOM: f32 = 150.0;

pub fn mouse_wheel_scroll_system(
    mut scroll_evr: EventReader<MouseWheel>,
    mut projection_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    if let Ok(mut projection) = projection_query.get_single_mut() {
        for ev in scroll_evr.iter() {
            let scroll_value: f32;

            match ev.unit {
                MouseScrollUnit::Line => {
                    scroll_value = ev.y * CAMERA_ZOOM_STEP;
                }
                MouseScrollUnit::Pixel => {
                    scroll_value = ev.y * CAMERA_ZOOM_STEP;
                }
            }

            let new_scale = f32::clamp(
                // need to inverse the scroll value to have zoom work as expected
                projection.scale + (scroll_value * -1.0),
                CAMERA_MINIMUM_ZOOM,
                CAMERA_MAXIMUM_ZOOM,
            );

            if new_scale == projection.scale {
                println!("no change to zoom scale");

                return;
            }

            println!("new scale: {}", new_scale);

            projection.scale = new_scale;
        }
    }
}
