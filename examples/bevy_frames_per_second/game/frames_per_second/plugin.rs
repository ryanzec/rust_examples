use bevy::prelude::*;

use crate::game::frames_per_second::frames_per_second_system;

pub struct FramesPerSecondPlugin;

impl Plugin for FramesPerSecondPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, frames_per_second_system);
    }
}
