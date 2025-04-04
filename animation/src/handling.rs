use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            animation_transition,
            handle_sprite_direction,
            handle_animation,
        )
            .chain()
            .in_set(UpdateSets::Draw),
    );
}

#[derive(Component, Default)]
pub struct AnimateOnce;

// TODO make generic and apply this to enemiess as well.
/// System resets animation comfigurations.
/// Runs if a [`TransitionStates`] component is bundled with the entity.
fn animation_transition(
    mut player: Query<(
        Entity,
        &mut AnimationConfig,
        &mut AnimationState,
        &TransitionStates,
    )>,
    mut commands: Commands,
) {
    {
        let (player, mut config, mut state, transition) =
            get_single_mut!(player);
        match transition.0 {
            AnimationState::Idle => {
                *config = AnimationConfig::new(0, 5, 8, true);
            }
            AnimationState::Run => {
                *config = AnimationConfig::new(6, 6, 8, true);
            }
            AnimationState::Jump => {
                *config = AnimationConfig::new(12, 3, 8, false);
                commands.entity(player).insert(AnimateOnce);
            }
            AnimationState::Fall => {
                *config = AnimationConfig::new(18, 1, 8, true);
            }
        }
        *state = transition.0.clone();
        commands.entity(player).remove::<TransitionStates>();
    }
}

fn handle_sprite_direction(
    mut sprite: Query<(&HorizontalDirection, &mut Sprite)>,
) {
    sprite.iter_mut().for_each(|(dir, mut sprite)| match dir.0 {
        -1. => sprite.flip_x = true,
        1. => sprite.flip_x = false,
        _ => (),
    });
}

/// After all the animation checks are done, atlas indexes are updated.
fn handle_animation(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
) {
    query.iter_mut().for_each(|(mut config, mut sprite)| {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = config.get_new_index();
            }
        }
    });
}
