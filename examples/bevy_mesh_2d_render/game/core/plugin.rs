use bevy::prelude::*;
use bevy::window::PresentMode;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // this disables vsync (allowing better tracking of FPS / performance changes
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }));
    }
}
