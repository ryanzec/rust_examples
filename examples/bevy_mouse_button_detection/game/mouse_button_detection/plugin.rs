use crate::game::mouse_button_detection::mouse_button_detection_system;
use bevy::prelude::*;

pub struct MouseButtonDetectionPlugin;

impl Plugin for MouseButtonDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_button_detection_system);
    }
}
