use bevy::prelude::*;
use crate::game::sprite_renderer::add_sprite_renderer;

pub struct SpriteRendererPlugin;

impl Plugin for SpriteRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_sprite_renderer);
    }
}