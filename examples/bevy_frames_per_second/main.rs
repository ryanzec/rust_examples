use crate::game::frames_per_second::FramesPerSecondPlugin;
use bevy::prelude::*;
use game::camera::CameraPlugin;
use game::core::CorePlugin;
use game::sprite_renderer::SpriteRendererPlugin;

mod game;

fn main() -> () {
    let mut main_app = App::new();

    main_app.add_plugins(CorePlugin);
    main_app.add_plugins(CameraPlugin);
    main_app.add_plugins(SpriteRendererPlugin);
    main_app.add_plugins(FramesPerSecondPlugin);

    main_app.run();
}
