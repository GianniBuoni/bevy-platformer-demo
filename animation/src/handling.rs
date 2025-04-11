use crate::prelude::*;

pub(super) fn plugin(schedule: impl SystemSet + Clone) -> impl Fn(&mut App) {
    move |app| {
        app.add_systems(
            Update,
            (
                handle_animation_transiton,
                handle_sprite_direction,
                handle_animation,
            )
                .in_set(schedule.clone())
                .chain(),
        );
    }
}

fn handle_animation_transiton(
    mut query: Query<(Entity, &mut Config, &Transition)>,
    mut command: Commands,
) {
    query
        .iter_mut()
        .for_each(|(entity, mut config, transition)| {
            *config = transition.into();
            command.entity(entity).remove::<Transition>();
        });
}

fn handle_sprite_direction(mut sprite: Query<(&ConfigDirection, &mut Sprite)>) {
    sprite.iter_mut().for_each(|(dir, mut sprite)| match dir.x {
        -1. => sprite.flip_x = true,
        1. => sprite.flip_x = false,
        _ => (),
    });
}

/// After all the animation checks are done, atlas indexes are updated.
fn handle_animation(
    time: Res<Time>,
    mut query: Query<(&mut Config, &mut Sprite)>,
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
