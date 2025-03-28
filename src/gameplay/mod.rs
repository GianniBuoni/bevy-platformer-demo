use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::animation::AnimateOnce;
    pub(crate) use super::animation_config::AnimationConfig;
}

mod animation;
mod animation_config;
mod level;
mod walls;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((animation::plugin, level::plugin, walls::plugin));
}
