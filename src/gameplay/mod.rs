use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::animation::AnimateOnce;
    pub(crate) use super::animation_config::AnimationConfig;
    pub(crate) use super::level_entitiy_movement::OrthoMovementBundle;
}

mod animation;
mod animation_config;
mod level;
mod level_entities;
mod level_entitiy_movement;
mod platforms;
mod walls;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        level_entities::plugin,
        platforms::plugin,
        walls::plugin,
    ));
}
