use crate::game::global_resource_load::global_resource_load_system;
use crate::game::global_resource_load::play_sound_system;
use bevy::prelude::*;

pub struct LoadGlobalResourcePlugin;

impl Plugin for LoadGlobalResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, global_resource_load_system);
        app.add_systems(Update, play_sound_system);
    }
}
