mod plugin;

mod mesh_2d_render_system;
mod mesh_2d_update_system;
mod utils;

pub use plugin::Mesh2DRenderPlugin;

pub use utils::get_tile_index;
pub use utils::get_tile_vertices;
pub use utils::get_triangle_for_tile;
pub use utils::get_uvs_for_tile;

pub use mesh_2d_render_system::mesh_2d_render_system;
pub use mesh_2d_update_system::mesh_2d_update_system;
