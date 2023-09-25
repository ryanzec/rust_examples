use crate::game::global_resource_load::{InterfaceClickSound, MeshMaterial};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn global_resource_load_system(
    mut commands: Commands,
    asset_sever: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> () {
    let handle: Handle<AudioSource> = asset_sever.load("audio/interface/click_001.ogg");
    let mesh_material = materials.add(ColorMaterial::from(asset_sever.load("sprites/tileset.png")));

    commands.insert_resource(InterfaceClickSound(handle));
    commands.insert_resource(MeshMaterial(mesh_material));
}
