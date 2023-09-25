use crate::game::mesh_2d_render::mesh_2d_render_system;
use bevy::prelude::*;

pub struct Mesh2DRenderPlugin;

impl Plugin for Mesh2DRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, mesh_2d_render_system);
    }
}
