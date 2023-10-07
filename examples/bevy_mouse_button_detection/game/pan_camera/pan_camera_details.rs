use bevy::prelude::*;

#[derive(Component)]
pub struct PanCameraDetails {
    pub origin_camera_position: Option<Vec3>,
    pub origin_mouse_position: Option<Vec2>,
    pub current_mouse_position: Option<Vec2>,
    pub is_tracking: bool,
}
