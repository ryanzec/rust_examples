use crate::game::sprite_bounce::sprite_bounce_system;
use bevy::prelude::*;

pub struct SpriteBounceSystem;

impl Plugin for SpriteBounceSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sprite_bounce_system);
    }
}
