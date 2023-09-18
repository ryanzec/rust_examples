use bevy::prelude::*;
use game::core::CorePlugin;
use game::camera::CameraPlugin;

mod game;

fn main() -> () {
    let mut main_app = App::new();

    main_app.add_plugins(CorePlugin);
    main_app.add_plugins(CameraPlugin);

    main_app.run();
}