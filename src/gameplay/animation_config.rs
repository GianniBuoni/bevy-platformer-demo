use crate::prelude::*;
use std::time::Duration;

#[derive(Component, Default)]
pub struct AnimationConfig {
    pub first_sprite: usize,
    pub sprite_count: usize,
    pub relative_current_index: usize,
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
            relative_current_index: 0,
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
    pub fn get_new_index(&mut self) -> usize {
        self.relative_current_index += 1;
        self.relative_current_index % self.sprite_count + self.first_sprite
    }
}

// Default comfigurations for animated LDTK entites
impl From<&EntityInstance> for AnimationConfig {
    fn from(value: &EntityInstance) -> Self {
        match value.identifier.as_str() {
            "PalmTop" => AnimationConfig::new(0, 4, 4, true),
            "Candle" => AnimationConfig::new(32, 6, 10, true),
            "CandleLight" => AnimationConfig::new(40, 4, 4, true),
            "ChainSmall" => AnimationConfig::new(16, 8, 2, true),
            "ChainBig" => AnimationConfig::new(24, 8, 2, true),
            "Window" => AnimationConfig::new(48, 72, 10, true),
            _ => AnimationConfig::new(0, 4, 10, true),
        }
    }
}
