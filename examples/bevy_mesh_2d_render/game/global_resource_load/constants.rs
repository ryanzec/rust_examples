use bevy::audio::AudioSource;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Handle, Resource};
use bevy::sprite::ColorMaterial;

#[derive(Resource)]
pub struct InterfaceClickSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct MeshMaterial(pub Handle<ColorMaterial>);
