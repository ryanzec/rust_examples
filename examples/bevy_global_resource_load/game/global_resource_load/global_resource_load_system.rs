use crate::game::global_resource_load::InterfaceClickSound;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn global_resource_load_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_sever: Res<AssetServer>,
) -> () {
    // we are guaranteed to have a value here so unwrap should be fine
    // @todo(resource) not sure if there is a better way to handle this without unwrap()
    let window: &Window = window_query.get_single().unwrap();
    let handle: Handle<AudioSource> = asset_sever.load("audio/interface/click_001.ogg");

    commands.insert_resource(InterfaceClickSound(handle));
}
