use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (animation_transition, handle_animation)
            .chain()
            .in_set(UpdateSets::Draw),
    );
}

#[derive(Component, Default)]
pub struct AnimateOnce;

// TODO make generic and apply this to enemiess as well.
/// System resets animation comfigurations.
/// Runs if a [`PlayerStateTransition`] component is bundled with the entity.
fn animation_transition(
    mut player: Query<(
        Entity,
        &mut AnimationConfig,
        &mut PlayerState,
        &PlayerStateTransition,
    )>,
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
                *config = AnimationConfig::new(12, 3, 8, false);
                commands.entity(player).insert(AnimateOnce);
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
