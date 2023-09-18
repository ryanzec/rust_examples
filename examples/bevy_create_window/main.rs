use bevy::prelude::*;
use game::core::CorePlugin;

mod game;

fn main() -> () {
    let mut main_app = App::new();

    main_app.add_plugins(CorePlugin);

    main_app.run();
}