use crate::game::global_resource_load::InterfaceClickSound;
use bevy::prelude::*;

pub fn play_sound_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    audio_resource: Res<InterfaceClickSound>,
) -> () {
    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn(AudioBundle {
            source: audio_resource.0.clone(),
            settings: PlaybackSettings::DESPAWN,
            ..default()
        });
    }
}
