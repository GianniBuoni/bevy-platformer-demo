use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::animation::AnimationConfig;
    pub(crate) use super::level_entitiy_movement::OrthoMovement;
    pub(crate) use super::platforms::Platform;
}

mod animation;
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
