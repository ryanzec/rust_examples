use crate::game::mouse_wheel_scroll::mouse_wheel_scroll_system;
use bevy::prelude::*;

pub struct MouseWheelScrollPlugin;

impl Plugin for MouseWheelScrollPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_wheel_scroll_system);
    }
}
