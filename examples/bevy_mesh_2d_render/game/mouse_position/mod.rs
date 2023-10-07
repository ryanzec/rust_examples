mod plugin;

mod initialize_mouse_position_system;
mod mouse_position;
mod mouse_position_system;

pub use plugin::MousePositionPlugin;

pub use initialize_mouse_position_system::initialize_mouse_position_system;
pub use mouse_position::MousePosition;
pub use mouse_position_system::mouse_position_system;
