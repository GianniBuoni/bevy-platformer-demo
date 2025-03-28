use crate::prelude::*;
use std::time::Duration;

#[derive(Component, Default)]
pub struct AnimationConfig {
    pub first_sprite: usize,
    pub sprite_count: usize,
    pub frame_timer: Timer,
    pub animation_timer: Option<Timer>,
}

impl AnimationConfig {
    pub fn new(
        first_sprite: usize,
        sprite_count: usize,
        fps: u8,
        repeat: bool,
    ) -> Self {
        Self {
            first_sprite,
            sprite_count,
            frame_timer: Self::timer_from_fps(fps),
            animation_timer: Self::timer_from_frame_count(
                fps,
                sprite_count,
                repeat,
            ),
        }
    }
    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / fps as f32),
            TimerMode::Repeating,
        )
    }
    fn timer_from_frame_count(
        fps: u8,
        sprite_count: usize,
        repeat: bool,
    ) -> Option<Timer> {
        match repeat {
            true => None,
            false => Some(Timer::new(
                Duration::from_secs_f32(
                    (1.0 / fps as f32) * sprite_count as f32,
                ),
                TimerMode::Once,
            )),
        }
    }
}

// Default comfigurations for animated LDTK entites
impl From<&EntityInstance> for AnimationConfig {
    fn from(value: &EntityInstance) -> Self {
        match value.identifier.as_str() {
            "Player" => AnimationConfig::new(0, 4, 6, true),
            _ => AnimationConfig::new(1, 2, 6, true),
        }
    }
}
