use crate::prelude::*;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (animation_state, animate, animate_once)
            .chain()
            .in_set(UpdateSets::Draw),
    );
}

#[derive(Component)]
pub(super) struct AnimateOnce;

#[derive(Component, Default)]
pub struct AnimationConfig {
    pub first_sprite: usize,
    pub sprite_count: usize,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first_sprite: usize, last_sprite: usize, fps: u8) -> Self {
        Self {
            first_sprite,
            sprite_count: last_sprite,
            frame_timer: Self::timer_from_fps(fps, TimerMode::Repeating),
        }
    }
    fn timer_from_fps(fps: u8, mode: TimerMode) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / fps as f32), mode)
    }
}

impl From<&EntityInstance> for AnimationConfig {
    fn from(value: &EntityInstance) -> Self {
        match value.identifier.as_str() {
            "Player" => AnimationConfig::new(0, 4, 6),
            _ => AnimationConfig::new(1, 2, 6),
        }
    }
}

fn animation_state(
    mut player: Query<
        (
            Entity,
            &mut AnimationConfig,
            &PlayerState,
            &PlayerStateTransition,
        ),
        With<Player>,
    >,
    mut commands: Commands,
) {
    {
        let (player, mut config, state, _transition) = get_single_mut!(player);
        match state {
            PlayerState::Idle => {
                *config = AnimationConfig::new(0, 5, 8);
            }
            PlayerState::Run => {
                *config = AnimationConfig::new(6, 6, 8);
            }
            PlayerState::Jump => {
                *config = AnimationConfig::new(12, 3, 8);
                commands.entity(player).insert(AnimateOnce);
            }
            PlayerState::Fall => {
                *config = AnimationConfig::new(18, 1, 8);
            }
        }
        commands.entity(player).remove::<PlayerStateTransition>();
    }
}

fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &PlayerInput, &mut Sprite)>,
) {
    for (mut config, input, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            match input.x {
                -1. => sprite.flip_x = true,
                1. => sprite.flip_x = false,
                _ => (),
            }

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % config.sprite_count
                    + config.first_sprite;
            }
        }
    }
}

fn animate_once(
    player: Query<
        (Entity, &AnimationConfig, &AnimateOnce, &Sprite),
        With<Player>,
    >,
    mut commands: Commands,
) {
    let (player, config, _, sprite) = get_single!(player);
    let Some(atlas) = &sprite.texture_atlas else {
        warn!("Sprite w/o texture atlas: {}", player);
        return;
    };
    // TODO look at this in an inspector
    if atlas.index >= config.first_sprite + config.sprite_count - 1 {
        commands.entity(player).remove::<AnimateOnce>();
    }
}
