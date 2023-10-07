use crate::game::mouse_position::{initialize_mouse_position_system, mouse_position_system};
use bevy::prelude::*;
use bevy::render::camera::CameraUpdateSystem;

pub struct MousePositionPlugin;

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            initialize_mouse_position_system.after(CameraUpdateSystem),
        );

        app.add_systems(Update, mouse_position_system);
    }
}
