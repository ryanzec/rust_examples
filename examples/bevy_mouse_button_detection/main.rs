use crate::game::mouse_wheel_scroll::MouseWheelScrollPlugin;
use bevy::prelude::*;
use game::camera::CameraPlugin;
use game::core::CorePlugin;
use game::mouse_position::MousePositionPlugin;
use game::pan_camera::PanCameraPlugin;
use game::sprite_renderer::SpriteRendererPlugin;

mod game;

fn main() -> () {
    let mut main_app = App::new();

    main_app.add_plugins(CorePlugin);
    main_app.add_plugins(CameraPlugin);
    main_app.add_plugins(SpriteRendererPlugin);
    main_app.add_plugins(PanCameraPlugin);
    main_app.add_plugins(MousePositionPlugin);
    main_app.add_plugins(MouseWheelScrollPlugin);

    main_app.run();
}
