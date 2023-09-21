use crate::game::camera::CameraPlugin;
use crate::game::core::CorePlugin;
use crate::game::global_resource_load::LoadGlobalResourcePlugin;
use crate::game::sprite_renderer::SpriteRendererPlugin;
use bevy::prelude::*;

mod game;

fn main() -> () {
    let mut main_app = App::new();

    main_app.add_plugins(CorePlugin);
    main_app.add_plugins(CameraPlugin);
    main_app.add_plugins(SpriteRendererPlugin);
    main_app.add_plugins(LoadGlobalResourcePlugin);

    main_app.run();
}
