use crate::game::pan_camera::{initialize_pan_camera_system, pan_camera_system};
use bevy::prelude::*;

pub struct PanCameraPlugin;

impl Plugin for PanCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, initialize_pan_camera_system);
        app.add_systems(Update, pan_camera_system);
    }
}
