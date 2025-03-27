use std::time::Duration;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, animate.in_set(UpdateSets::Update));
}

#[derive(Component, Default)]
pub(super) struct AnimationConfig {
    first_sprite: usize,
    last_sprite: usize,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(
        first_sprite: usize,
        last_sprite: usize,
        fps: u8,
        mode: TimerMode,
    ) -> Self {
        Self {
            first_sprite,
            last_sprite,
            frame_timer: Self::timer_from_fps(fps, mode),
        }
    }
    fn timer_from_fps(fps: u8, mode: TimerMode) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / fps as f32), mode)
    }
}

impl From<&EntityInstance> for AnimationConfig {
    fn from(value: &EntityInstance) -> Self {
        match value.identifier.as_str() {
            "Player" => AnimationConfig::new(0, 4, 6, TimerMode::Repeating),
            _ => AnimationConfig::new(1, 2, 6, TimerMode::Once),
        }
    }
}

fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite {
                    atlas.index = config.first_sprite;
                } else {
                    atlas.index += 1
                }
            }
        }
    }
}
