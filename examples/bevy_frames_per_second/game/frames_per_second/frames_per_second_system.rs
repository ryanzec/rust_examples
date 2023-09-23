use bevy::prelude::*;

pub fn frames_per_second_system(time: Res<Time>) -> () {
    println!("FPS: {}", 1.0 / time.delta_seconds());
}
