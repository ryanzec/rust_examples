use bevy::prelude::*;
use game::camera::CameraPlugin;
use game::core::CorePlugin;
use game::mouse_wheel_scroll::MouseWheelScrollPlugin;
use game::sprite_renderer::SpriteRendererPlugin;

mod game;

fn main() -> () {
    let mut main_app = App::new();

    main_app.add_plugins(CorePlugin);
    main_app.add_plugins(CameraPlugin);
    main_app.add_plugins(SpriteRendererPlugin);
    main_app.add_plugins(MouseWheelScrollPlugin);

    main_app.run();
}
