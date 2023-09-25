use crate::game::frames_per_second::FramesPerSecondDisplay;
use bevy::prelude::*;

pub fn frames_per_second_system(mut commands: Commands) -> () {
    commands.spawn(FramesPerSecondDisplay::default());
}
