use bevy::audio::AudioSource;
use bevy::prelude::{Handle, Resource};

#[derive(Resource)]
pub struct InterfaceClickSound(pub Handle<AudioSource>);
