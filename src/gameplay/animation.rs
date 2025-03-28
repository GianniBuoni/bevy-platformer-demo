use crate::prelude::*;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            validate_animation_state,
            animation_transition,
            handle_animation,
        )
            .chain()
            .in_set(UpdateSets::Draw),
    );
}

#[derive(Component, Default)]
pub struct AnimationConfig {
    pub first_sprite: usize,
    pub sprite_count: usize,
    pub frame_timer: Timer,
    pub animation_timer: Option<Timer>,
}

impl AnimationConfig {
    fn new(
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

impl From<&EntityInstance> for AnimationConfig {
    fn from(value: &EntityInstance) -> Self {
        match value.identifier.as_str() {
            "Player" => AnimationConfig::new(0, 4, 6, true),
            _ => AnimationConfig::new(1, 2, 6, true),
        }
    }
}

/// System validates states that are determined in the [`Update`]
/// If ther current player state is no longer valid for animation,
/// i.e. timer has finished, state is changed.
fn validate_animation_state(
    mut player: Query<
        (Entity, &mut AnimationConfig, &mut PlayerState),
        With<Player>,
    >,
    time: Res<Time>,
    mut commands: Commands,
) {
    let (player, mut config, state) = get_single_mut!(player);
    let mut should_transition = false;
    if let Some(timer) = &config.animation_timer {
        let mut timer = timer.clone();
        timer.tick(time.delta());
        should_transition = timer.finished();
        config.animation_timer = Some(timer);
    }

    // Handle a state change
    if should_transition {
        info!("State: {:?}, unvalidated, switching to:", state);
        commands
            .entity(player)
            .insert(PlayerStateTransition::new(PlayerState::Fall));
    }
}

/// System resets animation comfigurations.
/// Runs if a [`PlayerStateTransition`] component is bundled with the entity.
fn animation_transition(
    mut player: Query<
        (
            Entity,
            &mut AnimationConfig,
            &mut PlayerState,
            &PlayerStateTransition,
        ),
        With<Player>,
    >,
    mut commands: Commands,
) {
    {
        let (player, mut config, mut state, transition) =
            get_single_mut!(player);
        match transition.0 {
            PlayerState::Idle => {
                *config = AnimationConfig::new(0, 5, 8, true);
            }
            PlayerState::Run => {
                *config = AnimationConfig::new(6, 6, 8, true);
            }
            PlayerState::Jump => {
                info!("jump config set");
                *config = AnimationConfig::new(12, 3, 8, false);
            }
            PlayerState::Fall => {
                *config = AnimationConfig::new(18, 1, 8, true);
            }
        }
        *state = transition.0.clone();
        commands.entity(player).remove::<PlayerStateTransition>();
    }
}

/// After all the animation checks are done, atlas indexes are updated.
fn handle_animation(
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
