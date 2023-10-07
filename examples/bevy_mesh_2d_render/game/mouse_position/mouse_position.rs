use bevy::prelude::*;

#[derive(Component)]
pub struct MousePosition {
    pub viewport: Vec2,
    pub world: Vec3,
}
