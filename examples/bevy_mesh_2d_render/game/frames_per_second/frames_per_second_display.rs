use bevy::prelude::Component;

#[derive(Component)]
pub struct FramesPerSecondDisplay {
    pub latest_frames_delta: f32,
    pub collected_limit: usize,
    pub currently_collected_count: usize,
}

impl Default for FramesPerSecondDisplay {
    fn default() -> Self {
        FramesPerSecondDisplay {
            latest_frames_delta: f32::default(),
            collected_limit: 180,
            currently_collected_count: usize::default(),
        }
    }
}
