use crate::game::frames_per_second::FramesPerSecondDisplay;
use bevy::prelude::*;

pub fn frames_per_second_display_system(
    mut frames_per_second_display_query: Query<&mut FramesPerSecondDisplay>,
    time: Res<Time>,
) -> () {
    if let Ok(mut frames_per_second_display) = frames_per_second_display_query.get_single_mut() {
        frames_per_second_display.latest_frames_delta += time.delta_seconds();
        frames_per_second_display.currently_collected_count += 1;

        if frames_per_second_display.currently_collected_count
            < frames_per_second_display.collected_limit
        {
            return;
        }

        println!(
            "FPS: {}",
            (1 * frames_per_second_display.currently_collected_count) as f32
                / frames_per_second_display.latest_frames_delta
        );

        frames_per_second_display.latest_frames_delta = 0.0;
        frames_per_second_display.currently_collected_count = 0;
    }
}
