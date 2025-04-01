use std::time::Duration;

use bevy::{
    prelude::Component,
    time::{Timer, TimerMode},
};

#[derive(Component, Default)]
pub struct Config {
    first_index: usize,
    sprite_count: usize,
    relative_current_index: usize,
    frame_timer: Timer,
    animation_timer: Option<Timer>,
}

impl Config {
    pub fn new(
        first_index: usize,
        sprite_count: usize,
        fps: u8,
        repeat: bool,
    ) -> Self {
        Self {
            first_index,
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
        self.relative_current_index % self.sprite_count + self.first_index
    }
}
