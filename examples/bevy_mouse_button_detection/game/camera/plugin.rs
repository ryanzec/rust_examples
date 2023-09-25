use bevy::prelude::*;

use crate::game::camera::camera_system;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_system);
    }
}
