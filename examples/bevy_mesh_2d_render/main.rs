use crate::game::frames_per_second::FramesPerSecondPlugin;
use crate::game::global_resource_load::LoadGlobalResourcePlugin;
use crate::game::mouse_wheel_scroll::MouseWheelScrollPlugin;
use bevy::prelude::*;
use game::camera::CameraPlugin;
use game::core::CorePlugin;
use game::mesh_2d_render::Mesh2DRenderPlugin;

mod game;

fn main() -> () {
    let mut main_app = App::new();

    main_app.add_plugins(CorePlugin);
    main_app.add_plugins(LoadGlobalResourcePlugin);
    main_app.add_plugins(CameraPlugin);
    main_app.add_plugins(Mesh2DRenderPlugin);
    main_app.add_plugins(FramesPerSecondPlugin);
    main_app.add_plugins(MouseWheelScrollPlugin);

    main_app.run();
}
