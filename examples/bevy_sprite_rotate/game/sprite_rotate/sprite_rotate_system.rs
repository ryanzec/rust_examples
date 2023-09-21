use bevy::prelude::*;

pub fn sprite_rotate_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_query: Query<&mut Transform, With<Sprite>>,
    time: Res<Time>,
) -> () {
    if let Ok(mut transform) = sprite_query.get_single_mut() {
        let mut rotation_difference: f32 = 0.0;

        if keyboard_input.pressed(KeyCode::A) {
            rotation_difference = time.delta_seconds() * 5.0;
        } else if keyboard_input.pressed(KeyCode::D) {
            rotation_difference = time.delta_seconds() * -5.0;
        }

        transform.rotate(Quat::from_rotation_z(rotation_difference))
    }
}
