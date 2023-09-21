use crate::game::sprite_rotate::sprite_rotate_system;
use bevy::prelude::*;

pub struct SpriteRotatePlugin;

impl Plugin for SpriteRotatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sprite_rotate_system);
    }
}
